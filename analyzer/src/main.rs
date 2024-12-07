use blobspace_analyzer::blobs;
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let analyzer = blobs::BlobAnalyzer::new();
    let blocks = analyzer.query_blocks().await?;
    let stats = analyzer.stats(blocks.clone()).await?;
    dbg!(stats);
    for block in blocks {
        println!("{:?}", block);
        let blob_data = analyzer.query_blobs(block.number, block.number).await?;
        dbg!(blob_data);
    }
    Ok(())
}