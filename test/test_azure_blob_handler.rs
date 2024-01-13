#[cfg(test)]
mod tests {
    use common_modules::azure_connectors::azure_blob_handler::AzureBlobHandler;
    use common_modules::set_env_var;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    pub async fn test_azure_blob_storage_methods() -> Result<(), Box<dyn std::error::Error>> {
        env_logger::init();

        let env_file_path = "secrets.cfg";
        dotenv::from_path(env_file_path).ok();

        let mut account = String::from("devstoreaccount1");
        let mut access_key = String::from("Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==");

        if let Some(value) = set_env_var("AZURE_ACCOUNT_NAME") {
            account = value;
        }
        if let Some(value) = set_env_var("AZURE_ACCESS_KEY") {
            access_key = value;
        }

        let container_name = std::env::var("AZURE_CONTAINER_NAME")
            .expect("AZURE_CONTAINER_NAME environment variable expected");
        let blob_name = "sample.txt";
        let upload_file_path = "assets/sample.txt";
        let download_file_path = "output/copy-sample.txt";

        let handler = AzureBlobHandler::new(&account, &access_key, &container_name, &blob_name);
        handler.upload_blob(upload_file_path).await?;
        handler.download_blob(download_file_path).await?;
        handler.delete_blob().await?;

        Ok(())
    }
}
