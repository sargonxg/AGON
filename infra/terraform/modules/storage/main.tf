variable "project_id" { type = string }
variable "region"     { type = string }
variable "env"        { type = string }
variable "labels" {
  type    = map(string)
  default = {}
}

resource "google_storage_bucket" "docs" {
  name                        = "${var.project_id}-agon-documents-${var.env}"
  project                     = var.project_id
  location                    = var.region
  uniform_bucket_level_access = true
  force_destroy               = true
  labels                      = var.labels

  lifecycle_rule {
    condition { age = 30 }
    action {
      type          = "SetStorageClass"
      storage_class = "NEARLINE"
    }
  }
  lifecycle_rule {
    condition { age = 90 }
    action {
      type          = "SetStorageClass"
      storage_class = "COLDLINE"
    }
  }
  lifecycle_rule {
    condition {
      age                = 1
      matches_prefix     = ["_tmp/"]
    }
    action { type = "Delete" }
  }
}

resource "google_storage_bucket" "exports" {
  name                        = "${var.project_id}-agon-exports-${var.env}"
  project                     = var.project_id
  location                    = var.region
  uniform_bucket_level_access = true
  force_destroy               = true
  labels                      = var.labels
}

output "docs_bucket"    { value = google_storage_bucket.docs.name }
output "exports_bucket" { value = google_storage_bucket.exports.name }
