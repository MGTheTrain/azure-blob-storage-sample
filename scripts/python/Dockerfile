FROM python:alpine

WORKDIR project
COPY . .
RUN /bin/sh -c "pip install -r requirements.txt"

# Precondition: 
# cd ../../ && sudo docker compose up -d --build
# Build: 
# sudo docker build -t azure-blob-storage-test:python .
# Run: 
# - [Enter docker container] 
#   sudo docker run --rm --network azure-blob-storage_custom-network -it azure-blob-storage-test:python sh
#   ping azurite
#   python upload-file-to-az-storage-account-container.py --connection-string "DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;BlobEndpoint=http://azurite:10000/devstoreaccount1;" --container-name "my-container" --blob-name "python-sample.txt" --local-file-path "python-sample.txt"
# - [Use python script once trough docker container] 
#   sudo docker run --rm --network azure-blob-storage_custom-network azure-blob-storage-test:python /bin/sh -c \
#   "python upload-file-to-az-storage-account-container.py --connection-string \"DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;BlobEndpoint=http://azurite:10000/devstoreaccount1;\" --container-name \"my-container\" --blob-name \"python-sample.txt\" --local-file-path \"python-sample.txt\""

# Executing locally:
# python3 upload-file-to-az-storage-account-container.py --connection-string "DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;BlobEndpoint=http://localhost:10000/devstoreaccount1;" --container-name "my-container" --blob-name "local-python-sample.txt" --local-file-path "local-python-sample.txt"