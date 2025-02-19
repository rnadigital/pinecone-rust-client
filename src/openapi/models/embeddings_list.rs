/*
 * Pinecone Control Plane API
 *
 * Pinecone is a vector database that makes it easy to search and retrieve billions of high-dimensional vectors.
 *
 * The version of the OpenAPI document: 2024-07
 * Contact: support@pinecone.io
 * Generated by: https://openapi-generator.tech
 */

use crate::openapi::models;
use serde::{Deserialize, Serialize};

/// EmbeddingsList : Embeddings generated for the input
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingsList {
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::Embedding>>,
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<models::EmbeddingsListUsage>>,
}

impl EmbeddingsList {
    /// Embeddings generated for the input
    pub fn new() -> EmbeddingsList {
        EmbeddingsList {
            model: None,
            data: None,
            usage: None,
        }
    }
}

