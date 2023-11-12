use blandango::*;
use convert_case::{Casing, Case};
use crate::model::test_config;

mod model;


#[test]
fn test_string_parse() {
    let trial: &str = "Some::Cool::Object::TestObjectCollection";
    let reduced: &str = &trial.split("::").last().to_owned().unwrap().to_owned();
    println!("reduced: {}", reduced);

    // regex conversion
    println!("formatted {}", reduced.to_case(Case::Snake));
}


#[tokio::test]
async fn test_collection(){
    pub struct IntegrationTest {}

    let config: &Config = &test_config();

    let db: Database = Arango::new(config);

    let mut collection: Collection = db.collection(&name::<IntegrationTest>());

    let new_collection: NewCollection = NewCollection::default_document_collection(name::<IntegrationTest>());
    let properties: Properties = db.new_collection(&new_collection).await.unwrap();
    println!("Collection: {:#?}", properties);

    let cols: Vec<Information> = collection.read().await.unwrap();
    println!("Collections: {:#?}", cols);

    let col: Information = collection.information().await.unwrap();
    println!("Collection: {:#?}", col);

    let col: Checksum = collection.checksum().await.unwrap();
    println!("Checksum: {:#?}", col);

    let col: Information = collection.compact().await.unwrap();
    println!("Compact: {:#?}", col);

    let count: Count = collection.count().await.unwrap();
    println!("Count: {:#?}", count);

    let summary: Summary = collection.figures().await.unwrap();
    println!("Summary: {:#?}", summary);

    let properties: Properties = collection.properties().await.unwrap();
    println!("Properties: {:#?}", properties);

    let revision: Revision = collection.revision().await.unwrap();
    println!("Revision: {:#?}", revision);    

    let loaded_indexes: bool = collection.load_indexes().await.unwrap();
    println!("Indexes Loaded: {:#?}", loaded_indexes); 

    let truncate: Information = collection.truncate().await.unwrap();
    println!("Truncate: {:#?}", truncate);

    let upd_properties: PropertiesUpdate = PropertiesUpdate { 
        wait_for_sync: true, 
        cache_enabled: false, 
        computed_values: None, 
        replication_factor: 1, 
        write_concern: 1
    };
    let props: Properties = collection.update_properties(&upd_properties).await.unwrap();
    println!("Updated Properties: {:#?}", props);

    let recalculated: bool = collection.recalculate_count().await.unwrap();
    println!("Recalculated Count: {:#?}", recalculated); 

    let renamed_collection: Information = collection.rename("sample_data_renamed").await.unwrap();
    println!("Renamed Collection: {:#?}", renamed_collection);

    collection.drop().await.unwrap();

}
