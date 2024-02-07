resource "azurerm_resource_group" "this" {
  name     = "${var.digital_product_affix}-${var.environment}-rg${var.resource_instance_number}"
  location = var.location

  tags = local.tags
}

resource "azurerm_storage_account" "this" {
  name                     = local.storage_account_names[count.index]
  resource_group_name      = azurerm_resource_group.this.name
  location                 = var.location
  count                    = var.number_of_storage_accounts
  account_tier             = var.storage_account_tier
  account_replication_type = var.storage_account_replication_type

  tags = local.tags
}

resource "azurerm_storage_container" "this" {
  name                  = "${var.digital_product_affix}${var.environment}sac${var.resource_instance_number}"
  count                 = var.number_of_storage_accounts
  storage_account_name  = azurerm_storage_account.this[count.index].name
}