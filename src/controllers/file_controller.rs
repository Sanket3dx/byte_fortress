use crate::data_schema::files_schema::*;
use crate::rocks_db_operations;
use actix_web::{web, HttpRequest, HttpResponse};
use bson::{oid::ObjectId, Binary};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct FileResponse {
    id: ObjectId,
    name: String,
    size: u64,
    mime_type: String,
    created_at: String,
    updated_at: Option<String>,
    owner_id: ObjectId,
    storage_location: String,
    full_file_path: String,
    metadata: Metadata,
}

pub async fn handle_file_get(req: HttpRequest) -> HttpResponse {
    let full_file_path = match req
        .headers()
        .get("x-full-file-path")
        .and_then(|v| v.to_str().ok())
    {
        Some(path) => path,
        None => return HttpResponse::BadRequest().body("x-full-file-path header is missing"),
    };
    if let Some(file) = rocks_db_operations::retrieve_file(full_file_path.to_string()) {
        HttpResponse::Ok().body(file.binary_content)
    } else {
        HttpResponse::InternalServerError()
            .body(format!("Failed to retrieve file at path: {}", full_file_path))
    }
}

pub async fn handle_file_post(req: HttpRequest, file_data: web::Bytes) -> HttpResponse {
    let full_file_path = match req
        .headers()
        .get("x-full-file-path")
        .and_then(|v| v.to_str().ok())
    {
        Some(path) => path,
        None => return HttpResponse::BadRequest().body("x-full-file-path header is missing"),
    };

    let content_type = match req
        .headers()
        .get("Content-Type")
        .and_then(|v| v.to_str().ok())
    {
        Some(content_type) => content_type,
        None => return HttpResponse::BadRequest().body("Content-Type header is missing"),
    };

    let file_path = Path::new(full_file_path);
    let file_name = match file_path.file_name().and_then(|v| v.to_str()) {
        Some(name) => name,
        None => return HttpResponse::BadRequest().body("Invalid file path"),
    };

    let file_location = match file_path.parent().and_then(|v| v.to_str()) {
        Some(location) => location,
        None => return HttpResponse::BadRequest().body("Invalid file path"),
    };

    // Validate file size
    let binary_data = file_data.to_vec();
    let size = binary_data.len() as u64;
    // if size > 10_000_000 {
    //     return HttpResponse::BadRequest().body("File size exceeds the limit");
    // }

    // Create FileSchema object
    let file = FileSchema {
        id: ObjectId::new(),
        name: file_name.to_string(),
        size,
        mime_type: content_type.to_string(),
        binary_content: binary_data,
        created_at: chrono::Utc::now().to_string(),
        updated_at: None,
        owner_id: ObjectId::new(),
        storage_location: file_location.to_string(),
        full_file_path: full_file_path.to_string(),
        metadata: Metadata {
            description: Some(String::from("File description")),
            tags: vec![String::from("tag1"), String::from("tag2")],
        },
    };

    // Store the file in the database
    if let Err(err) = rocks_db_operations::store_file(full_file_path.to_string(), &file) {
        eprintln!("Failed to store file: {}", err);
        return HttpResponse::InternalServerError().body("Failed to store file");
    }
    let file_response = FileResponse {
        id: file.id,
        name: file.name,
        size: file.size,
        mime_type: file.mime_type,
        created_at: file.created_at,
        updated_at: file.updated_at,
        owner_id: file.owner_id,
        storage_location: file.storage_location,
        full_file_path: file.full_file_path,
        metadata: file.metadata,
    };

    // Return a 201 Created status code with JSON body
    HttpResponse::Created()
        .json(json!({"message": "File uploaded successfully", "file": file_response}))
}

pub async fn handle_file_put() -> HttpResponse {
    HttpResponse::Ok().body("File put successfully")
}

pub async fn handle_file_delete(req: HttpRequest) -> HttpResponse {
    let full_file_path = match req
        .headers()
        .get("x-full-file-path")
        .and_then(|v| v.to_str().ok())
    {
        Some(path) => path,
        None => return HttpResponse::BadRequest().body("x-full-file-path header is missing"),
    };

    match rocks_db_operations::delete_file(full_file_path.to_string()) {
        Ok(()) => HttpResponse::Ok().body("File deleted successfully"),
        Err(err) => {
            eprintln!("Failed to delete file: {}", err);
            HttpResponse::InternalServerError().body("Failed to delete file")
        }
    }
}

pub async fn index() -> HttpResponse {
    let html_content = r#"
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Byte Fortress</title>
        <!-- Bootstrap CSS -->
        <link
          href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css"
          rel="stylesheet"
        />
        <style>
          body {
            padding-top: 50px;
            background-color: #f0f0f0;
            color: #333;
          }
    
          .container {
            max-width: 600px;
            margin: auto;
            padding: 20px;
            background-color: #fff;
            border-radius: 10px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
          }
    
          h1 {
            margin-top: 0;
            color: #333;
          }
    
          .form-group {
            margin-bottom: 20px;
          }
          form {
            margin-top: 20px;
          }
    
          input[type="text"],
          input[type="file"],
          button[type="submit"] {
            width: 100%;
            padding: 10px;
            border-radius: 5px;
          }
    
          .message {
            margin-top: 10px;
            padding: 10px;
            border-radius: 5px;
            display: none;
          }
    
          .message.success {
            background-color: #d4edda;
            border-color: #c3e6cb;
            color: #155724;
          }
    
          .message.error {
            background-color: #f8d7da;
            border-color: #f5c6cb;
            color: #721c24;
          }
    
          .loader {
            border: 6px solid #f3f3f3;
            border-top: 6px solid #3498db;
            border-radius: 50%;
            width: 30px;
            height: 30px;
            animation: spin 1s linear infinite;
            margin: auto;
            display: none;
          }
    
          @keyframes spin {
            0% {
              transform: rotate(0deg);
            }
            100% {
              transform: rotate(360deg);
            }
          }
        </style>
      </head>
      <body>
        <div class="container">
          <h1 class="mb-4">Byte Fortress</h1>
    
          <form id="uploadForm" enctype="multipart/form-data">
            <div class="form-group">
              <input
                type="file"
                id="uploadFile"
                name="file"
                class="form-control"
                required
              />
            </div>
            <div class="form-group">
              <input
                type="text"
                id="uploadFilePath"
                class="form-control"
                placeholder="Enter file path"
              />
            </div>
            <button type="submit" class="btn btn-primary">Upload</button>
          </form>
          <hr />
          <form id="downloadForm">
            <div class="form-group">
              <input
                type="text"
                id="downloadPath"
                name="filePath"
                class="form-control"
                placeholder="Enter file path"
                required
              />
            </div>
            <button type="submit" class="btn btn-success">Download</button>
          </form>
          <hr />
          <form id="updateForm" enctype="multipart/form-data">
            <div class="form-group">
              <input
                type="file"
                id="updateFile"
                name="file"
                class="form-control"
                required
              />
            </div>
            <div class="form-group">
              <input
                type="text"
                id="updateFilePath"
                class="form-control"
                placeholder="Enter file path"
                required
              />
            </div>
            <button type="submit" class="btn btn-secondary">Update</button>
          </form>
          <hr />
          <form id="deleteForm">
            <div class="form-group">
              <input
                type="text"
                id="deletePath"
                name="filePath"
                class="form-control"
                placeholder="Enter file path"
                required
              />
            </div>
            <button type="submit" class="btn btn-danger">Delete</button>
          </form>
    
          <div id="message" class="message"></div>
          <div id="loader" class="loader"></div>
        </div>
    
        <!-- Bootstrap JS -->
        <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/js/bootstrap.bundle.min.js"></script>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/jquery/3.6.0/jquery.min.js"></script>
        <script>
          $(document).ready(function () {
            $(".message").addClass("fadeIn");
    
            $("\#uploadForm").submit(function (e) {
              e.preventDefault();
              $("\#loader").show();
              var file = $("\#uploadFile")[0].files[0];
              var filePath = $("\#uploadFilePath").val();
              $.ajax({
                type: "POST",
                url: "/file",
                data: file,
                headers: {
                  "x-full-file-path": filePath,
                },
                contentType: false,
                processData: false,
                success: function (response) {
                  $("\#loader").hide();
                  setMessage("File uploaded successfully", "success");
                },
                error: function (xhr, status, error) {
                  $("\#loader").hide();
                  setMessage("Error uploading file", "error");
                },
              });
            });
    
            $("\#downloadForm").submit(function (e) {
              e.preventDefault();
              var filePath = $("\#downloadPath").val();
              var xhr = new XMLHttpRequest();
              xhr.open("GET", "/file", true);
              xhr.setRequestHeader("x-full-file-path", filePath);
              xhr.responseType = "arraybuffer";
              xhr.onload = function () {
                if (xhr.status === 200) {
                  var blob = new Blob([xhr.response]);
                  var url = window.URL.createObjectURL(blob);
                  var a = document.createElement("a");
                  a.href = url;
                  a.download = filePath.substr(filePath.lastIndexOf("/") + 1);
                  document.body.appendChild(a);
                  a.click();
                  window.URL.revokeObjectURL(url);
                } else {
                  console.error("Error downloading file:", xhr.status);
                }
              };
              xhr.send();
            });
    
            $("\#updateForm").submit(function (e) {
              e.preventDefault();
              $("\#loader").show();
              var file = $("\#updateFile")[0].files[0];
              var filePath = $("\#updateFilePath").val();
              formData.append("file", file);
              $.ajax({
                type: "POST",
                url: "/file",
                headers: {
                  "x-full-file-path": filePath,
                },
                data: file,
                contentType: false,
                processData: false,
                success: function (response) {
                  $("\#loader").hide();
                  setMessage("File updated successfully", "success");
                },
                error: function (xhr, status, error) {
                  $("\#loader").hide();
                  setMessage("Error updating file", "error");
                },
              });
            });
    
            $("\#deleteForm").submit(function (e) {
              e.preventDefault();
              $("\#loader").show();
              var filePath = $("\#deletePath").val();
              $.ajax({
                type: "DELETE",
                url: "/file",
                headers: {
                  "x-full-file-path": filePath,
                },
                success: function (response) {
                  $("\#loader").hide();
                  setMessage("File deleted successfully", "success");
                },
                error: function (xhr, status, error) {
                  $("\#loader").hide();
                  setMessage("Error deleting file", "error");
                },
              });
            });
    
            function setMessage(message, type) {
              var messageElement = $("\#message");
              messageElement.text(message);
              messageElement.addClass("message " + type);
              messageElement.fadeIn().delay(3000).fadeOut();
            }
          });
        </script>
      </body>
    </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}