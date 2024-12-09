use reth::{revm::primitives::alloy_primitives::TxHash, rpc::types::BlobTransactionSidecar};

pub struct Blob {
    key: TxHash,
    value: BlobTransactionSidecar,
}

pub trait BlobStore {
    fn save(&self, blob: Blob) -> eyre::Result<()>;
}
