use serde::{Deserialize, Serialize};

use crate::{Result, Client, Router, IndexRouter, IdResponse};


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewIdx{
    pub collection: String,

    pub name: String,

    /// Types:
    /// - fulltext
    /// - geo
    /// - inverted (not supported)
    /// - persistent
    /// - ttl
    /// - zkd
    #[serde(rename = "type")]
    pub r#type: String,
    pub fields: Vec<String>,

    pub in_background: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_json: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// "double" is only supported value
    pub field_value_types: Option<String>
}

impl Default for NewIdx {
    fn default() -> Self {
        Self { 
            collection: "".to_owned(), 
            name: "".to_owned(), 
            r#type: "persistent".to_owned(), 
            fields: vec![], 
            unique: None, 
            sparse: None, 
            deduplicate: None, 
            estimates: None, 
            cache_enabled: None, 
            geo_json: None,
            min_length: None,
            stored_values: None,
            expires_after: None,
            field_value_types: None,
            in_background: false
        }
    }
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Idx{
    pub fields: Vec<String>,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,

    pub selectivity_estimate: Option<u32>,
    pub unique: Option<bool>,
    pub min_length: Option<u8>,
    pub geo_json: Option<bool>,
    pub stored_values: Option<String>,
    pub sparse: Option<bool>,
    pub deduplicate: Option<bool>,
    pub estimates: Option<bool>,
    pub cache_enabled: Option<bool>,
    pub expires_after: Option<u64>,
    /// "double" is only supported value
    pub field_value_types: Option<String>
}


#[derive(Deserialize, Debug)]
pub struct IndexesResponse {
    pub error: bool,
    pub code: u32,
    pub indexes: Vec<Idx>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexResponse {
    pub error: bool,
    pub code: u32,
    pub fields: Vec<String>,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,

    pub selectivity_estimate: Option<u32>,
    pub unique: Option<bool>,
    pub min_length: Option<u8>,
    pub geo_json: Option<bool>,
    pub stored_values: Option<String>,
    pub sparse: Option<bool>,
    pub deduplicate: Option<bool>,
    pub estimates: Option<bool>,
    pub cache_enabled: Option<bool>,
    pub expires_after: Option<u64>,
    /// "double" is only supported value
    pub field_value_types: Option<String>,
    pub is_newly_created: Option<bool>
}

pub struct Index {
    pub client: Client
}

impl Index {
    /// Read all indexes of a collection
    pub async fn read(&self, collection_name: &str) -> Result<Vec<Idx>>{
        let response: IndexesResponse = self.client.get(Router::base_as_query(collection_name)).await?;
        Ok(response.indexes)
    }

    /// Get index for collection
    pub async fn get(&self, id: &str) -> Result<IndexResponse> {
        let response: IndexResponse = self.client.get(Router::base_as_path(id)).await?;
        Ok(response)
    }

    /// Create an index
    pub async fn create(&self, new_index: &NewIdx) -> Result<IndexResponse>{
        let response: IndexResponse = self.client.post(Router::base_as_query(&new_index.collection), new_index).await?;
        Ok(response)
    }

    /// Delete an index
    pub async fn delete(&self, id: &str) -> Result<IdResponse>{
        let response: IdResponse = self.client.delete(Router::base_as_path(id)).await?;
        Ok(response)
    }
}