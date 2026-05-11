variable "project_id" { type = string }
variable "region"     { type = string }
variable "env"        { type = string }

resource "google_compute_network" "vpc" {
  name                    = "agon-${var.env}-vpc"
  project                 = var.project_id
  auto_create_subnetworks = false
}

resource "google_compute_subnetwork" "subnet" {
  name          = "agon-${var.env}-subnet"
  project       = var.project_id
  region        = var.region
  network       = google_compute_network.vpc.id
  ip_cidr_range = "10.10.0.0/20"
  private_ip_google_access = true
}

resource "google_compute_global_address" "private_ip_alloc" {
  name          = "agon-${var.env}-psa-range"
  project       = var.project_id
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 16
  network       = google_compute_network.vpc.id
}

resource "google_service_networking_connection" "private_vpc_connection" {
  network                 = google_compute_network.vpc.id
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.private_ip_alloc.name]
}

output "network_id"             { value = google_compute_network.vpc.id }
output "network_name"           { value = google_compute_network.vpc.name }
output "subnet_id"              { value = google_compute_subnetwork.subnet.id }
output "private_vpc_connection" { value = google_service_networking_connection.private_vpc_connection.id }
