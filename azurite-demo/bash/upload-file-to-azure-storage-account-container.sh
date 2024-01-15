#!/bin/bash

# Set environment variables for Azurite (assuming default ports)
# export AZURE_STORAGE_CONNECTION_STRING="DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;BlobEndpoint=http://127.0.0.1:10000/devstoreaccount1;TableEndpoint=http://127.0.0.1:10002/devstoreaccount1;QueueEndpoint=http://127.0.0.1:10001/devstoreaccount1;"
export AZURE_STORAGE_CONNECTION_STRING="DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;BlobEndpoint=http://azurite:10000/devstoreaccount1;TableEndpoint=http://azurite:10002/devstoreaccount1;QueueEndpoint=http://azurite:10001/devstoreaccount1;"

# Create a container
az storage container create --connection-string "$AZURE_STORAGE_CONNECTION_STRING" -n mycontainer

# Upload a sample blob
az storage blob upload --connection-string "$AZURE_STORAGE_CONNECTION_STRING" --container-name mycontainer --name myblob --type block --file ./sample.txt

# Access Azurite Storage Explorer via a web browser
echo "Access Azurite Storage Explorer via: http://localhost:10002/devstoreaccount1"
