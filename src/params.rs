use serde::Serialize;



#[derive(Serialize)]
pub struct DocumentQueryParams{
    #[serde(rename = "waitForSync")]
    pub wait_for_sync: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnNew")]
    pub return_new: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnOld")]
    pub return_old: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,    

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "overwriteMode")]
    pub overwrite_method: Option<String>, // "ignore", "replace", "update", "conflict"    

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keepNull")]
    pub keep_null: Option<bool>,    

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mergeObjects")]
    pub merge_objects: Option<bool>,    

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refillIndexCaches")]
    pub refill_index_cache: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreRevs")]
    pub ignore_revs: Option<bool>
}

impl Default for DocumentQueryParams {
    fn default() -> Self {
        Self { 
            // default behavior enforces new document to be returned
            // DocumentResponse is tightly coupled to these settings.
            // Not all of them are currently supported, stick with the default for now.
            return_new: Some(true),
            return_old: None,
            silent: None,
            wait_for_sync: true,
            overwrite: None,
            overwrite_method: None,
            keep_null: None,
            merge_objects: None,
            refill_index_cache: None,
            ignore_revs: None
        }
    }
}

#[derive(Serialize)]
pub struct EdgeQueryParams{
    pub vertex: String,
    pub direction: Option<String>
}

impl Default for EdgeQueryParams {
    fn default() -> Self {
        Self { 
            vertex: "".to_owned(),
            direction: None,
        }
    }
}