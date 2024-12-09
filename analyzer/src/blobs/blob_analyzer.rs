use eyre::{eyre, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::{ApiBlobResponse, ApiBlockResponse, Block};

#[derive(Debug)]
pub struct BlobAnalyzer {
    client: Client,
}

impl BlobAnalyzer {
    /// Creates a new instance of `BlobAnalyzer` with a default `reqwest::Client`.
    pub fn new() -> Self {
        BlobAnalyzer {
            client: Client::new(),
        }
    }

    /// Queries blocks from the BlobScan API.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or if the JSON deserialization fails.
    pub async fn query_blocks(&self) -> Result<Vec<Block>> {
        let url = "https://api.blobscan.com/blocks?sort=desc&type=canonical";
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            // Clone the response to read the text without consuming it
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "No error message".to_string());

            dbg!(&error_text);
            return Err(eyre!("Failed to fetch blocks: HTTP {}", status));
        }

        let block_response = serde_json::from_str::<ApiBlockResponse>(&response.text().await?)?;
        Ok(block_response.blocks)
    }

    /// Queries blobs from the BlobScan API.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or if the JSON deserialization fails.
    pub async fn query_blobs(&self, start_block: u64, end_block: u64) -> Result<ApiBlobResponse> {
        let url = format!(
            "https://api.blobscan.com/blobs?sort=desc&startBlock={}&endBlock={}&type=canonical",
            start_block, end_block
        );
        let response = self.client.get(url).send().await?;

        // Check if the response status is successful
        if !response.status().is_success() {
            // Clone the response to read the text without consuming it
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "No error message".to_string());
            dbg!(&error_text);
            return Err(eyre!("Failed to fetch blobs: HTTP {}", status));
        }

        // dbg!(response.text().await?);

        // Deserialize the JSON response directly into ApiResponse
        let blobs = serde_json::from_str::<ApiBlobResponse>(&response.text().await?)?;
        dbg!(&blobs);

        Ok(blobs)
    }

    /// Stats returns statistics about the blobs in the database, including average number of blobs per block,
    /// average number of transactions per block, average number of data storage references per blob, as well
    /// as statistics about the address that built the blobs.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or if the JSON deserialization fails.
    pub async fn stats(&self, blocks: Vec<Block>) -> Result<Stats> {
        dbg!(blocks.len());

        // Total count of blobs encountered in block range
        let mut total_blobs: u64 = 0;

        // Loop through the given block range
        for block in blocks.iter() {
            let block = block.clone();
            dbg!(block.number);
            dbg!(block.hash);

            block.transactions.iter().for_each(|tx| {
                let blob_count = tx.blobs.len() as u64;
                total_blobs += blob_count;
                dbg!(blob_count);
            });
        }

        Ok(Stats {
            total_blobs,
            block_map: HashMap::new(),
            rollup_map: HashMap::new(),
        })
    }

    /// Query details on a single block
    pub async fn query_block(&self, block_numhash: String) -> Result<Block> {
        let url = format!(
            "https://api.blobscan.com/blocks/{}?type=canonical",
            block_numhash
        );
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            // Clone the response to read the text without consuming it
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "No error message".to_string());

            dbg!(&error_text);
            return Err(eyre!("Failed to fetch block: HTTP {}", status));
        }

        let block_response = serde_json::from_str::<Block>(&response.text().await?)?;

        Ok(block_response.clone())
    }
}

/// Stats represents statistics about the blobs in the database.
#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    /// Total blobs in the range
    pub total_blobs: u64,
    /// Maps block number to number of blobs in that block.
    pub block_map: HashMap<u64, u64>,
    /// Maps rollup to number of transactions in the blobs.
    pub rollup_map: HashMap<String, u64>,
}
