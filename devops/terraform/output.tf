# Azure Storage Account service
output "storage_account_name_list" {
    value = [for storage_account in azurerm_storage_account.this : storage_account.name]
}

output "storage_account_primary_connection_string_list" {
    value = [for storage_account in azurerm_storage_account.this : storage_account.primary_connection_string]
    sensitive = true
}

output "storage_account_primary_access_key_list" {
    value = [for storage_account in azurerm_storage_account.this : storage_account.primary_access_key]
    sensitive = true
}

output "storage_account_container_name_list" {
    value = [for storage_account_container in azurerm_storage_container.this : storage_account_container.name]
}