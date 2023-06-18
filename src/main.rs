use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    Condition, CreateCollection, Filter, SearchPoints, VectorParams, VectorsConfig, UpdateCollection,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Serialize, Deserialize, Debug)]
struct Startup {
    name: String,
    images: String,
    alt: String,
    description: String,
    link: String,
    city: String,
}

#[tokio::main]
async fn main() -> Result<()> {
       let config = QdrantClientConfig::from_url("http://localhost:6334");
       let client = QdrantClient::new(Some(config))?;
       client.create_collection(&CreateCollection {
           collection_name: "startups".into(),
           vectors_config: Some(VectorsConfig {
               config: Some(Config::Params(VectorParams {
                   size: 768,
                   distance: Distance::Cosine.into(),
                   ..Default::default()
               })),
           }),
           ..Default::default()
       })
       .await?;

    // map startup payload data
    let file = File::open("src/startup.json")?;
    let reader = io::BufReader::new(file);

    let mut startups = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let startup: Startup = serde_json::from_str(&line).unwrap();
        startups.push(startup);
    }

    // load vectors
    let bytes = std::fs::read("src/vectors.npy")?;
    let npy = npyz::NpyFile::new(&bytes[..])?;

    Ok(())
}
