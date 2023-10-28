use std::{env, fs::{self, File}, io::{BufReader, Read, Write}};

use azure_storage::StorageCredentials;
use log::info;

use azure_core::Error;
use azure_storage_blobs::prelude::*;

use clap::Parser;

/// Provide arguments for 
#[derive(Parser, Debug)]
#[clap(author="MGTheTrain", version="1.0.0", about="A Cli tool enabling blob operations (delete, upload and download blobs) in an Azure Storage Account Container.")]
struct Cli {
    /// the blob to operate on
    #[clap(short, long)]
    blob_name: Option<String>,
    /// the blob operation
    #[clap(short, long)]
    operation: Option<String>,
    /// the file path of the blob to be uploaded
    #[clap(short, long)]
    upload_file_path: Option<String>,
    /// the file path in which the blob should be downloaded
    #[clap(short, long)]
    download_file_path: Option<String>,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
        
    let env_file_path = "secrets.cfg";
    dotenv::from_path(env_file_path).ok();

    let mut account = String::from("devstoreaccount1"); //resolves to devstoreaccount1.blob.core.windows.net
    let mut access_key = String::from("Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==");

    if let Some(value) = set_env_var("AZURE_ACCOUNT_NAME") {
        account = value;
    } 
    if let Some(value) = set_env_var("AZURE_ACCESS_KEY") {
        access_key = value;
    } 

    let container_name = std::env::var("AZURE_CONTAINER_NAME").
        expect("AZURE_CONTAINER_NAME environment variable expected");

    // parse args
    let args = Cli::parse();

    if args.blob_name.is_none() {
        panic!("Error: Blob name is missing");
    }

    if args.operation.is_none() {
        panic!("Error: Blob operation is missing");
    } else {
        match args.operation.unwrap().as_str() {
            // Example upload: 
            // - cargo.exe run -- -o upload -b blob.txt  -u sample.txt
            // - cargo.exe run -- --operation upload --blob-name blob.txt --upload-file-path sample.txt
            "upload" => {
                print!("Upload operation to be applied");
                if args.upload_file_path.is_none() {
                    panic!("Error: Upload file path is missing");
                }

                let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
                let client = ClientBuilder::new(
                    account, storage_credentials).blob_client(container_name, args.blob_name.unwrap());
                
                upload_blob(&client, &args.upload_file_path.unwrap()).await?;
            },
            // Example download: 
            // - cargo.exe run -- -o download -b blob.txt -d output/download.txt
            // - cargo.exe run -- --operation download --blob-name blob.txt --download-file-path "output/download.txt"
            "download" => {
                print!("Download operation to be applied");
                if args.download_file_path.is_none() {
                    panic!("Error: Download file path is missing");
                }

                let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
                let client = ClientBuilder::new(
                    account, storage_credentials).blob_client(container_name, args.blob_name.unwrap());

                download_blob(&client, &args.download_file_path.unwrap()).await?;
            },
            // Example delete: 
            // - cargo.exe run -- -o delete -b blob.txt
            // - cargo.exe run -- --operation delete --blob-name blob.txt
            "delete" => {
                print!("Delete operation to be applied");

                let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
                let client = ClientBuilder::new(
                    account, storage_credentials).blob_client(container_name, args.blob_name.unwrap());

                delete_blob(&client).await?;
            },
            _ => {
                panic!("Error: Blob operation not supported");
            }
        }
    }

    // NOTE: Preferably utilize the container client when developing a singleton struct
    // let client = ClientBuilder::new(account, storage_credentials).container_client(container_name);
    // client.blob_client(blob_name);
    
    Ok(())
}

fn set_env_var(env_var_name: &str) -> Option<String> {
    match env::var(env_var_name) {
        Ok(value) => {
            Some(value)
        }
        Err(_) => {
            println!("{} is not set.", env_var_name);
            None
        }
    }
}

async fn upload_blob(client: &BlobClient, file_path: &str) -> Result<(), Error> {
    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    // client.put_block_blob("hello world").content_type("text/plain").await?;
    client.put_block_blob(buffer).await?;
    info!("Successfully uploaded {} blob", client.blob_name());

    Ok(())
}

async fn download_blob(client: &BlobClient, file_path: &str) -> Result<(), Error> {
    let data = client.get_content().await?;

    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        .open(file_path)?;

    file.write_all(&data)?;
    info!("Successfully downloaded {} blob", client.blob_name());

    Ok(())
}

async fn delete_blob(client: &BlobClient) -> Result<(), Error> {
    client.delete().await?;
    info!("Successfully deleted {} blob", client.blob_name());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;
    use azure_storage::StorageCredentials;
    use azure_storage_blobs::prelude::ClientBuilder;
    use crate::{upload_blob, download_blob, delete_blob, set_env_var};
    
    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    pub async fn test_azure_blob_storage_methods() -> Result<(), Box<dyn std::error::Error>>{
        env_logger::init();
        
        let env_file_path = "secrets.cfg";
        dotenv::from_path(env_file_path).ok();

        let mut account = String::from("devstoreaccount1"); //resolves to devstoreaccount1.blob.core.windows.net
        let mut access_key = String::from("Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==");
    
        if let Some(value) = set_env_var("AZURE_ACCOUNT_NAME") {
            account = value;
        } 
        if let Some(value) = set_env_var("AZURE_ACCESS_KEY") {
            access_key = value;
        } 
    
        let container_name = std::env::var("AZURE_CONTAINER_NAME").
            expect("AZURE_CONTAINER_NAME environment variable expected");
        let blob_name = "sample.txt";
        let upload_file_path = "sample.txt";
        let download_file_path = "output/copy-sample.txt";
    
        let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
        let client = ClientBuilder::new(account, storage_credentials).blob_client(container_name, blob_name);
    
        assert!(upload_blob(&client, upload_file_path).await.is_ok());
        assert!(download_blob(&client, download_file_path).await.is_ok());
        assert!(delete_blob(&client).await.is_ok());
    
        Ok(())
    }
}
