//! The Avocado prelude provides re-exports of the most commonly used traits
//! and types for convenience, including ones from crates `bson` and `mongodb`.

pub use db::DatabaseExt;
pub use coll::Collection;
pub use doc::Doc;
pub use ops::*;
pub use literal::{ Order, BsonType };
pub use error::Error as AvocadoError;
pub use error::Result as AvocadoResult;
pub use bson::{ Bson, Document, oid::ObjectId, doc, bson };
pub use mongodb::{
    Client, ThreadedClient,
    db::Database,
    coll::options::{ IndexModel, IndexOptions }
};
