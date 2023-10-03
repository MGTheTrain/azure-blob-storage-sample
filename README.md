# azure-blob-storage-sample

## Table of Contents

+ [Summary](#summary)
+ [References](#references)
+ [How to use](#how-to-use)

## Summary

Repository demonstrating how to manage blobs in public Azure Storage Account Services container with Rust and required third-party crates.

## References

- [azure-sdk-for-rust](https://github.com/Azure/azure-sdk-for-rust)

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

The [Rust sample](./src/main.rs) can be started with `cargo run`. Please note that the azurite docker container can not be used for blob management (up-, download, deletion, metadata retrieval, etc.) here due to missing support for connection strings. Instead the `account` name and `access_key` needs to be set in the [main.rs](./src/main.rs), e.g.

```rust
// Use your public Azure Storage Account credentials
let account = String::from("devstoreaccount1"); //Resolves into `devstoreaccount1.blob.core.windows.net`
let access_key = String::from("Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==");
```
