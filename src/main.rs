use std::{env, fs::{self, File}, io::{BufReader, Read, Write}};

use azure_core::Error;
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;

fn set_env_var(env_var_name: &str) -> Option<String> {
    match env::var(env_var_name) {
        Ok(value) => {
            // Print the value if needed
            // println!("Value of {} is: {}", env_var_name, value);
            Some(value)
        }
        Err(_) => {
            println!("{} is not set.", env_var_name);
            None
        }
    }
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let mut account = String::from("devstoreaccount1"); //resolves to devstoreaccount1.blob.core.windows.net
    let mut access_key = String::from("Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==");

    if let Some(value) = set_env_var("AZURE_ACCOUNT_NAME") {
        account = value;
    } 
    if let Some(value) = set_env_var("AZURE_ACCESS_KEY") {
        access_key = value;
    } 

    let container_name = "rust-upload-test"; // manually create Azure Storage Account service container in Azure portal for now
    let blob_name = "sample.txt";
    let upload_file_path = "sample.txt";
    let download_file_path = "temp/copy-sample.txt";

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let client = ClientBuilder::new(account, storage_credentials).blob_client(container_name, blob_name);

    // NOTE: Preferably utilize the container client when developing a singleton struct
    // let client = ClientBuilder::new(account, storage_credentials).container_client(container_name);
    // client.blob_client(blob_name);

    upload_blob(&client, upload_file_path).await?;
    download_blob(&client, download_file_path).await?;
    delete_blob(&client).await?;

    Ok(())
}

async fn upload_blob(client: &BlobClient, file_path: &str) -> Result<(), Error> {
    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    // client.put_block_blob("hello world").content_type("text/plain").await?;
    client.put_block_blob(buffer).await?;
    Ok(())
}

async fn download_blob(client: &BlobClient, file_path: &str) -> Result<(), Error> {
    let data = client.get_content().await?;

    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        .open(file_path)?;

    file.write_all(&data)?;

    Ok(())
}

async fn delete_blob(client: &BlobClient) -> Result<(), Error> {
    client.delete().await?;

    Ok(())
}
