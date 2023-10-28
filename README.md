# rust-azure-blob-storage-sample

## Table of Contents

+ [Summary](#summary)
+ [References](#references)
+ [How to use](#how-to-use)

## Summary

Repository demonstrating how to manage blobs in public Azure Storage Account Services container with Rust and required third-party crates.

## References

- [azure_storage_blobs samples](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage_blobs)

## How to use

**0. Run the docker compose cluster to have an Azurite docker container locally running:**

```bash
sudo docker compose up -d --build
```

**1. Run tests**

| Scripting language | Description | 
|----------|----------|
| Bash | Navigate to the [bash scripts](./scripts/bash/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob up- and downloads to/from the azurite docker container. | 
| Python | Navigate to the [python scripts](./scripts/python/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob uploads to the the azurite docker container. | 

**2. Run Rust sample**

The [Rust sample](./src/main.rs) can be started with `cargo run`. Please note that the azurite docker container can not be used for local blob management tests (up-, download, deletion, metadata retrieval, etc.) on Windows 10 OS. You need to utilize a public Azure Storage Account Service container.

Therefore create from the [secrets.template.cfg](./templates/secrets.template.cfg) a `secrets.cfg` file and replace the `<PLACEHOLDER_*>` values.
Afterwards execute the following:

```bash
source templates/secrets.cfg
cargo run
```
