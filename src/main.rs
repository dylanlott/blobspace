use blobspace_analyzer::blobs;
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let analyzer = blobs::BlobAnalyzer::new();
    let blocks = analyzer.query_blocks().await?;
    let stats = analyzer.stats(blocks.clone()).await?;
    dbg!(stats);
    let block_details = analyzer.query_block(blocks[0].hash.clone()).await?;
    dbg!(block_details);
    Ok(())
}