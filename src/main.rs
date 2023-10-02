use azure_core::prelude::*;
use azure_storage::prelude::*;
use azure_storage_blobs::{container::PublicAccess, prelude::*};
use bytes::Bytes;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // let file_name = String::from("azure_sdk_for_rust_stream_test.txt");

    // First we retrieve the account name and access key from environment variables.
    let account = String::from("http://127.0.0.1:10000/devstoreaccount1");
    //127.0.0.1:10000/devstoreaccount1.blob.core.windows.net
    let access_key = String::from("Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==");

    // let container = String::from("mycontainer");
    // let blob_name = String::from("test.txt");
    let blob_name: &'static str = "append_blob.txt";
    let container_name: &'static str = "rust-upload-test";
    let _data = b"abcdef";

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let blob_service = BlobServiceClient::new(account, storage_credentials);
    let container = blob_service.container_client(container_name);
    let blob = container.blob_client(blob_name);

    if !blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap()
        .containers
        .iter()
        .any(|x| x.name == container_name)
    {
        container
            .create()
            .public_access(PublicAccess::None)
            .await
            .unwrap();
    }

    let mut metadata = Metadata::new();
    metadata.insert("attrib", "value");
    metadata.insert("second", "something");

    blob.put_append_blob()
        .content_type("text/plain")
        .metadata(metadata)
        .await
        .unwrap();

    println!("created {:?}", blob_name);

    let resp = blob.get_metadata().await.unwrap();

    assert_eq!(resp.metadata.len(), 2);

    assert_eq!(resp.metadata.get("attrib"), Some(Bytes::from("value")));
    assert_eq!(resp.metadata.get("second"), Some(Bytes::from("something")));
    assert_eq!(resp.metadata.get("not_found"), None);

    Ok(())
}
