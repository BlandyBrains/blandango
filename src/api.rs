use std::fmt::{self};
use serde::Deserialize;

pub (crate) enum API {
    Collection(Collection),
    Document(Document),
    Graph(Graph),
    Index(Index),
    Query(Query), 
    Database(Database)
}

pub (crate) enum Collection {
    Base,
    Properties,
    Count,
    Figures,
    ResponsibleShard,
    Shards,
    Revision,
    Checksum,
    Truncate,
    LoadIndexIntoMemory,
    Rename,
    RecalculateCount,
    Compact
}

pub (crate) enum Document {
    Base
}

pub (crate) enum Database {
    Base,
    Current,
    User
}

pub (crate) enum Query {
    Cursor,
    Function,
    Explain,
    Query,
    Cache,
    Current,
    Properties,
    Rules,
    Entries,
    Slow
}
pub (crate) enum Graph {
    Base,
    Edge,
}

pub (crate) enum Index {
    Base
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::Index(route) => {
                match route{
                    Index::Base => "_api/index"
                }
            },
            API::Collection(route) => {
                match route {
                    Collection::Base => "_api/collection",
                    Collection::Properties => "properties",
                    Collection::Count => "count",
                    Collection::Figures => "figures",
                    Collection::ResponsibleShard => "responsibleShard",
                    Collection::Shards => "shards",
                    Collection::Revision => "revision",
                    Collection::Checksum => "checksum",
                    Collection::Truncate => "truncate",
                    Collection::LoadIndexIntoMemory => "loadIndexesIntoMemory",
                    Collection::Rename => "rename",
                    Collection::RecalculateCount => "recalculateCount",
                    Collection::Compact => "compact"
                }
            },
            API::Database(route) => {
                match route {
                    Database::Base => "_api/database",
                    Database::Current => "current",
                    Database::User => "user"
                }
            },
            API::Document(route) => {
                match route {
                    Document::Base => "_api/document"
                }
            },
            API::Query(route) => {
                match route {
                    Query::Function => "_api/aqlfunction",
                    Query::Cursor => "_api/cursor",
                    Query::Explain => "_api/explain",
                    Query::Query => "_api/query",
                    Query::Cache => "_api/query-cache",
                    Query::Entries => "entries",
                    Query::Properties => "properties",
                    Query::Current => "current",
                    Query::Rules => "rules",
                    Query::Slow => "slow",
                }
            },
            API::Graph(route) => {
                match route {
                    Graph::Base => "_api/gharial",
                    Graph::Edge => "edge"
                }
            }
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct ApiError{
    pub code: i32,
    pub error: bool,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "errorNum")]
    pub error_num: i32
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
            "ApiError({}, {}, {}, {})", 
            self.code, 
            self.error,
            self.error_message,
            self.error_num
        )
    }
}

impl std::error::Error for ApiError{}