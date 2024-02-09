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
    if size > 10_000_000 {
        return HttpResponse::BadRequest().body("File size exceeds the limit");
    }

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
