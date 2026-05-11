variable "project_id"            { type = string }
variable "region"                { type = string }
variable "env"                   { type = string }
variable "service_account_email" { type = string }
variable "vpc_id"                { type = string }
variable "subnet_id"             { type = string }
variable "db_private_ip"         { type = string }
variable "db_user"               { type = string }
variable "db_name"               { type = string }
variable "db_password_secret_id" { type = string }
variable "docs_bucket"           { type = string }
variable "image_tag" {
  type    = string
  default = "latest"
}

locals {
  # First-deploy placeholder image. CI/CD replaces with Artifact Registry image.
  placeholder_image = "gcr.io/cloudrun/hello"
}

resource "google_cloud_run_v2_service" "agon" {
  name     = "agon-${var.env}"
  location = var.region
  project  = var.project_id

  deletion_protection = false

  template {
    service_account = var.service_account_email

    containers {
      image = local.placeholder_image

      ports {
        container_port = 8080
      }

      env {
        name  = "AGON_ENV"
        value = var.env
      }
      env {
        name  = "AGON_GCP_PROJECT_ID"
        value = var.project_id
      }
      env {
        name  = "AGON_GCP_REGION"
        value = var.region
      }
      env {
        name  = "AGON_DB_HOST"
        value = var.db_private_ip
      }
      env {
        name  = "AGON_DB_USER"
        value = var.db_user
      }
      env {
        name  = "AGON_DB_NAME"
        value = var.db_name
      }
      env {
        name  = "AGON_GCS_DOC_BUCKET"
        value = var.docs_bucket
      }
      env {
        name  = "AGON_VERTEX_MODEL_FLASH"
        value = "gemini-2.5-flash"
      }
      env {
        name  = "AGON_VERTEX_MODEL_PRO"
        value = "gemini-2.5-pro"
      }

      env {
        name = "AGON_DB_PASSWORD"
        value_source {
          secret_key_ref {
            secret  = var.db_password_secret_id
            version = "latest"
          }
        }
      }

      resources {
        limits = {
          cpu    = "1"
          memory = "1Gi"
        }
        startup_cpu_boost = true
      }
    }

    scaling {
      min_instance_count = 0
      max_instance_count = 10
    }

    vpc_access {
      network_interfaces {
        network    = var.vpc_id
        subnetwork = var.subnet_id
      }
      egress = "PRIVATE_RANGES_ONLY"
    }

    timeout = "3600s"
  }

  traffic {
    type    = "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
    percent = 100
  }

  # Ignore image so Cloud Build can update without TF clobbering it
  lifecycle {
    ignore_changes = [template[0].containers[0].image]
  }
}

# Allow unauthenticated for demo / dev
resource "google_cloud_run_v2_service_iam_member" "public_invoker" {
  count    = var.env == "dev" ? 1 : 0
  project  = var.project_id
  location = var.region
  name     = google_cloud_run_v2_service.agon.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}

output "service_name" { value = google_cloud_run_v2_service.agon.name }
output "service_url"  { value = google_cloud_run_v2_service.agon.uri }
