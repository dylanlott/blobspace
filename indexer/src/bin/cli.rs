use futures::{Future, TryStreamExt};
use reth_exex::{ExExContext, ExExNotification};
use reth_node_ethereum::EthereumNode;
use reth_node_api::FullNodeComponents;
use tokio;

#[tokio::main]
async fn main() -> eyre::Result<()> {
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
    while let Some(notification) = ctx.notifications.try_next().await? {
        match &notification {
            ExExNotification::ChainCommitted { new, .. } => {
                println!("new tip committed to block {:?}", new.tip());
            }
            ExExNotification::ChainReorged { new, old } => {
                println!(
                    "Chain reorganized from block {:?} to {:?}",
                    old.tip(),
                    new.tip()
                );
            }
            ExExNotification::ChainReverted { old, .. } => {
                println!("Chain reverted to block {:?}", old.tip());
            }
        }
    }
    Ok(())
}

async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> eyre::Result<impl Future<Output = eyre::Result<()>>> {
    Ok(exex(ctx))
}
