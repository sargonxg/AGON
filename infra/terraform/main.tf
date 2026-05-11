terraform {
  required_version = ">= 1.6"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 6.0"
    }
    google-beta = {
      source  = "hashicorp/google-beta"
      version = "~> 6.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.6"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

provider "google-beta" {
  project = var.project_id
  region  = var.region
}

data "google_project" "this" {
  project_id = var.project_id
}

locals {
  env_label = var.env
  common_labels = {
    project = "agon"
    env     = var.env
    managed = "terraform"
  }
}

module "network" {
  source     = "./modules/network"
  project_id = var.project_id
  region     = var.region
  env        = var.env
}

module "cloud_sql" {
  source                  = "./modules/cloud_sql"
  project_id              = var.project_id
  region                  = var.region
  env                     = var.env
  network_id              = module.network.network_id
  private_vpc_connection  = module.network.private_vpc_connection
  db_tier                 = var.db_tier
  db_ha                   = var.db_ha
  db_password_secret_id   = module.secrets.db_password_secret_id
}

module "storage" {
  source     = "./modules/storage"
  project_id = var.project_id
  region     = var.region
  env        = var.env
  labels     = local.common_labels
}

module "artifact_registry" {
  source     = "./modules/artifact_registry"
  project_id = var.project_id
  region     = var.region
  env        = var.env
}

module "secrets" {
  source     = "./modules/secrets"
  project_id = var.project_id
  env        = var.env
}

module "iam" {
  source       = "./modules/iam"
  project_id   = var.project_id
  env          = var.env
  docs_bucket  = module.storage.docs_bucket
  exports_bkt  = module.storage.exports_bucket
  ar_repo_name = module.artifact_registry.repository_name
  db_pw_secret = module.secrets.db_password_secret_id
}

module "cloud_run" {
  source                = "./modules/cloud_run"
  project_id            = var.project_id
  region                = var.region
  env                   = var.env
  service_account_email = module.iam.cloud_run_sa_email
  vpc_id                = module.network.network_id
  subnet_id             = module.network.subnet_id
  db_private_ip         = module.cloud_sql.db_private_ip
  db_user               = module.cloud_sql.db_user
  db_name               = module.cloud_sql.db_name
  db_password_secret_id = module.secrets.db_password_secret_id
  docs_bucket           = module.storage.docs_bucket
  image_tag             = var.image_tag
}

module "eventarc" {
  source            = "./modules/eventarc"
  project_id        = var.project_id
  region            = var.region
  env               = var.env
  docs_bucket       = module.storage.docs_bucket
  cloud_run_service = module.cloud_run.service_name
  eventarc_sa_email = module.iam.eventarc_sa_email
  depends_on        = [module.cloud_run]
}
