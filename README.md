# rust-azure-blob-storage-handler

## Table of Contents

- [Summary](#summary)
- [References](#references)
- [How to use](#how-to-use)
  - [Rust](#rust)
  - [Optional](#optional)

## Summary

Repository demonstrating how to manage blobs in public Azure Storage Account Services container with Rust and required third-party crates.

## References

- [azure_storage_blobs samples](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage_blobs)
- [Clap cli crate - Handling arguments and subcommands](https://rust-cli-recommendations.sunshowers.io/handling-arguments.html)

## How to use

### Rust

The [Rust sample](./src/main.rs) can be started with `cargo run` and tests can be run with `RUST_LOG=info cargo test` in a Unix terminal (Git Bash on Windows OS, etc.). Please note that the azurite docker container can not be used for local blob management tests (upload, download, deletion, metadata retrieval, etc.) on Windows 10 OS. You need to utilize a public Azure Storage Account Service container.

Create from the [secrets.template.cfg](./templates/secrets.template.cfg) in the [templates folder](./templates/) a `secrets.cfg` file in the project root directory and replace the `<PLACEHOLDER_*>` values. The [test_azure_blob_handler.rs](./test/test_azure_blob_handler.rs) and [main.rs](./src/main.rs) will export the environment variables trough the `secrets.cfg` file.
Afterwards execute the following to run the tests:

```bash
cargo test
# for more logs execute
RUST_LOG=info cargo test
```

Build and run the cli tool/application binary with:

```bash
# Example upload: 
RUST_LOG=info cargo run -- upload -b blob.txt  -u assets/sample.txt
RUST_LOG=info cargo run -- upload --blob-name blob.txt --upload-file-path assets/sample.txt 

# Example download: 
RUST_LOG=info cargo run -- download -b blob.txt -d output/download.txt
RUST_LOG=info cargo run -- download --blob-name blob.txt --download-file-path "output/download.txt"

# Example delete: 
RUST_LOG=info cargo run -- delete -b blob.txt
RUST_LOG=info cargo run -- delete --blob-name blob.txt

#####################################################################################################################
# Running the executable without cargo  
cargo build
cp target/debug/azure_blob_handler.exe . # On Windows OS when utilizing Git Bash or WSL
source secrets.cfg
./azure_blob_handler --help
# Example upload (Note: Colored crates console logs might not work on certain terminals): 
RUST_LOG=info ./azure_blob_handler upload --blob-name blob.txt --upload-file-path assets/sample.txt 
```

### Optional

#### Ramp up an Azurite docker container trough docker compose

```bash
sudo docker compose up -d --build
```

#### Run tests

| Scripting language | Description | 
|----------|----------|
| Bash | Navigate to the [bash scripts](./scripts/bash/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob up- and downloads to/from the azurite docker container. | 
| Python | Navigate to the [python scripts](./scripts/python/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob uploads to the the azurite docker container. | 
