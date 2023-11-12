// use async_trait::async_trait;
// use serde::Serialize;

// use crate::{document::*, Result, QueryBound, QueryResponse, Client, DatabaseError};


// const SAMPLE: &str = 
// r#"
// FOR loc in location
// FILTER loc._key == @location_key
//     FOR equip in equipment
//     FILTER equip.location_id == loc._id &&
//         (@active == null || equip.active == @active) && 
//         (@location_key == null || loc._key == @location_key) && 
//         (@equipment_key == null || equip._key == @equipment_key)
//     FOR template IN equipment_template
//     FILTER template._id == equip.template_id
//     RETURN MERGE(equip, {template: template})
// "#;

// fn filter_key(key: String) -> String {
//     format!("FILTER d._key == \"{}\"", key)
// }

// /*
//     A Provider is a Model that provides relational
//     abilities between another model.
// */
// #[async_trait]
// pub trait Provider<T: 'static + Document>
//     where Self:Document {

//     // collection header: `FOR x in collection`
//     fn root() -> String {
//         format!("FOR d in {0}", Self::collection_name())
//     }

//     fn collection_id() -> String {
//         format!("{0}_id", Self::collection_name())
//     }

//     // collection header: `FOR x in collection`
//     async fn add<M: Serialize + Send + Sync>(client: &Client, key: String, model: M) -> Result<T> {
//         let q: String = format!("
//             {0}
//             {1}
//             INSERT MERGE(@model, {{ {2}_id: d._id }}) INTO {3}
//             RETURN MERGE(NEW, {{ {2}: d }})
//             ", 
//             Self::root(), 
//             filter_key(key),
//             Self::collection_name(),
//             <T as Document>::collection_name(),
//         ).to_owned();

//         let query: QueryBound<ModelParams<M>> = QueryBound::new(q.clone(), ModelParams{model});

//         let mut results: QueryResponse<Vec<T>> = T::bounded_query(client, &query).await?;
//         if results.result.len() > 0 {
//             return Ok(results.result.pop().unwrap());
//         }
//         println!("query: {}", &q);
//         return Err(DatabaseError::insertion_error("failed to create related document".to_owned()));
//     }

//     async fn edit<M: Serialize + Send + Sync>(client: &Client, parent_key: String, child_key: String, model: M) -> Result<T> {
//         let q: String = format!("
//             {0}
//             {1}
//             FOR c IN {3}
//             FILTER c.{2}_id == d._id && c._key == \"{4}\"
//             UPDATE c._key WITH MERGE(@model, {{ {2}_id: d._id }}) IN {3}
//             RETURN MERGE(NEW, {{ {2}: d }})
//             ", 
//             Self::root(), 
//             filter_key(parent_key),
//             Self::collection_name(),
//             <T as Document>::collection_name(),
//             child_key
//         ).to_owned();

//         let query: QueryBound<ModelParams<M>> = QueryBound::new(q, ModelParams{model});

//         let mut results: QueryResponse<Vec<T>> = T::bounded_query(client, &query).await?;
//         if results.result.len() > 0 {
//             return Ok(results.result.pop().unwrap());
//         }
//         return Err(DatabaseError::insertion_error("failed to edit related document".to_owned()));
//     }       

//     // async fn find(client: &Client) -> Result<T> {
//     //     let q: String = format!("            
//     //     FOR co in company
//     //     FILTER co._key == @company_key
//     //     FOR loc in location
//     //     FILTER loc.company_id == co._id &&
//     //         (@active == null || loc.active == @active) && 
//     //         (@location_key == null || loc._key == @location_key)
//     //     RETURN MERGE(loc, {company: co})");
//     // }
// }


// #[derive(Serialize, Clone)]
// pub struct ModelParams<T>{
//     pub model: T
// }
