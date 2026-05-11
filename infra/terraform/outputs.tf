output "service_url" {
  description = "Cloud Run service URL"
  value       = module.cloud_run.service_url
}

output "service_name" {
  value = module.cloud_run.service_name
}

output "db_connection_name" {
  value = module.cloud_sql.connection_name
}

output "db_private_ip" {
  value     = module.cloud_sql.db_private_ip
  sensitive = true
}

output "docs_bucket_name" {
  value = module.storage.docs_bucket
}

output "cloud_run_sa_email" {
  value = module.iam.cloud_run_sa_email
}

output "artifact_registry" {
  value = "${var.region}-docker.pkg.dev/${var.project_id}/${module.artifact_registry.repository_name}"
}
