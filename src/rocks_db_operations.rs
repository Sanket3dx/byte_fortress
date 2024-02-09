
use rocksdb::{DB, Error as RocksDBError};
use std::env;
use std::path::PathBuf;

use serde_json;
use crate::data_schema::files_schema::*;

pub fn store_file(key: String, file: &FileSchema) -> Result<(), String> {
    let mut path = match env::var_os("HOME") {
        Some(home) => PathBuf::from(home),
        None => PathBuf::from("/"),
    };
    path.push(".byte_fortress");

    let db = match DB::open_default(&path) {
        Ok(db) => db,
        Err(err) => return Err(format!("Failed to open RocksDB: {}", err)),
    };

    let file_json = match serde_json::to_string(file) {
        Ok(json) => json,
        Err(err) => return Err(format!("Failed to serialize file to JSON: {}", err)),
    };

    match db.put(key.as_bytes(), file_json.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to store file in RocksDB: {}", err)),
    }
}

pub fn retrieve_file(key: String) -> Option<FileSchema> {
   let mut path = match env::var_os("HOME") {
        Some(home) => PathBuf::from(home),
        None => {
            PathBuf::from("/")
        }
    };
    path.push(".byte_fortress");

    let db = DB::open_default(path).unwrap();

    if let Some(stored_file_json) = db.get(key.as_bytes()).unwrap() {
        let stored_file: FileSchema = serde_json::from_slice(&stored_file_json).unwrap();
        Some(stored_file)
    } else {
        None
    }
}

pub fn delete_file(key: String) -> Result<(), String> {
    let mut path = match env::var_os("HOME") {
        Some(home) => PathBuf::from(home),
        None => PathBuf::from("/"),
    };
    path.push(".byte_fortress");

    let db = match DB::open_default(&path) {
        Ok(db) => db,
        Err(err) => return Err(format!("Failed to open RocksDB: {}", err)),
    };

    match db.delete(key.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to delete file from RocksDB: {}", err))
    }
}