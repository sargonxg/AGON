variable "project_id" { type = string }
variable "env"        { type = string }

resource "google_secret_manager_secret" "db_password" {
  project   = var.project_id
  secret_id = "agon-db-password-${var.env}"
  replication {
    auto {}
  }
}

resource "google_secret_manager_secret" "jwt_key" {
  project   = var.project_id
  secret_id = "agon-jwt-signing-key-${var.env}"
  replication {
    auto {}
  }
}

resource "random_password" "jwt" {
  length  = 64
  special = false
}

resource "google_secret_manager_secret_version" "jwt" {
  secret      = google_secret_manager_secret.jwt_key.id
  secret_data = random_password.jwt.result
}

output "db_password_secret_id" { value = google_secret_manager_secret.db_password.id }
output "jwt_secret_id"         { value = google_secret_manager_secret.jwt_key.id }
