use eyre::{eyre, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BlobAnalyzer {
    client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataStorageReference {
    blob_storage: String,
    data_reference: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    rollup: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    commitment: String,
    proof: String,
    size: u64,
    versioned_hash: String,
    data_storage_references: Vec<DataStorageReference>,
    index: u64,
    tx_hash: String,
    block_hash: String,
    block_number: u64,
    block_timestamp: String,
    transaction: Option<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    blobs: Vec<Blob>,
    total_blobs: u64,
}

impl BlobAnalyzer {
    /// Creates a new instance of `BlobAnalyzer` with a default `reqwest::Client`.
    pub fn new() -> Self {
        BlobAnalyzer {
            client: Client::new(),
        }
    }

    /// Queries blobs from the BlobScan API.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or if the JSON deserialization fails.
    pub async fn query_blobs(&self) -> Result<ApiResponse> {
        let url = "https://api.blobscan.com/blobs?sort=desc&startBlock=0&type=canonical";
        
        let response = self.client.get(url).send().await?;
        
        // Check if the response status is successful
        if !response.status().is_success() {
            // Clone the response to read the text without consuming it
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "No error message".to_string());
            dbg!(&error_text);
            return Err(eyre!("Failed to fetch blobs: HTTP {}", status));
        }
        
        // Deserialize the JSON response directly into ApiResponse
        let blobs = serde_json::from_str::<ApiResponse>(&response.text().await?)?;

        Ok(blobs)
    }
}
