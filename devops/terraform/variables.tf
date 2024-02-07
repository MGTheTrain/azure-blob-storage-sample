# Azure Rg
variable "digital_product_affix" {
  default     = "rabsh"
  description = "The digital product affix."
  type        = string
}

variable "environment" {
  default     = "sbx"
  description = "The environment."
  type        = string

  validation {
    condition     = can(regex("^(sbx|dev|qas|prd)$", var.environment))
    error_message = "Invalid input, options: \"sbx\", \"dev\", \"qas\", \"prd\"."
  }
}

variable "resource_instance_number" {
  default     = "001"
  description = "The resource instance number."
  type        = string

  validation {
    condition     = length(var.resource_instance_number) == 3
    error_message = "Must be a 3 character long resource_instance_number, e.g. 001."
  }

  validation {
    condition     = can(regex("^[0-9.]*$", var.resource_instance_number))
    error_message = "The 'resource_instance_number' value must be a valid and can only contain number characters from 0 to 9."
  }
}

variable "location" {
  default     = "West Europe"
  description = "The geographic location in which to deploy."
  type        = string
}

variable "team" {
  default     = "MG Innovators"
  description = "The team used for tagging resource groups and resources."
  type        = string
}

# Azure Storage Account Service
variable "storage_account_tier" {
  default     = "Standard"
  description = "The Azure Storage Account Tier."
  type        = string
}

variable "number_of_storage_accounts" {
  default     = 1
  description = "The total number of Storage Account Services to deploy."
  type        = number
}

variable "storage_account_replication_type" {
  default     = "LRS"
  description = <<EOF
   The Azure Storage Account Replication Type. 
   See: https://docs.microsoft.com/en-us/azure/storage/common/storage-redundancy 
  EOF
  type        = string
}

variable "storage_account_kind" {
  default     = "StorageV2"
  description = "The Azure Storage Account Kind."
  type        = string
}

# Azure Storage Account Container
variable "storage_account_container_access_type" {
  default     = "private"
  description = "The Storage Account Container Instance Type."
  type        = string
}