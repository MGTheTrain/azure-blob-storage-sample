import argparse
from azure.storage.blob import BlobServiceClient

def upload_blob(connection_string, container_name, blob_name, local_file_path):
    # Create a BlobServiceClient to interact with the Blob service
    blob_service_client = BlobServiceClient.from_connection_string(connection_string)

    # Get a ContainerClient to work with the container
    container_client = blob_service_client.get_container_client(container_name)

    try:
        container_client.get_container_properties()
        print(f"Container '{container_name}' already exists.")
    except Exception as e:
        # If the container doesn't exist, create it
        container_client.create_container()
        print(f"Container '{container_name}' created successfully.")

    # Create a blob client to upload a file
    blob_client = container_client.get_blob_client(blob_name)

    # Upload the file to Azure Blob Storage
    with open(local_file_path, "rb") as data:
        blob_client.upload_blob(data)

    print(f"File '{blob_name}' uploaded to container '{container_name}'")

def main():
    parser = argparse.ArgumentParser(description="Upload a file to Azure Blob Storage")
    parser.add_argument("--connection-string", required=True, help="Azure Blob Storage connection string")
    parser.add_argument("--container-name", required=True, help="Name of the container")
    parser.add_argument("--blob-name", required=True, help="Name of the blob in the container")
    parser.add_argument("--local-file-path", required=True, help="Local file path to upload")

    args = parser.parse_args()
    print(f"connection_string: {args.connection_string}")
    print(f"container_name: {args.container_name}")
    print(f"blob_name: {args.blob_name}")
    print(f"local_file_path: {args.local_file_path}")

    upload_blob(args.connection_string, args.container_name, args.blob_name, args.local_file_path)

if __name__ == "__main__":
    main()
