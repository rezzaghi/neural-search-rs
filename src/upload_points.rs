use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    Condition, CreateCollection, Filter, PointId, SearchPoints, Struct, UpdateCollection, Vector,
    VectorParams, VectorsConfig,
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
async fn upload_points() -> Result<()> {
       let config = QdrantClientConfig::from_url("http://localhost:6334");
       let client = QdrantClient::new(Some(config))?;

       client.delete_collection("startups").await?;

       client.create_collection(&CreateCollection {
           collection_name: "startups".into(),
           vectors_config: Some(VectorsConfig {
               config: Some(Config::Params(VectorParams {
                   size: 384,
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

    // load vectors
    let bytes = std::fs::read("src/vectors.npy")?;
    let npy = npyz::NpyFile::new(&bytes[..])?;
    let mut points: Vec<PointStruct> = vec![];

    let row_length = 384;

    for (i, (row, line)) in npy
        .into_vec::<f32>()
        .unwrap()
        .chunks(row_length)
        .zip(reader.lines())
        .enumerate()
    {
        let line = line?;

        let vector = qdrant_client::qdrant::Vector { data: row.into() };
        let vectors_options = qdrant_client::qdrant::vectors::VectorsOptions::Vector(vector);
        let new_value = qdrant_client::qdrant::Vectors {
            vectors_options: Some(vectors_options),
        };

        let mut payload = Payload::new();
        let startup: Startup = serde_json::from_str(&line).unwrap();
        let startup_json: serde_json::Value = serde_json::to_value(startup).unwrap();
        payload.insert(i, startup_json);

        points.push(PointStruct {
            id: Some((i as u64 + 1).into()),
            payload: payload.into(),
            vectors: Some(new_value),
        });
    }

    client
        .upsert_points_blocking("startups", points, None)
        .await?;
    println!("{:?}", client.collection_info("startups").await?);

    Ok(())
}