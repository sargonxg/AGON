variable "project_id"             { type = string }
variable "region"                 { type = string }
variable "env"                    { type = string }
variable "network_id"             { type = string }
variable "private_vpc_connection" { type = string }
variable "db_tier"                { type = string }
variable "db_ha"                  { type = bool }
variable "db_password_secret_id"  { type = string }

resource "random_password" "db" {
  length  = 32
  special = false
}

resource "google_secret_manager_secret_version" "db_password" {
  secret      = var.db_password_secret_id
  secret_data = random_password.db.result
}

resource "google_sql_database_instance" "agon" {
  name             = "agon-${var.env}"
  project          = var.project_id
  region           = var.region
  database_version = "POSTGRES_16"
  deletion_protection = false

  depends_on = [var.private_vpc_connection]

  settings {
    tier              = var.db_tier
    edition           = "ENTERPRISE"
    availability_type = var.db_ha ? "REGIONAL" : "ZONAL"
    disk_size         = 10
    disk_type         = "PD_SSD"
    disk_autoresize   = true

    ip_configuration {
      ipv4_enabled    = false
      private_network = var.network_id
    }

    backup_configuration {
      enabled    = true
      start_time = "03:00"
      point_in_time_recovery_enabled = true
    }

    insights_config {
      query_insights_enabled = true
    }
  }
}

resource "google_sql_database" "agon" {
  name     = "agon"
  project  = var.project_id
  instance = google_sql_database_instance.agon.name
}

resource "google_sql_user" "agon" {
  name     = "agon"
  project  = var.project_id
  instance = google_sql_database_instance.agon.name
  password = random_password.db.result
}

output "connection_name" { value = google_sql_database_instance.agon.connection_name }
output "db_private_ip"   { value = google_sql_database_instance.agon.private_ip_address }
output "db_user"         { value = google_sql_user.agon.name }
output "db_name"         { value = google_sql_database.agon.name }
