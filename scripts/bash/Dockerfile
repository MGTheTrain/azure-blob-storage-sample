FROM ubuntu:20.04

WORKDIR project

RUN /bin/sh -c "apt-get update && \
                apt-get install -y curl && \ 
                curl -sL https://aka.ms/InstallAzureCLIDeb | bash"

COPY . .
RUN /bin/sh -c "chmod +x *.sh"

# Precondition: 
# cd ../../ && sudo docker compose up -d --build
# Build: 
# sudo docker build -t azure-blob-storage-test:bash .
# Run: 
# - [Enter docker container] 
#   sudo docker run --rm --network azure-blob-storage_custom-network -it azure-blob-storage-test:bash bash
#   ping azurite # we should see here responses from the azurite docker container
# - [Use bash script once trough docker container] 
#   sudo docker run --rm --network azure-blob-storage_custom-network \
#   azure-blob-storage-test:bash /bin/bash -c \
#   "./upload-file-to-azure-storage-account-container.sh && \
#    ./download-file-to-azure-storage-account-container.sh && \
#    cat sample-download.txt"
