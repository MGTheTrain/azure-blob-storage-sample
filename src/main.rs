// The MIT License
//
// Copyright (c) 2024 MGTheTrain
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::{
    env,
    fs::{self, File},
    io::{BufReader, Read, Write},
};

use log::info;

use clap::{Parser, Subcommand};

use colored::Colorize;

use common_modules::connectors::azure_blob_handler::AzureBlobHandler;
use common_modules::set_env_var;

#[derive(Parser, Debug)]
#[clap(
    author = "MGTheTrain",
    version = "1.0.0",
    about = "A Cli tool enabling blob operations (deletion, upload and download of blobs) in an Azure Storage Account Container."
)]
struct Cli {
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

    let mut account = String::from("devstoreaccount1");
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

            let handler = AzureBlobHandler::new(
                &account,
                &access_key,
                &container_name,
                &blob_name.clone().unwrap(),
            );
            handler
                .upload_blob(&upload_file_path.clone().unwrap())
                .await?;
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

            let handler = AzureBlobHandler::new(
                &account,
                &access_key,
                &container_name,
                &blob_name.clone().unwrap(),
            );
            handler
                .download_blob(&download_file_path.clone().unwrap())
                .await?;
        }
        AzureStorageAccountContainerOperation::Delete { blob_name } => {
            colored_string = "Selected operation: Delete".blue();
            info!("{}", colored_string);

            if blob_name.is_none() {
                colored_string = "Error: --blob-name input argument needs to be set".red();
                panic!("{}", colored_string)
            }

            let handler = AzureBlobHandler::new(
                &account,
                &access_key,
                &container_name,
                &blob_name.clone().unwrap(),
            );
            handler.delete_blob().await?;
        }
        _ => {
            colored_string = "Error: Operation not supported".red();
            panic!("{}", colored_string)
        }
    }

    Ok(())
}
