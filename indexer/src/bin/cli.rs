use futures::{Future, TryStreamExt};
use reth::core::primitives::Transaction;
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

async fn exex<Node: FullNodeComponents>(mut ctx: ExExContext<Node>) -> eyre::Result<()> {
    println!("exex booting up");

    // TODO: Track a "latest indexed block" and set the head to that number to be restart-tolerant
    ctx.set_notifications_without_head();
    handle_notifications(ctx).await?;

    Ok(())
}

async fn handle_notifications<Node: FullNodeComponents>(
    mut ctx: ExExContext<Node>,
) -> eyre::Result<()> {
    loop {
        let result = ctx.notifications.try_next().await?;
        match result {
            Some(notification) => match notification {
                reth_exex::ExExNotification::ChainCommitted { new } => {
                    new.blocks_iter().for_each(move |block| {
                        for tx in block.transactions() {
                            tx.clone().is_eip4844().then(|| {
                                println!("found 4844 tx: {:?}", tx);
                                // TODO: Send off to the indexer
                            });
                        }
                    });
                    println!("chain committed: {:?}", new);
                }
                reth_exex::ExExNotification::ChainReorged { old, new } => {
                    println!("chain reorged: {:?}, {:?}", old, new);
                }
                reth_exex::ExExNotification::ChainReverted { old } => {
                    println!("chain reverted: {:?}", old);
                }
            },
            None => continue,
        }
    }
}

async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> eyre::Result<impl Future<Output = eyre::Result<()>>> {
    println!("initializing exex");
    Ok(exex(ctx))
}
