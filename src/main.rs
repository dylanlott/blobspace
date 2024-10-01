mod blobs;

#[tokio::main]
async fn main() {
    let analyzer = blobs::BlobAnalyzer::new();
    let result = analyzer.query_blobs().await;
    if result.is_err() {
        dbg!(result.err().unwrap());
    } else {
        dbg!(result.unwrap());
    }
}