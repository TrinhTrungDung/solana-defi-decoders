use anyhow::Result;
use decoders::drift_v2::DriftV2Instruction;
use extractors::EncodedTransactionExtractor;
use futures::StreamExt;
use settings::Settings;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;
use utils::find_all_instructions_by_program_id;

pub const DRIFT_V2: &str = "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH";

#[tokio::main]
async fn main() -> Result<()> {
    // Setup tracing and parse args.
    let settings = Settings::new()?;

    let file_appender = tracing_appender::rolling::hourly(
        settings.tracing.dirname.clone(),
        settings.tracing.prefix.clone(),
    );
    let filter = EnvFilter::builder()
        .with_default_directive(settings.tracing.to_level_filter().into())
        .from_env()?;
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(non_blocking)
        .init();

    let pubsub_client = PubsubClient::new(&settings.solana.wss.clone()).await?;
    if let Ok((mut stream, _)) = pubsub_client
        .block_subscribe(
            RpcBlockSubscribeFilter::MentionsAccountOrProgram(DRIFT_V2.to_string()),
            Some(RpcBlockSubscribeConfig {
                commitment: Some(CommitmentConfig::confirmed()),
                encoding: Some(UiTransactionEncoding::JsonParsed),
                transaction_details: Some(TransactionDetails::Full),
                show_rewards: None,
                max_supported_transaction_version: Some(0),
            }),
        )
        .await
    {
        while let Some(res) = stream.next().await {
            if let Some(block) = res.value.block {
                if block.block_time.is_none() {
                    continue;
                }

                if let Some(transactions) = block.transactions {
                    for transaction in transactions {
                        if let Some(meta) = transaction.meta.clone() {
                            if meta.err.is_some() {
                                continue;
                            }
                        }

                        let block_time = block.block_time.unwrap();

                        let mut extractor = EncodedTransactionExtractor::new(
                            &settings.solana.rpc.clone(),
                            block_time,
                            &transaction,
                        );
                        if let Some(signature) = extractor.extract_signature() {
                            if let Some(tx) = extractor.parse_readonly_transaction().await {
                                let instructions =
                                    find_all_instructions_by_program_id(tx.instructions, DRIFT_V2);
                                for instruction in instructions {
                                    let data =
                                        bs58::decode(instruction.data.clone()).into_vec().unwrap();
                                    match DriftV2Instruction::unpack(&data) {
                                        Ok(instruction) => {
                                            info!(
                                                "Signature {:?} has instruction: {:?}",
                                                signature, instruction
                                            );
                                        }
                                        Err(_) => {
                                            warn!(
                                                "Unknown instruction data: {:?}: {:?}",
                                                signature, instruction.data
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
