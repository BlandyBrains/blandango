use blandango::*;
use serde::{Serialize, Deserialize};

mod model;

use crate::model::{test_config, teardown, setup_collection};


#[derive(Serialize, Deserialize, Debug)]
pub struct SampleData {
    pub body: String
}

// Define a document collection
type SampleDocument = Doc<SampleData>;


#[tokio::test]
async fn test_query(){
    let config: &Config = &test_config();
    let document: Document = Arango::new(config);
    let query: Query = Arango::new(config);

    let _: Properties = setup_collection(&name::<SampleData>()).await;

    // Clear Cache
    query.clear_cache().await.unwrap();

    // Get Cache
    let mut cache_properties: CacheProperties = query.cache_properties().await.unwrap();
    println!("Properties: {:#?}", cache_properties);

    cache_properties.mode = "on".to_owned();
    cache_properties.include_system = !cache_properties.include_system;

    // Update Cache
    let cache_properties: CacheProperties = query.set_cache_properties(&cache_properties).await.unwrap();
    println!("Update Properties: {:#?}", cache_properties);

    // Seed Data
    let data: Vec<SampleData> = vec![
        SampleData{
            body: "David Beckham".to_owned(),
        },
        SampleData{
            body: "Zinedine Zidan".to_owned(),
        },
        SampleData{
            body: "This Guy".to_owned(),
        },
        SampleData{
            body: "POTUS".to_owned(),
        },        
    ];
    let records: Vec<ArangoKeys> = document.insert_many(&data).await.unwrap();
    println!(r"Records: {:#?}", records);

    // Raw Cursor (cached)
    let mut request: CursorRequest = CursorRequest::default();
    request.query = r#"
        FOR R IN sample_data
        RETURN R
    "#.to_owned();
    request.cache = true;

    let results: CursorResponse<Vec<SampleDocument>> = query.cursor(&request).await.unwrap();
    println!("Results: {:#?}", results);

    // Bound Cursor (cached)
    #[derive(Serialize, Debug)]
    pub struct SampleDataBinding {
        #[serde(rename="@collection")]
        pub collection: String,
        pub body: String
    }

    let binding: SampleDataBinding = SampleDataBinding { collection: name::<SampleData>(), body: "POTUS".to_owned()};

    let mut request: BoundCursorRequest<SampleDataBinding> = BoundCursorRequest::new(binding);
    request.query = r#"
        FOR R IN @@collection
        FILTER R.body == @body
        RETURN R
    "#.to_owned();
    request.cache = true;

    let results: CursorResponse<Vec<Doc<SampleData>>> = query.bound_cursor(&request).await.unwrap();
    println!("Results: {:#?}", results);
    assert_eq!(results.result.len(), 1);

    // Cache Entries
    let entries: Vec<Entry> = query.cache_entries().await.unwrap();
    println!("Entries: {:#?}", entries);

    // Parse Query
    let mut parse_query: ParseQuery = ParseQuery { 
        query: "FOR r IN sample_data RETURN r".to_owned()
    };

    let parsed_response: ParseResponse = query.parse(&parse_query).await.unwrap();
    println!("Parsed: {:#?}", parsed_response);

    // Parse Query w/ Bindings
    parse_query.query = "FOR f IN sample_data \nFILTER f.body == @body \nRETURN f".to_owned();
    let parsed_response: ParseResponse = query.parse(&parse_query).await.unwrap();
    println!("Parsed: {:#?}", parsed_response);

    teardown(&name::<SampleData>()).await;
}
