use bson::{bson, doc, oid::ObjectId, Binary};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSchema {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub size: u64,
    pub mime_type: String,
    pub binary_content: Vec<u8>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub owner_id: ObjectId,
    pub storage_location: String,
    pub full_file_path: String,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl FileSchema {
    fn to_bson(&self) -> bson::Document {
        bson::to_document(self).unwrap()
    }
}