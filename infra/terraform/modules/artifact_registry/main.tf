variable "project_id" { type = string }
variable "region"     { type = string }
variable "env"        { type = string }

resource "google_artifact_registry_repository" "agon" {
  project       = var.project_id
  location      = var.region
  repository_id = "agon"
  format        = "DOCKER"
  description   = "AGON container images"
}

output "repository_name" { value = google_artifact_registry_repository.agon.repository_id }
output "repository_id"   { value = google_artifact_registry_repository.agon.id }
