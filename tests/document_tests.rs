use blandango::*;
use serde::{Serialize, Deserialize};
mod model;

use crate::model::{test_config, setup_collection, teardown, setup_edge_collection};


#[derive(Serialize, Deserialize, Debug)]
pub struct SampleData {
    pub body: String
}

// Define a document collection
type SampleDocument = Doc<SampleData>;


#[derive(Serialize, Deserialize, Debug)]
pub struct FromData {
    pub body: String
}

// Define a document collection
type FromDocument = Doc<FromData>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ToData {
    pub body: String
}

// Define a document collection
type ToDocument = Doc<ToData>;


#[derive(Serialize, Deserialize, Debug)]
pub struct EdgeData {
    pub body: String
}

// Define a document collection
type EdgeDocument = Edge<EdgeData>;

#[tokio::test]
async fn test_document(){
    let config: &Config = &test_config();
    let document: Document = Arango::new(config);

    let data: &str = r#"
        {
            "_id": "test_id",
            "_key": "test_key",
            "_rev": "test_rev",
            "body": "the inner data"
        }
    "#;

    let _: Properties = setup_collection(&name::<SampleData>()).await;

    // deserialize
    let result: SampleDocument = serde_json::from_str(data).unwrap();

    // create 
    let mut message: SampleDocument = document.insert(&result.record).await.unwrap();
    println!("created message {:#?}", message);

    // update
    message.record.body = "updated data".to_owned();

    document.update(&message).await.unwrap();
    println!("updated message {:#?}", message);

    // replace
    message.record.body = "replaced data".to_owned();
    document.replace(&message).await.unwrap();
    println!("replaced message {:#?}", message);

    // get
    let msg: SampleDocument = document.read(&message.keys.key).await.unwrap();
    println!("fetched message {:#?}", msg);

    // delete
    let _ = document.destroy(&message).await;
    println!("deleted message");

    teardown(&name::<SampleData>()).await;
}


#[tokio::test]
async fn test_edge(){
    let config: &Config = &test_config();
    let document: Document = Arango::new(config);

    let _: Properties = setup_collection(&name::<FromData>()).await;
    let _: Properties = setup_collection(&name::<ToData>()).await;
    let _: Properties = setup_edge_collection(&name::<EdgeDocument>()).await;

    let data: &str = r#"
        {
            "body": "sample body"
        }
    "#;

    // deserialize
    let from: FromData = serde_json::from_str(data).unwrap();
    let to: ToData = serde_json::from_str(data).unwrap();

    // create 
    let from_doc: FromDocument = document.insert(&from).await.unwrap();
    let to_doc: ToDocument = document.insert(&to).await.unwrap();

    let data: EdgeData = serde_json::from_str(data).unwrap();
    let nat_edge: EdgeDocument = Edge { 
        link: ArangoEdgeKeys { to: to_doc.keys.id, from: from_doc.keys.id }, 
        record: data 
    };
    let real_edge: Doc<Edge<EdgeData>> = document.insert(&nat_edge).await.unwrap();
    
    /*
    EDGE structure - link keys are nested within the Doc<T> structure.

    {
        keys: ArangoKeys {
            id: "edge/8963380",
            key: "8963380",
            rev: "_gyOmZey---",
        },
        record: Edge {
            link: ArangoEdgeKeys {
                to: "to_data/8963378",
                from: "from_data/8963376",
            },
            record: EdgeData {
                body: "sample body",
            },
        },
        modified_on: Some(
            1697655582221,
        ),
        created_on: Some(
            1697655582221,
        ),
    }
    */
    println!("EDGE {:#?}", real_edge);

    teardown(&name::<FromData>()).await;
    teardown(&name::<ToData>()).await;
    teardown(&name::<EdgeData>()).await;
}

