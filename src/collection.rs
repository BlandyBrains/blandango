use serde::{Serialize, Deserialize};

use crate::{
    Result, Client, 
    Router, 
    IdResponse, CollectionRouter, 
    api::{API, self}, 
    Response, Empty, FlatResponse
};


#[derive(Serialize)]
pub struct CollectionQueryParams{
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "waitForSyncReplication")]
    pub wait_for_sync_replication: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enforceReplicationFactor")]
    pub enforce_replication_factor: Option<bool>
}

impl Default for CollectionQueryParams{
    fn default() -> Self {
        Self { 
            wait_for_sync_replication: Some(true), 
            enforce_replication_factor: Some(false)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComputedValue{
    pub name: String,
    pub expression: String,
    pub overwrite: bool,
    pub compute_on: Vec<String>,
    pub keep_null: bool,
    pub fail_on_warning: bool    
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeyOptions {
    pub allow_user_keys: bool,
    pub r#type: String,
    pub increment: Option<u32>,
    pub offset: Option<u32>,
    pub last_value: u32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Information {
    pub id: String,
    pub name: String,
    pub status: u8,
    #[serde(rename = "type")]
    pub r#type: u8,
    pub is_system: bool,
    pub globally_unique_id: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewCollection {
    pub name: String,    
    pub cache_enabled: bool,

    #[serde(rename = "type")]
    pub r#type: u8,

    pub is_system: bool,
    pub write_concern: u8,
    pub wait_for_sync: bool,
    pub replication_factor: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_values: Option<Vec<ComputedValue>>,

    #[serde( skip_serializing_if = "Option::is_none")]
    pub key_options: Option<KeyOptions>,

    #[serde( skip_serializing_if = "Option::is_none")]
    pub smart_join_attribute: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub is_disjoint: Option<bool>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub is_smart: Option<bool>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub number_of_shards: Option<u8>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub shard_keys: Option<Vec<String>>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub sharding_strategy: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub distrube_shard_like: Option<String>
}


impl NewCollection{

    pub fn default_computed_values() -> Vec<ComputedValue> {
        vec![
            ComputedValue{
                name: "created_on".to_owned(),
                expression: "RETURN DATE_NOW()".to_owned(),
                compute_on: vec![
                    "insert".to_owned(),
                    "replace".to_owned()
                ],
                overwrite: true,
                keep_null: false,
                fail_on_warning: true,
            },
            ComputedValue{
                name: "modified_on".to_owned(),
                expression: "RETURN DATE_NOW()".to_owned(),
                compute_on: vec![
                    "insert".to_owned(),
                    "update".to_owned(),
                    "replace".to_owned(),
                ],
                overwrite: true,
                keep_null: false,
                fail_on_warning: true,
            }
        ]
    }

    pub fn default_document_collection(name: String) -> Self{
        Self::new(name, 
            false, 
            Some(Self::default_computed_values()))
    }

    pub fn default_edge_collection(name: String) -> Self{
        let mut col: Self = Self::new(name, 
            false, 
            Some(Self::default_computed_values()));
        col.r#type = 3;
        return col;
    }

    pub fn new(name: String, cache_enabled: bool, computed_values: Option<Vec<ComputedValue>>) -> Self {
        Self {
            name, 
            wait_for_sync: true,
            is_system: false,
            r#type: 2,  // 2: Document, 3: Edge
            cache_enabled,
            computed_values,
            write_concern: 1,
            replication_factor: 1,
            key_options: None,
            smart_join_attribute: None,
            is_disjoint: None,
            is_smart: None,
            number_of_shards: None,
            shard_keys: None,
            sharding_strategy: None,
            distrube_shard_like: None
        }
    }
    
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Checksum {
    pub id: String,
    pub name: String,
    pub status: u8,
    #[serde(rename = "type")]
    pub r#type: u8,
    pub is_system: bool,
    pub globally_unique_id: String,

    pub revision: String,
    pub checksum: String
}

#[derive(Deserialize, Debug)]
pub struct Indexes {
    pub count: u32,
    pub size: u32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Figures{
    pub indexes: Indexes,
    pub documents_size: u32,
    pub cache_in_use: bool,
    pub cache_size: u32,
    pub cache_usage: u32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub id: String,
    pub name: String,
    pub status: u8,
    #[serde(rename = "type")]
    pub r#type: u8,
    pub is_system: bool,
    pub globally_unique_id: String,

    pub write_concern: u8,
    pub wait_for_sync: bool,
    pub uses_revisions_as_document_ids: bool,
    pub sync_by_revision: bool,
    pub status_string: String,
    pub internal_validator_type: u8,
    pub cache_enabled: bool,
    pub is_smart_child: bool,
    // pub schema: NOT_SUPPORTED,
    pub key_options: KeyOptions,
    pub computed_values: Option<Vec<ComputedValue>>,
    pub object_id: String,

    pub count: u32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub id: String,
    pub name: String,
    pub status: u8,
    #[serde(rename = "type")]
    pub r#type: u8,
    pub is_system: bool,
    pub globally_unique_id: String,

    pub write_concern: u8,
    pub wait_for_sync: bool,
    pub uses_revisions_as_document_ids: bool,
    pub sync_by_revision: bool,
    pub status_string: String,
    pub internal_validator_type: u8,
    pub cache_enabled: bool,
    pub is_smart_child: bool,
    // pub schema: NOT_SUPPORTED,
    pub key_options: KeyOptions,
    pub computed_values: Option<Vec<ComputedValue>>,
    pub object_id: String,

    pub count: u32,
    pub figures: Figures
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub id: String,
    pub name: String,
    pub status: u8,
    #[serde(rename = "type")]
    pub r#type: u8,
    pub is_system: bool,
    pub globally_unique_id: String,

    pub write_concern: u8,
    pub wait_for_sync: bool,
    pub uses_revisions_as_document_ids: bool,
    pub sync_by_revision: bool,
    pub status_string: String,
    pub internal_validator_type: u8,
    pub cache_enabled: bool,
    pub is_smart_child: bool,
    // pub schema: NOT_SUPPORTED,
    pub key_options: KeyOptions,
    pub computed_values: Option<Vec<ComputedValue>>,
    pub object_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PropertiesUpdate {
    pub wait_for_sync: bool,
    pub cache_enabled: bool,
    // pub schema: NOT_SUPPORTED,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub computed_values: Option<Vec<ComputedValue>>,
    pub replication_factor: u8,
    pub write_concern: u8
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Revision {
    pub id: String,
    pub name: String,
    pub status: u8,
    #[serde(rename = "type")]
    pub r#type: u8,
    pub is_system: bool,
    pub globally_unique_id: String,

    pub write_concern: u8,
    pub wait_for_sync: bool,
    pub uses_revisions_as_document_ids: bool,
    pub sync_by_revision: bool,
    pub status_string: String,
    pub internal_validator_type: u8,
    pub cache_enabled: bool,
    pub is_smart_child: bool,
    // pub schema: NOT_SUPPORTED,
    pub key_options: KeyOptions,
    pub computed_values: Option<Vec<ComputedValue>>,
    pub object_id: String,

    pub revision: String
}

#[derive(Serialize, Debug)]
pub struct Rename {
    pub name: String
}

pub struct Collection {
    pub client: Client,
    pub name: String
}

impl Collection
{
    pub fn new(client: Client, name: String) -> Self {
        Self { client, name }
    }

    /// Read all collections
    /// ```
    /// { 
    ///     "error" : false, 
    ///     "code" : 200, 
    ///     "result" : [ 
    ///       { 
    ///         "id" : "16", 
    ///         "name" : "_statisticsRaw", 
    ///         "status" : 3, 
    ///         "type" : 2, 
    ///         "isSystem" : true, 
    ///         "globallyUniqueId" : "_statisticsRaw" 
    ///       }
    ///     ]
    /// }
    /// ```
    pub async fn read(&self) -> Result<Vec<Information>> {
        let endpoint: String = Router::base();
        let response: Response<Vec<Information>> = self.client.get(endpoint).await?;
        Ok(response.result)
    }

    // /// Create a new collection with options
    // pub async fn create_with_options<R>(client: &Client, request: &CollectionRequest) -> Result<Properties> {
    //     let endpoint: String = Router::base_with_params(CollectionQueryParams::default())?;
    //     Ok(client.post(endpoint, &request).await?)
    // }
    
    /// Drop collection
    pub async fn drop(&self) -> Result<IdResponse> {
        let endpoint: String = Router::base_collection(&self.name);
        Ok(self.client.delete(endpoint).await?)
    }

    /// Get information about a collection
    /// Returns:
    /// {
    ///     "id": String,
    ///     "name": String,
    ///     "status": String,
    ///     "type": int,
    ///     "isSystem": bool
    /// }
    pub async fn information(&self) -> Result<Information> {
        let endpoint: String = Router::base_collection(&self.name);
        Ok(self.client.get(endpoint).await?)
    }

    /// Get Checksum
    pub async fn checksum(&self) -> Result<Checksum> {
        let endpoint: String = Router::extension(&self.name, API::Collection(api::Collection::Checksum));
        let response: FlatResponse<Checksum> = self.client.get(endpoint).await?;
        Ok(response.result)
    }
    
    /// Compact
    pub async fn compact(&self) -> Result<Information> {
        let endpoint: String = Router::extension(&self.name, API::Collection(api::Collection::Compact));
        let response: FlatResponse<Information> = self.client.put(endpoint, &Empty{}).await?;
        Ok(response.result)
    }
    
    /// Count
    pub async fn count(&self) -> Result<Count> {
        let endpoint: String = Router::extension(&self.name, API::Collection(api::Collection::Count));
        let response: FlatResponse<Count> = self.client.get(endpoint).await?;
        Ok(response.result)
    }    
    
    /// Figures
    pub async fn figures(&self) -> Result<Summary> {
        let endpoint: String = Router::extension(&self.name, API::Collection(api::Collection::Figures));
        let response: FlatResponse<Summary> = self.client.get(endpoint).await?;
        Ok(response.result)
    }
    
    /// Get the properties of the collection
    pub async fn properties(&self) -> Result<Properties>{
        let endpoint: String = Router::extension(&self.name, API::Collection(api::Collection::Properties));
        let response: FlatResponse<Properties> = self.client.get(endpoint).await?;
        Ok(response.result)
    }

    /// Update the properties of the collection
    pub async fn update_properties(&self, properties: &PropertiesUpdate) -> Result<Properties>{
        let endpoint: String = Router::extension(&self.name, API::Collection(api::Collection::Properties));
        let response: FlatResponse<Properties> = self.client.put(endpoint, &properties).await?;
        Ok(response.result)
    }

    /// Load Indexes into Memory
    pub async fn load_indexes(&self) -> Result<bool>{
        let endpoint: String = Router::extension(&self.name, API::Collection(api::Collection::LoadIndexIntoMemory));
        let response: Response<bool> = self.client.put(endpoint, &Empty{}).await?;
        Ok(response.result)
    }

    /// Recalculate Count
    pub async fn recalculate_count(&self) -> Result<bool>{
        let endpoint: String = Router::extension(&self.name, API::Collection(api::Collection::RecalculateCount));
        let response: Response<bool> = self.client.put(endpoint, &Empty{}).await?;
        Ok(response.result)
    }
    
    /// Rename
    pub async fn rename(&mut self, new_collection_name: &str) -> Result<Information>{
        let endpoint: String = Router::extension(&self.name, API::Collection(api::Collection::Rename));
        let collection: FlatResponse<Information> = self.client.put(endpoint, &Rename{ name: new_collection_name.to_owned()}).await?;
        self.name = new_collection_name.to_string();
        Ok(collection.result)
    }

    /// Responsible Shard for a document
    
    /// Revision
    pub async fn revision(&self) -> Result<Revision>{
        let endpoint: String = Router::extension(&self.name, API::Collection(api::Collection::Revision));
        let response: FlatResponse<Revision> = self.client.get(endpoint).await?;
        Ok(response.result)
    }

    /// Shards - Only available on Cluster Coordinator
    
    /// Truncate
    pub async fn truncate(&self) -> Result<Information> {
        let endpoint: String = Router::extension(&self.name, API::Collection(api::Collection::Truncate));
        let response: FlatResponse<Information> = self.client.put(endpoint, &Empty{}).await?;
        Ok(response.result)
    }
}

/*
    /// todo - relocate to dedicated Index API struct
    /// Create an index for the collection
    async fn create_index<R>(client: &Client, index: &mut Index) -> Result<NameResponse> {
        index.name = name::<R>();
        let endpoint: String = Router::index(&index.collection);
        let res: NameResponse = client.post(endpoint, index).await?;
        Ok(res)
    }

*/