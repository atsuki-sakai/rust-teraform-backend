# =============================================================================
# Cloud SQL PostgreSQL Instance
# =============================================================================

resource "google_sql_database_instance" "postgres" {
  name             = "todo-postgres-${var.environment}"
  database_version = "POSTGRES_15"
  region           = var.region

  settings {
    tier = var.db_tier

    ip_configuration {
      ipv4_enabled    = false
      private_network = "projects/${var.project_id}/global/networks/default"
    }

    backup_configuration {
      enabled = var.environment == "prod"
    }

    disk_size = 10
    disk_type = "PD_SSD"
  }

  deletion_protection = var.environment == "prod"

  depends_on = [
    google_project_service.sqladmin_api,
    google_project_service.servicenetworking_api
  ]
}

# =============================================================================
# Database and User
# =============================================================================

resource "google_sql_database" "database" {
  name     = var.db_name
  instance = google_sql_database_instance.postgres.name
}

resource "google_sql_user" "user" {
  name     = var.db_user
  instance = google_sql_database_instance.postgres.name
  password = random_password.db_password.result
}

resource "random_password" "db_password" {
  length  = 32
  special = false
}

# =============================================================================
# Private Service Connection
# =============================================================================

resource "google_compute_global_address" "private_ip_range" {
  name          = "todo-private-ip-range"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 16
  network       = "projects/${var.project_id}/global/networks/default"
}

resource "google_service_networking_connection" "private_vpc_connection" {
  network                 = "projects/${var.project_id}/global/networks/default"
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.private_ip_range.name]
}
