# Azure Storage implementation for the cache interface

This crate contains an Azure Storage implementation for the WASI experimental
cache interface. It is not intended to be a stable or feature complete.

The component expects the storage account, key, and container to be passed as
environment variables at runtime:

```
export STORAGE_ACCOUNT=<storage account>
export STORAGE_MASTER_KEY=<storage account access key>
export CONTAINER=<storage container>
```
