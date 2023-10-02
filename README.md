# azure-blob-storage-sample

Repository demonstrating how to interface (up- and download blobs) with public Azure Storage Account Services with Rust and required third-party crates.

## How to use

0. Run the docker compose cluster to have an Azurite docker container locally running:

```bash
sudo docker compose up -d --build
```

1. Run tests

| Scripting language | Description | 
|----------|----------|
| Bash | Navigate to the [bash scripts](./scripts/bash/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob up- and downloads to/from the azurite docker container. | 
| Python | Navigate to the [python scripts](./scripts/python/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob uploads to the the azurite docker container. | 

2. Run Rust sample

The Rust sample can be started with `cargo run`. Please note that the azurite docker container can not be used for blobs due to missing support for connection strings. Instead the `account` name and `access_key` needs to be set, e.g.

```rust
// Use your public Azure Storage Account credentials
let account = String::from("devstoreaccount1"); //Resolves into `devstoreaccount1.blob.core.windows.net`
let access_key = String::from("Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==");
```
