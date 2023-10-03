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

The [Rust sample](./src/main.rs) can be started with `cargo run`. Please note that the azurite docker container can not be used for local blob management tests (up-, download, deletion, metadata retrieval, etc.). You need to utilize a public Azure Storage Account Service container.

Therefore create from the [secrets.cfg.template](./secrets.cfg.template) a `secrets.cfg` file and replace the `<PLACEHOLDER_*>` values.
Afterwards execute the following:

```bash
source secrets.cfg
cargo run
```
