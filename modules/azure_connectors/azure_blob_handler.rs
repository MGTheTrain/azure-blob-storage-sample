
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
//
// Maintainers:
// - MGTheTrain 
//
// Contributors:
// - TBD

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
