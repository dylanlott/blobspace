use eyre;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

/// Analyzes blobspace data from Blobscan
#[derive(Debug)]
pub struct BlobAnalyzer {
    client: reqwest::Client,
}

/// Blobscan's representation of a Blob

#[derive(Serialize, Deserialize, Debug)]
pub struct DataStorageReference {
    blobStorage: String,
    dataReference: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    rollup: Option<String>,  // Optional field, as it's not always present
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blob {
    commitment: String,
    proof: String,
    size: u64,
    versionedHash: String,
    dataStorageReferences: Vec<DataStorageReference>,
    index: u64,
    txHash: String,
    blockHash: String,
    blockNumber: u64,
    blockTimestamp: String,
    transaction: Option<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    blobs: Vec<Blob>,
    totalBlobs: u64,
}


/// Blobspace Analyzer
impl BlobAnalyzer {
    pub fn new() -> Self {
        BlobAnalyzer {
            client: reqwest::Client::new(),
        }
    }

    /// Fetch the given block range of blobs
    pub async fn query_blobs(&self) -> eyre::Result<ApiResponse> {
        let url = format!( "https://api.blobscan.com/blobs?sort=desc&startBlock=0&type=canonical");
        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            dbg!(response.text().await?);
            return Err(eyre::eyre!("Failed to fetch blobs"));
        }
        let blobs = serde_json::from_str::<ApiResponse>(&response.text().await?)?;
        Ok(blobs)
    }
}
