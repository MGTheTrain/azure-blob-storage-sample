locals {
  tags = {
    env         = "${var.environment}",
    team        = "${var.team}",
    owner       = "MGTheTrain",
    project     = "rust-azure-blob-storage-handler",
    app-purpose = "Deployment of temporary integration test environment",
    Stage       = "${var.environment}"
  }
  storage_account_names                              = [for i in range(var.var.number_of_storage_accounts) : format("%s%ssa%03d", var.digital_product_affix, var.environment, i + 1)]
}