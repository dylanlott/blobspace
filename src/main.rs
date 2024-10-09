use blobspace_analyzer::blobs;
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let analyzer = blobs::BlobAnalyzer::new();
    let blocks = analyzer.query_blocks().await?;
    let stats = analyzer.stats(blocks).await?;
    dbg!(stats);
    Ok(())
}