use futures::stream::*;
use neo4rs::{query, Graph, Node, Txn};
use rocket::tokio;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use uuid::Uuid;

use crate::libs::graph_database::database_conn;

pub async fn example_01() -> Vec<String> {
    let uri = "127.0.0.1:7687";
    let user = "neo4j";
    let pass = "neo";
    let names: Vec<String> = Vec::new();
    let graph = Arc::new(Graph::new(uri, user, pass).await.unwrap());
    for _ in 1..=42 {
        let graph = graph.clone();
        tokio::spawn(async move {
            let mut result = graph
                .execute(query("MATCH (p:Person {name: $name}) RETURN p").param("name", "Mark"))
                .await
                .unwrap();
            while let Ok(Some(row)) = result.next().await {
                let node: Node = row.get("p").unwrap();
                let name: String = node.get("name").unwrap();
                names.push(name);
            }
        });
    }
    return names;
}
pub async fn example_02() -> Result<Txn, E> {
    let uri = "127.0.0.1:7687";
    let user = "neo4j";
    let pass = "neo";
    let graph = Arc::new(Graph::new(uri, user, pass).await.unwrap());
    let mut txn = graph.start_txn().await.unwrap();
    txn.run_queries(vec![
        query("CREATE (p:Person {name: 'mark'})"),
        query("CREATE (p:Person {name: 'jake'})"),
        query("CREATE (p:Person {name: 'luke'})"),
    ])
    .await
    .unwrap();
}
