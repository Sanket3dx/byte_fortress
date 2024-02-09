# Byte Fortress

Byte Fortress is a Rust-based file storage and management system that utilizes RocksDB for efficient data storage and retrieval.

## Features

- **Efficient Storage:** Utilizes RocksDB, a high-performance embedded database, for efficient data storage and retrieval.
- **RESTful API:** Provides a RESTful API for uploading, downloading, updating, and deleting files.
- **Easy Deployment:** All dependencies are packaged within a single binary for easy deployment and use.

## Installation

To use Byte Fortress, follow these steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/Sanket3dx/byte_fortress.git

2. Build the project:

    ```bash
   cargo build --release

3. Run the server:

   ```bash
   ./target/release/byte_fortress


## Usage

### 1. Uploading a File

  To upload a file, send a POST request to the /file endpoint with the file data in the request body. Include the file path in the x-full-file-path header and the Content-Type header with the appropriate MIME type.

  Example using curl:

    curl -X POST \
    -H "x-full-file-path: /path/to/file.txt" \
    -H "Content-Type: application/octet-stream" \
    --data-binary @/path/to/file.txt \
    http://localhost:8080/file


### 2. Downloading a File

  To download a file, send a GET request to the /file endpoint with the file path in the x-full-file-path header.

  Example using curl:

    curl -X GET \
      -H "x-full-file-path: /path/to/file.txt" \
      http://localhost:8080/file
      
### 3. Updating a File

To update a file, send a POST request to the `/file` endpoint with the updated file data in the request body. Include the file path in the `x-full-file-path` header and the `Content-Type` header with the appropriate MIME type. If a file with the same path already exists, it will be overwritten with the new data.

  Example using `curl`:
  
    curl -X POST \
      -H "x-full-file-path: /path/to/file.txt" \
      -H "Content-Type: application/octet-stream" \
      --data-binary @/path/to/updated_file.txt \
      http://localhost:8080/file

### 3. Updating a File

To update a file, send a POST request to the `/file` endpoint with the updated file data in the request body. Include the file path in the `x-full-file-path` header and the `Content-Type` header with the appropriate MIME type. If a file with the same path already exists, it will be overwritten with the new data.

  Example using `curl`:
  
  ```bash
    curl -X DELETE \
      -H "x-full-file-path: /path/to/file.txt" \
      http://localhost:8080/file


