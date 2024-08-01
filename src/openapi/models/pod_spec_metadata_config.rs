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

/// PodSpecMetadataConfig : Configuration for the behavior of Pinecone's internal metadata index. By default, all metadata is indexed; when `metadata_config` is present, only specified metadata fields are indexed. These configurations are only valid for use with pod-based indexes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PodSpecMetadataConfig {
    /// By default, all metadata is indexed; to change this behavior, use this property to specify an array of metadata fields that should be indexed.
    #[serde(rename = "indexed", skip_serializing_if = "Option::is_none")]
    pub indexed: Option<Vec<String>>,
}

impl PodSpecMetadataConfig {
    /// Configuration for the behavior of Pinecone's internal metadata index. By default, all metadata is indexed; when `metadata_config` is present, only specified metadata fields are indexed. These configurations are only valid for use with pod-based indexes.
    pub fn new() -> PodSpecMetadataConfig {
        PodSpecMetadataConfig {
            indexed: None,
        }
    }
}
