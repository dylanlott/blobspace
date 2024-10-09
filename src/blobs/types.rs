use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataStorageReference {
    pub blob_storage: String,
    pub data_reference: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub hash: Option<String>,
    pub blobs: Vec<Blob>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub max_fee_per_blob_gas: Option<String>,
    pub blob_as_calldata_gas_used: Option<String>,
    pub blob_gas_used: Option<String>,
    pub category: Option<String>,
    pub rollup: Option<String>,
    pub index: Option<u64>,
    pub blob_as_calldata_gas_fee: Option<String>,
    pub blob_gas_base_fee: Option<String>,
    pub blob_gas_max_fee: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    pub commitment: Option<String>,
    pub proof: Option<String>,
    pub size: Option<u64>,
    pub versioned_hash: Option<String>,
    pub data_storage_references: Option<Vec<DataStorageReference>>,
    pub index: Option<u64>,
    pub tx_hash: Option<String>,
    pub block_hash: Option<String>,
    pub block_number: Option<u64>,
    pub block_timestamp: Option<String>,
    pub transaction: Option<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiBlobResponse {
    pub blobs: Vec<Blob>,
    pub total_blobs: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiBlockResponse {
    pub blocks: Vec<Block>,
    pub total_blocks: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub hash: String,
    pub number: u64,
    pub timestamp: String,
    pub slot: u64,
    pub blob_gas_used: String,
    pub blob_as_calldata_gas_used: String,
    pub blob_gas_price: String,
    pub excess_blob_gas: String,
    pub transactions: Vec<Transaction>,
}
