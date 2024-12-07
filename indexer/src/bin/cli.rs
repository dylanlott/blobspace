use futures::Future;
use reth::{providers::TransactionsProvider, transaction_pool::TransactionPool};
use reth_exex::ExExContext;
use reth_node_api::FullNodeComponents;
use reth_node_ethereum::EthereumNode;

fn main() -> eyre::Result<()> {
    println!("indexer booting up");
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("signet-indexer", exex_init)
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    })
}

async fn exex<Node: FullNodeComponents>(ctx: ExExContext<Node>) -> eyre::Result<()> {
    println!("exex booting up");

    while let Some(sidecar) = ctx.pool().blob_transaction_sidecars_listener().recv().await {
        // Transaction hash -- One index
        let tx_hash = sidecar.tx_hash;
        println!("tx hash: {:?}", tx_hash);
        // Blob Sidecare -- One value
        let blob_sidecar = sidecar.sidecar.clone();
        println!("blob sidecar: {:?}", blob_sidecar);
        // Block number
        if let Some(tx_details) = ctx.provider().transaction_by_hash(tx_hash)? {
            println!("tx details: {:?}", tx_details);
        } else {
            println!("tx not found");
        }
    }

    Ok(())
}

async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> eyre::Result<impl Future<Output = eyre::Result<()>>> {
    println!("initializing exex");
    Ok(exex(ctx))
}
