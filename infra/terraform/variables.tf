variable "project_id" {
  type        = string
  description = "GCP project ID"
}

variable "region" {
  type        = string
  default     = "us-central1"
  description = "GCP region"
}

variable "env" {
  type        = string
  description = "Environment label: dev | prod"
}

variable "github_owner" {
  type    = string
  default = "sargonxg"
}

variable "github_repo" {
  type    = string
  default = "AGON"
}

variable "db_tier" {
  type        = string
  default     = "db-f1-micro"
  description = "Cloud SQL tier"
}

variable "db_ha" {
  type        = bool
  default     = false
  description = "Enable Cloud SQL regional HA"
}

variable "image_tag" {
  type        = string
  default     = "latest"
  description = "Cloud Run container image tag"
}

variable "budget_amount_usd" {
  type    = number
  default = 50
}
