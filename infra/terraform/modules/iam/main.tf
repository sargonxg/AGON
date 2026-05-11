variable "project_id"   { type = string }
variable "env"          { type = string }
variable "docs_bucket"  { type = string }
variable "exports_bkt"  { type = string }
variable "ar_repo_name" { type = string }
variable "db_pw_secret" { type = string }

resource "google_service_account" "cloud_run" {
  project      = var.project_id
  account_id   = "agon-cloud-run-${var.env}"
  display_name = "AGON Cloud Run runtime"
}

resource "google_service_account" "eventarc" {
  project      = var.project_id
  account_id   = "agon-eventarc-${var.env}"
  display_name = "AGON Eventarc"
}

# Cloud Run SA — least privilege
resource "google_project_iam_member" "run_sql" {
  project = var.project_id
  role    = "roles/cloudsql.client"
  member  = "serviceAccount:${google_service_account.cloud_run.email}"
}

resource "google_project_iam_member" "run_aip" {
  project = var.project_id
  role    = "roles/aiplatform.user"
  member  = "serviceAccount:${google_service_account.cloud_run.email}"
}

resource "google_project_iam_member" "run_logs" {
  project = var.project_id
  role    = "roles/logging.logWriter"
  member  = "serviceAccount:${google_service_account.cloud_run.email}"
}

resource "google_project_iam_member" "run_metrics" {
  project = var.project_id
  role    = "roles/monitoring.metricWriter"
  member  = "serviceAccount:${google_service_account.cloud_run.email}"
}

resource "google_project_iam_member" "run_trace" {
  project = var.project_id
  role    = "roles/cloudtrace.agent"
  member  = "serviceAccount:${google_service_account.cloud_run.email}"
}

resource "google_storage_bucket_iam_member" "run_docs" {
  bucket = var.docs_bucket
  role   = "roles/storage.objectAdmin"
  member = "serviceAccount:${google_service_account.cloud_run.email}"
}

resource "google_storage_bucket_iam_member" "run_exports" {
  bucket = var.exports_bkt
  role   = "roles/storage.objectAdmin"
  member = "serviceAccount:${google_service_account.cloud_run.email}"
}

resource "google_secret_manager_secret_iam_member" "run_db_pw" {
  project   = var.project_id
  secret_id = var.db_pw_secret
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.cloud_run.email}"
}

# Eventarc SA — invoke Cloud Run
resource "google_project_iam_member" "eventarc_receiver" {
  project = var.project_id
  role    = "roles/eventarc.eventReceiver"
  member  = "serviceAccount:${google_service_account.eventarc.email}"
}

resource "google_project_iam_member" "eventarc_invoker" {
  project = var.project_id
  role    = "roles/run.invoker"
  member  = "serviceAccount:${google_service_account.eventarc.email}"
}

output "cloud_run_sa_email" { value = google_service_account.cloud_run.email }
output "eventarc_sa_email"  { value = google_service_account.eventarc.email }
