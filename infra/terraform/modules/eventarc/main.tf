variable "project_id"        { type = string }
variable "region"            { type = string }
variable "env"               { type = string }
variable "docs_bucket"       { type = string }
variable "cloud_run_service" { type = string }
variable "eventarc_sa_email" { type = string }

# Grant Cloud Storage SA permission to publish to Pub/Sub (required for GCS Eventarc triggers)
data "google_storage_project_service_account" "gcs" {
  project = var.project_id
}

data "google_project" "this" {
  project_id = var.project_id
}

resource "google_project_iam_member" "gcs_pubsub" {
  project = var.project_id
  role    = "roles/pubsub.publisher"
  member  = "serviceAccount:${data.google_storage_project_service_account.gcs.email_address}"
}

# Eventarc service agent needs storage.objectViewer on the source bucket to validate
resource "google_storage_bucket_iam_member" "eventarc_agent_storage" {
  bucket = var.docs_bucket
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:service-${data.google_project.this.number}@gcp-sa-eventarc.iam.gserviceaccount.com"
}

# Provided trigger SA also needs to read bucket metadata
resource "google_storage_bucket_iam_member" "eventarc_sa_storage" {
  bucket = var.docs_bucket
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${var.eventarc_sa_email}"
}

resource "google_eventarc_trigger" "doc_uploaded" {
  name     = "agon-doc-uploaded-${var.env}"
  project  = var.project_id
  location = var.region

  matching_criteria {
    attribute = "type"
    value     = "google.cloud.storage.object.v1.finalized"
  }
  matching_criteria {
    attribute = "bucket"
    value     = var.docs_bucket
  }

  destination {
    cloud_run_service {
      service = var.cloud_run_service
      region  = var.region
      path    = "/api/eventarc/upload"
    }
  }

  service_account = var.eventarc_sa_email

  depends_on = [google_project_iam_member.gcs_pubsub]
}

output "trigger_name" { value = google_eventarc_trigger.doc_uploaded.name }
