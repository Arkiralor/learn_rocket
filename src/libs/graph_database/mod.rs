#![allow(warnings)]
use futures::stream::*;
use neo4rs::Graph;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use uuid::Uuid;

pub async fn database_conn() -> Arc<Graph> {
    let uri = "127.0.0.1:7687";
    let user = "neo4j";
    let pass = "neo";
    let graph_db: Arc<Graph> = Arc::new(Graph::new(uri, user, pass).await.unwrap());
    return graph_db;
}

// pub static GRAPH_DATABASE: Arc<Graph> = database_conn().await;

pub mod example;
