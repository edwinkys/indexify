use super::{CreateIndexParams, IndexDistance, VectorChunk, VectorDb};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use oasysdb::prelude::{Collection, Config, Database, Distance, Metadata, Record};
use serde_json::Value;
use std::fmt;

pub struct OasysDb {
    db: Database,
}

impl fmt::Debug for OasysDb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OasysDb")
    }
}

impl OasysDb {
    /// Open OasysDB at the given path.
    pub fn open(path: &str) -> Result<Self> {
        let db = Database::open(path).map_err(|e| anyhow!("Failed to open OasysDB: {e:?}"))?;
        Ok(Self { db })
    }
}

#[async_trait]
impl VectorDb for OasysDb {
    /// Creates a new vector index with the specified configuration.
    #[tracing::instrument]
    async fn create_index(&self, index: CreateIndexParams) -> Result<()> {
        // Convert the distance metric to OasysDB's Distance enum.
        let distance = match index.distance {
            IndexDistance::Cosine => Distance::Cosine,
            IndexDistance::Dot => Distance::Dot,
            IndexDistance::Euclidean => Distance::Euclidean,
        };

        // Create OasysDB collection configuration.
        let config = Config::default();
        config.distance = distance;

        // Create OasysDB collection.
        let collection = Collection::new(&config);
        collection.set_dimension(index.vector_dim as usize);

        // Add the collection to the database.
        self.db
            .save_collection(&index.vectordb_index_name, &collection)
            .map_err(|e| anyhow!("{e:?}"))?;

        Ok(())
    }

    /// Adds a vector embedding to the specified index, along with associated
    /// attributes.
    #[tracing::instrument]
    async fn add_embedding(&self, index: &str, chunks: Vec<VectorChunk>) -> Result<()> {
        // Get the collection from the database.
        let collection = self
            .db
            .get_collection(index)
            .map_err(|e| anyhow!("{e:?}"))?;

        // Convert VectorChunk to OasysDB Records.
        unimplemented!();

        Ok(())
    }

    /// Deletes the specified vector index from the vector database.
    #[tracing::instrument]
    async fn drop_index(&self, index: &str) -> Result<()> {
        self.db
            .delete_collection(index)
            .map_err(|e| anyhow!("{e:?}"))?;
        Ok(())
    }

    /// Returns the number of vectors in the specified index.
    #[tracing::instrument]
    async fn num_vectors(&self, index: &str) -> Result<u64> {
        let collection = self
            .db
            .get_collection(index)
            .map_err(|e| anyhow!("{e:?}"))?;
        Ok(collection.len() as u64)
    }

    fn name(&self) -> String {
        "oasysdb".into()
    }
}
