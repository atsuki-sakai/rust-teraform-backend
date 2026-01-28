# =============================================================================
# Project Configuration
# =============================================================================

variable "project_id" {
  description = "The GCP project ID"
  type        = string
}

variable "region" {
  description = "The GCP region"
  type        = string
  default     = "asia-northeast1"
}

variable "environment" {
  description = "Environment name (dev, staging, prod)"
  type        = string
  default     = "dev"
}

# =============================================================================
# Cloud SQL Configuration
# =============================================================================

variable "db_tier" {
  description = "Cloud SQL instance tier"
  type        = string
  default     = "db-f1-micro"
}

variable "db_name" {
  description = "Database name"
  type        = string
  default     = "todo_db"
}

variable "db_user" {
  description = "Database user"
  type        = string
  default     = "todo_user"
}

# =============================================================================
# Cloud Run Configuration
# =============================================================================

variable "cloud_run_memory" {
  description = "Cloud Run memory limit"
  type        = string
  default     = "256Mi"
}

variable "cloud_run_cpu" {
  description = "Cloud Run CPU limit"
  type        = string
  default     = "1"
}

variable "cloud_run_min_instances" {
  description = "Minimum number of Cloud Run instances"
  type        = number
  default     = 0
}

variable "cloud_run_max_instances" {
  description = "Maximum number of Cloud Run instances"
  type        = number
  default     = 2
}

variable "container_image" {
  description = "Container image URL"
  type        = string
}
