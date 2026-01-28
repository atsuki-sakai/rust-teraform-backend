# =============================================================================
# Outputs
# =============================================================================

output "cloud_run_url" {
  description = "The URL of the Cloud Run service"
  value       = google_cloud_run_v2_service.api.uri
}

output "cloud_sql_connection_name" {
  description = "The connection name for Cloud SQL"
  value       = google_sql_database_instance.postgres.connection_name
}

output "service_account_email" {
  description = "The email of the Cloud Run service account"
  value       = google_service_account.cloud_run_sa.email
}
