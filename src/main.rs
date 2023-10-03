use std::{env, fs};

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

    let container_name = "rust-upload-test";
    let blob_name = "sample-blob";

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let client = ClientBuilder::new(account, storage_credentials).blob_client(container_name, blob_name);

    upload_sample_blob(&client).await?;

    Ok(())
}

async fn upload_sample_blob(client: &BlobClient) -> Result<(), Error> {
    client.put_block_blob("hello world").content_type("text/plain").await?;

    let mut result: Vec<u8> = vec![];

    // The stream is composed of individual calls to the get blob endpoint
    let mut stream = client.get().into_stream();
    while let Some(value) = stream.next().await {
        let mut body = value?.data;
        // For each response, we stream the body instead of collecting it all
        // into one large allocation.
        while let Some(value) = body.next().await {
            let value = value?;
            result.extend(&value);
        }
    }

    println!("result: {:?}", result);

    Ok(())
}