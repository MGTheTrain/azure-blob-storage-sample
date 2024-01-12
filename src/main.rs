use std::{
    env,
    fs::{self, File},
    io::{BufReader, Read, Write},
};

use azure_storage::StorageCredentials;
use log::info;

use azure_core::Error;
use azure_storage_blobs::{blob, prelude::*};

use clap::{Parser, Subcommand};

use colored::Colorize;

/// Provide arguments for
#[derive(Parser, Debug)]
#[clap(
    author = "MGTheTrain",
    version = "1.0.0",
    about = "A Cli tool enabling blob operations (deletion, upload and download of blobs) in an Azure Storage Account Container."
)]
struct Cli {
    /// the azure storage account container pperation
    #[clap(subcommand)]
    operation: AzureStorageAccountContainerOperation,
}

#[derive(Debug, Subcommand)]
enum AzureStorageAccountContainerOperation {
    /// Upload operation arguments
    Upload {
        /// the blob name
        #[clap(short, long)]
        blob_name: Option<String>,
        /// the file path of the blob to be uploaded
        #[clap(short, long)]
        upload_file_path: Option<String>,
    },
    /// Download operation arguments
    Download {
        /// the blob name
        #[clap(short, long)]
        blob_name: Option<String>,
        /// the file path in which the blob should be downloaded
        #[clap(short, long)]
        download_file_path: Option<String>,
    },
    /// Delete operation arguments
    Delete {
        /// the blob name
        #[clap(short, long)]
        blob_name: Option<String>,
    },
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    let mut colored_string: colored::ColoredString;

    let env_file_path = "secrets.cfg";
    dotenv::from_path(env_file_path).ok();

    let mut account = String::from("devstoreaccount1"); //resolves to devstoreaccount1.blob.core.windows.net
    let mut access_key = String::from(
        "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==",
    );

    if let Some(value) = set_env_var("AZURE_ACCOUNT_NAME") {
        account = value;
    }
    if let Some(value) = set_env_var("AZURE_ACCESS_KEY") {
        access_key = value;
    }

    colored_string = "Error: AZURE_CONTAINER_NAME environment variable expected".red();
    let container_name = std::env::var("AZURE_CONTAINER_NAME").expect(&colored_string.to_string());

    // parse args
    let args = Cli::parse();

    match &args.operation {
        AzureStorageAccountContainerOperation::Upload {
            upload_file_path,
            blob_name,
        } => {
            colored_string = "Selected operation: Upload".blue();
            info!("{}", colored_string);

            if blob_name.is_none() {
                colored_string = "Error: --blob-name input argument needs to be set".red();
                panic!("{}", colored_string)
            }

            if upload_file_path.is_none() {
                colored_string = "Error: --upload-file-path input argument needs to be set".red();
                panic!("{}", colored_string)
            }

            let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
            let client = ClientBuilder::new(account, storage_credentials)
                .blob_client(container_name, blob_name.clone().unwrap());

            upload_blob(&client, &upload_file_path.clone().unwrap()).await?;
        }
        AzureStorageAccountContainerOperation::Download {
            download_file_path,
            blob_name,
        } => {
            colored_string = "Selected operation: Download".blue();
            info!("{}", colored_string);

            if blob_name.is_none() {
                colored_string = "Error: --blob-name input argument needs to be set".red();
                panic!("{}", colored_string)
            }

            if download_file_path.is_none() {
                colored_string = "Error: --upload-file-path input argument needs to be set".red();
                panic!("{}", colored_string)
            }

            let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
            let client = ClientBuilder::new(account, storage_credentials)
                .blob_client(container_name, blob_name.clone().unwrap());

            download_blob(&client, &download_file_path.clone().unwrap()).await?;
        }
        AzureStorageAccountContainerOperation::Delete { blob_name } => {
            colored_string = "Selected operation: Delete".blue();
            info!("{}", colored_string);

            if blob_name.is_none() {
                colored_string = "Error: --blob-name input argument needs to be set".red();
                panic!("{}", colored_string)
            }

            let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
            let client = ClientBuilder::new(account, storage_credentials)
                .blob_client(container_name, blob_name.clone().unwrap());

            delete_blob(&client).await?;
        }
        _ => {
            colored_string = "Error: Operation not supported".red();
            panic!("{}", colored_string)
        }
    }

    Ok(())
}

fn set_env_var(env_var_name: &str) -> Option<String> {
    match env::var(env_var_name) {
        Ok(value) => Some(value),
        Err(_) => {
            info!("{} is not set.", env_var_name.blue());
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

    let mut colored_string: colored::ColoredString;
    colored_string = format!("Successfully uploaded {} blob", client.blob_name()).blue();
    info!("{}", colored_string);

    Ok(())
}

async fn download_blob(client: &BlobClient, file_path: &str) -> Result<(), Error> {
    let data = client.get_content().await?;

    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        .open(file_path)?;

    file.write_all(&data)?;

    let mut colored_string: colored::ColoredString;
    colored_string = format!("Successfully downloaded {} blob", client.blob_name()).blue();
    info!("{}", colored_string);

    Ok(())
}

async fn delete_blob(client: &BlobClient) -> Result<(), Error> {
    client.delete().await?;

    let mut colored_string: colored::ColoredString;
    colored_string = format!("Successfully deleted {} blob", client.blob_name()).blue();
    info!("{}", colored_string);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{delete_blob, download_blob, set_env_var, upload_blob};
    use azure_storage::StorageCredentials;
    use azure_storage_blobs::prelude::ClientBuilder;
    use std::env;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    pub async fn test_azure_blob_storage_methods() -> Result<(), Box<dyn std::error::Error>> {
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

        let container_name = std::env::var("AZURE_CONTAINER_NAME")
            .expect("AZURE_CONTAINER_NAME environment variable expected");
        let blob_name = "sample.txt";
        let upload_file_path = "sample.txt";
        let download_file_path = "output/copy-sample.txt";

        let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
        let client =
            ClientBuilder::new(account, storage_credentials).blob_client(container_name, blob_name);

        assert!(upload_blob(&client, upload_file_path).await.is_ok());
        assert!(download_blob(&client, download_file_path).await.is_ok());
        assert!(delete_blob(&client).await.is_ok());

        Ok(())
    }
}
