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

#[cfg(test)]
mod tests {
    use common_modules::azure_connectors::azure_blob_handler::AzureBlobHandler;
    use common_modules::set_env_var;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    pub async fn test_azure_blob_storage_methods() -> Result<(), Box<dyn std::error::Error>> {
        env_logger::init();

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
        let upload_file_path = "test/assets/sample.txt";
        let download_file_path = "test/output/copy-sample.txt";

        let handler = AzureBlobHandler::new(&account, &access_key, &container_name, &blob_name);
        handler.upload_blob(upload_file_path).await?;
        handler.download_blob(download_file_path).await?;
        handler.delete_blob().await?;

        Ok(())
    }
}
