use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{Map, Value};
use crate::{Client, Result, Router, QueryRouter, IdResponse, Empty, Response};


pub struct Query{
    pub client: Client
}

impl Query {
    // Query API operations

    // User-defined functions

    /*
    // GET /_api/aqlfunction    Return registered AQL user functions

    // POST /_api/aqlfunction   Create AQL user function

    // DELETE /_api/aqlfunction/{}  Remove existing AQL user function
    */

    // Cursor
    
    /// Create Bound Cursor
    pub async fn bound_cursor<B: Serialize + Send + Sync, R: DeserializeOwned + Send + Sync>(&self, request: &BoundCursorRequest<B>) -> Result<CursorResponse<R>>{
        let response: CursorResponse<R> = self.client.post( Router::cursor(&request.id), request).await?;
        Ok(response)
    }

    /// Create Cursor
    pub async fn cursor<R: DeserializeOwned + Send + Sync>(&self, request: &CursorRequest) -> Result<CursorResponse<R>>{
        let response: CursorResponse<R> = self.client.post(Router::cursor(&request.id), request).await?;
        Ok(response)
    }

    /// Delete Cursor
    pub async fn delete_cursor(&self, cursor_id: String) -> Result<IdResponse>{
        let response: IdResponse = self.client.delete(Router::cursor(&Some(cursor_id))).await?;
        Ok(response)
    }
    
    // Caching

    /// Clears results in the AQL query results cache 
    pub async fn clear_cache(&self) -> Result<()>{
        let _: Empty = self.client.delete(Router::cache()).await?;
        Ok(())
    }

    /// Returns the currently cached query results
    pub async fn cache_entries(&self) -> Result<Vec<Entry>>{
        let response: Vec<Entry> = self.client.get(Router::cache_entries()).await?;
        Ok(response)
    }

    /// Returns the global properties for the AQL query results cache
    pub async fn cache_properties(&self) -> Result<CacheProperties>{
        let response: CacheProperties = self.client.get(Router::cache_properties()).await?;
        Ok(response)
    }

    /// Globally adjusts the AQL query results cache properties
    pub async fn set_cache_properties(&self, properties: &CacheProperties) -> Result<CacheProperties>{
        let response: CacheProperties = self.client.put(Router::cache_properties(), properties).await?;
        Ok(response)
    }

    // Query

    /// Explain an AQL query
    pub async fn bound_explain<B: Serialize + Send + Sync, R: DeserializeOwned + Send + Sync>(&self, query: &BoundExplain<B>) -> Result<ExplainResponse> {
        let response: Response<ExplainResponse> = self.client.post(Router::explain(), query).await?;
        Ok(response.result)
    }

    /// Explain an AQL query
    pub async fn explain(&self, query: &Explain) -> Result<ExplainResponse> {
        let response: Response<ExplainResponse> = self.client.post(Router::explain(), query).await?;
        Ok(response.result)
    }

    /// Parse an AQL query
    pub async fn parse(&self, query: &ParseQuery) -> Result<ParseResponse> {
        let response: ParseResponse = self.client.post(Router::query(), query).await?;
        Ok(response)
    }

    /// Returns the currently running AQL queries
    pub async fn running(&self) -> Result<Vec<RunningQuery>> {
        let response: Vec<RunningQuery> = self.client.get(Router::running()).await?;
        Ok(response)
    }

    /// Clears the list of slow AQL queries
    pub async fn clear_slow(&self) -> Result<()> {
        let response: () = self.client.delete(Router::slow()).await?;
        Ok(response)
    }

    // Returns the list of slow AQL queries
    pub async fn slow(&self) -> Result<Vec<RunningQuery>> {
        let response: Vec<RunningQuery> = self.client.get(Router::slow()).await?;
        Ok(response)
    }

    // Kills a running AQL query
    pub async fn kill(&self, query_id: &str) -> Result<()> {
        let response: () = self.client.delete(Router::kill(query_id)).await?;
        Ok(response)
    }
    // GET /_api/query/properties   Returns the properties for the AQL query tracking

    // PUT /_api/query/properties   Changes the properties for the AQL query tracking

    // GET /_api_query/rules    Returns all AQL optimizer rules    
}


#[derive(Deserialize, Debug)]
pub struct ParseResponse {
    pub error: bool,
    pub code: u32,
    pub parsed: bool,
    pub collections: Vec<String>,
    #[serde(rename="bindVars")]
    pub bind: Vec<String>,
    pub ast: Vec<Map<String, Value>> 
}

#[derive(Serialize, Debug)]
pub struct ParseQuery {
    pub query: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExplainResponse {
    pub plans: Vec<Plan>,
    pub warnings: Vec<Warning>,
    pub stats: Stats
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExplainOptions {
    pub all_plans: bool,
    pub max_num_of_plans: u32,
    pub optimizer: Optimizer
}

#[derive(Serialize, Debug)]
pub struct BoundExplain<T>
where T: Serialize + Send + Sync {
    pub query: String,
    #[serde(rename="bindVars")]
    pub bind: T,
    pub options: Option<ExplainOptions>
}

#[derive(Serialize, Debug)]
pub struct Explain{
    pub query: String,
    pub bind: Option<Map<String, Value>>,
    pub options: Option<ExplainOptions>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CacheProperties {
    pub mode: String,
    pub max_results: u64,
    pub max_results_size: u64,
    pub max_entry_size: u64,
    pub include_system: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub hash: String,
    pub query: String,
    #[serde(rename="bindVars")]
    pub bind: Option<Map<String, Value>>,
    pub size: u64,
    pub results: u64,
    pub started: String,
    pub hits: u32,
    pub run_time: f64,
    pub data_sources: Vec<String>
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    pub rules: String
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub full_count: Option<bool>,

    pub fill_block_cache: Option<bool>,

    pub max_number_of_plans: Option<u32>,
    pub max_nodes_per_callstack: Option<u32>,
    /// default = 10
    pub max_warning_count: Option<u8>, 
    pub fail_on_warning: Option<bool>,
    pub stream: Option<bool>,
    /// Experimental features | Default = 128MB
    pub spill_over_threshold_memory_usage: Option<u64>,
    /// Default = 5,000,000
    pub spill_over_threshold_num_rows: Option<u64>,

    pub optimizer: Option<Optimizer>,
    /// Provides the extra.profile, extra.nodes, and extra.plan attributes
    pub profile: Option<u8>,

    /// Enterprise Edition
    pub satelite_sync_wait: Option<f32>,

    /// Default = 0.0
    pub max_runtime: Option<u64>,

    pub max_transaction_size: Option<u64>,

    pub intermediate_commit_size: Option<u64>,

    pub intermediate_commit_count: Option<u64>,

    pub skip_inaccessible_collections: Option<bool>,

    pub allow_dirty_reads: Option<bool>
}


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoundCursorRequest<T> 
where T: Serialize + Send + Sync {
    pub id: Option<String>,
    pub query: String,
    pub count: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>, // default = 30 seconds
    /// Determines whether to use cached results
    pub cache: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<u64>,

    #[serde(rename = "bindVars")]
    pub bind: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>
}

impl<T> BoundCursorRequest<T>
where 
    T: Serialize + Send + Sync {
    pub fn new(bind: T) -> Self {
        Self { 
            id: None,
            query: "".to_owned(), 
            count: true, 
            batch_size: None, 
            ttl: None,
            cache: false,
            memory_limit: None,
            options: None,
            bind,
        }
    }
    // todo - additional helpers for paging?
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CursorRequest {
    pub id: Option<String>,
    pub query: String,
    pub count: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>, // default = 30 seconds
    /// Determines whether to use cached results
    pub cache: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<u64>,
    #[serde(rename = "bindVars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>
}

impl Default for CursorRequest {
    fn default() -> Self {
        Self { 
            id: None,
            query: "".to_owned(), 
            count: true, 
            batch_size: None, 
            ttl: None,
            cache: false,
            memory_limit: None,
            bind: None, 
            options: None,
        }
    }
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Warning {
    pub code: u64,
    pub message: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub calls: u32,
    pub items: u64,
    pub runtime: u64
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub writes_executed: u32,
    pub writes_ignored: u32,
    pub scanned_full: u32,
    pub scanned_index: u32,
    pub cursors_created: u32,
    pub cursors_rearmed: u32,
    pub cache_hits: u32,
    pub cache_misses: u32,
    pub filtered: u32,
    pub http_requests: u32,
    pub execution_time: f64,
    pub peak_memory_usage: f64,
    pub full_count: Option<u64>,
    pub nodes: Option<Node>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanCollection{
    pub name: String,
    pub r#type: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Plan{
    pub nodes: Option<Map<String, Value>>,
    pub rules: String,
    pub collections: Vec<PlanCollection>,
    pub variables: Option<Map<String, Value>>,
    pub estimated_cost: u64,
    pub estimated_nr_items: u64,
    pub is_modification_query: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extra {
    pub warnings: Vec<Warning>,
    pub stats: Option<Stats>,
    pub profile: Option<String>,
    pub plan: Option<Plan>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CursorResponse<T> {
    // id is used for pagination.
    // and will only be present when has_more=True
    pub id: Option<String>,

    pub error: bool,
    pub code: u8,

    pub result: T,

    #[serde(rename = "hasMore")]
    pub has_more: bool,
    pub count: Option<u32>,
    
    pub extra: Option<Extra>,
    pub cached: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningQuery {
    pub id: String,
    pub database: String,
    pub user: String,
    pub query: String,
    #[serde(rename="bindVars")]
    pub bind: Vec<String>,
    pub started: String,
    pub run_time: f64,
    /// States:
    /// - initializing
    /// - parsing
    /// - optimizing ast
    /// - loading collections
    /// - instantiating plan
    /// - optimizing plan
    /// - executing
    /// - finalizing
    /// - finished
    /// - killed
    /// - invalid
    pub state: String,
    pub stream: bool
}
