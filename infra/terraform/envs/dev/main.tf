module "agon" {
  source     = "../.."
  project_id = var.project_id
  region     = var.region
  env        = var.env
  db_tier    = var.db_tier
  db_ha      = var.db_ha
}

variable "project_id" { type = string }
variable "region"     { type = string }
variable "env"        { type = string }
variable "db_tier"    { type = string }
variable "db_ha"      { type = bool }

output "service_url" { value = module.agon.service_url }
output "db_private_ip" {
  value     = module.agon.db_private_ip
  sensitive = true
}
output "docs_bucket_name"  { value = module.agon.docs_bucket_name }
output "artifact_registry" { value = module.agon.artifact_registry }
output "cloud_run_sa_email"{ value = module.agon.cloud_run_sa_email }
