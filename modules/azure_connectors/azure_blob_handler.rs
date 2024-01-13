use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
};

use azure_storage::StorageCredentials;

use azure_core::Error;
use azure_storage_blobs::prelude::*;

use colored::Colorize;
use log::info;

pub struct AzureBlobHandler {
    client: BlobClient,
}

impl AzureBlobHandler {
    pub fn new(account: &str, access_key: &str, container_name: &str, blob_name: &str) -> Self {
        let storage_credentials =
            StorageCredentials::access_key(account.to_string(), access_key.to_string());
        let client = ClientBuilder::new(account.to_string(), storage_credentials)
            .blob_client(container_name, blob_name.to_string());

        AzureBlobHandler { client }
    }

    pub async fn upload_blob(&self, file_path: &str) -> Result<(), Error> {
        let f = File::open(file_path)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;

        self.client.put_block_blob(buffer).await?;

        let colored_string =
            format!("Successfully uploaded {} blob", self.client.blob_name()).blue();
        info!("{}", colored_string);

        Ok(())
    }

    pub async fn download_blob(&self, file_path: &str) -> Result<(), Error> {
        let data = self.client.get_content().await?;

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)?;

        file.write_all(&data)?;

        let colored_string =
            format!("Successfully downloaded {} blob", self.client.blob_name()).blue();
        info!("{}", colored_string);

        Ok(())
    }

    pub async fn delete_blob(&self) -> Result<(), Error> {
        self.client.delete().await?;

        let colored_string =
            format!("Successfully deleted {} blob", self.client.blob_name()).blue();
        info!("{}", colored_string);

        Ok(())
    }
}
