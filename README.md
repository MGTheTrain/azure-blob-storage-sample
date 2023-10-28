# rust-azure-blob-storage-sample

## Table of Contents

+ [Summary](#summary)
+ [References](#references)
+ [How to use](#how-to-use)

## Summary

Repository demonstrating how to manage blobs in public Azure Storage Account Services container with Rust and required third-party crates.

## References

- [azure_storage_blobs samples](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage_blobs)
- [Clap cli crate - Handling arguments and subcommands](https://rust-cli-recommendations.sunshowers.io/handling-arguments.html)

## How to use

### Optional

#### Run the docker compose cluster to have an Azurite docker container locally running

```bash
sudo docker compose up -d --build
```

#### Run tests

| Scripting language | Description | 
|----------|----------|
| Bash | Navigate to the [bash scripts](./scripts/bash/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob up- and downloads to/from the azurite docker container. | 
| Python | Navigate to the [python scripts](./scripts/python/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob uploads to the the azurite docker container. | 

### Rust

The [Rust sample](./src/main.rs) can be started with `cargo run` and tests can be run with `RUST_LOG=info cargo test` in a Unix terminal (Git Bash on Windows OS, etc.). Please note that the azurite docker container can not be used for local blob management tests (up-, download, deletion, metadata retrieval, etc.) on Windows 10 OS. You need to utilize a public Azure Storage Account Service container.

Therefore create from the [secrets.template.cfg](./secrets.template.cfg) a `secrets.cfg` file and replace the `<PLACEHOLDER_*>` values.
Afterwards execute the following to run the tests:

```bash
cargo test
```

In order to build and run the main executable run:

```bash
cargo build

# Example upload: 
cargo.exe run -- upload -b blob.txt  -u sample.txt
cargo.exe run -- upload --blob-name blob.txt --upload-file-path sample.txt 

# Example download: 
cargo.exe run -- download -b blob.txt -d output/download.txt
cargo.exe run -- download --blob-name blob.txt --download-file-path "output/download.txt"

# Example delete: 
cargo.exe run -- delete -b blob.txt
cargo.exe run -- delete --blob-name blob.txt

# or running the executable  
cp target/debug/azure-blob-storage.exe .
source secrets.cfg
./azure-blob-storage.exe --help
# Example upload (Note: Colored crates console logs might not work on certain terminals): 
./azure-blob-storage.exe upload --blob-name blob.txt --upload-file-path sample.txt 
```