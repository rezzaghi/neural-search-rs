use anyhow::Result;
use qdrant_client::prelude::{QdrantClientConfig, QdrantClient};
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};

#[tokio::main]
async fn main() -> Result<()> {
       let config = QdrantClientConfig::from_url("http://localhost:6334");
       let client = QdrantClient::new(Some(config))?;

       let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2).create_model()?;

    Ok(())
}
