use std::{borrow::BorrowMut, str::FromStr};

use models::{ReadOnlyInstruction, ReadOnlyTransaction};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{address_lookup_table::state::AddressLookupTable, pubkey::Pubkey};
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedTransaction, EncodedTransactionWithStatusMeta,
    UiInstruction, UiMessage, UiParsedInstruction, UiParsedMessage, UiRawMessage,
};

pub struct EncodedTransactionExtractor {
    rpc_client: RpcClient,
    block_time: i64,
    encoded_transaction: EncodedTransactionWithStatusMeta,
}

impl EncodedTransactionExtractor {
    pub fn new(
        rpc_url: &str,
        block_time: i64,
        encoded_transaction: &EncodedTransactionWithStatusMeta,
    ) -> Self {
        let rpc_client = RpcClient::new(rpc_url.to_string());

        Self {
            rpc_client,
            block_time,
            encoded_transaction: encoded_transaction.clone(),
        }
    }

    pub async fn parse_readonly_transaction(&mut self) -> Option<ReadOnlyTransaction> {
        if let Some(meta) = self.encoded_transaction.meta.clone() {
            if meta.err.is_some() {
                return None;
            }
        }

        let signature = self.extract_signature();
        if let Some(signature) = signature {
            let accounts = self.extract_accounts().await;
            let instructions = self.extract_readonly_instructions().await;

            return Some(ReadOnlyTransaction {
                signature,
                log_messages: self.extract_raw_logs(),
                accounts,
                instructions,
            });
        }

        None
    }

    pub fn extract_signature(&self) -> Option<String> {
        if let EncodedTransaction::Json(transaction) = &self.encoded_transaction.transaction {
            if let Some(signature) = transaction.signatures.first() {
                return Some(signature.to_string());
            }
        }

        None
    }

    pub async fn extract_accounts(&self) -> Vec<String> {
        let mut accounts = Vec::new();
        let mut writable = Vec::new();
        let mut readonly = Vec::new();
        let meta = &self.encoded_transaction.clone().meta.unwrap();
        if let EncodedTransaction::Json(transaction) = &self.encoded_transaction.transaction {
            if let UiMessage::Raw(UiRawMessage {
                account_keys,
                address_table_lookups,
                ..
            }) = &transaction.message
            {
                account_keys
                    .iter()
                    .for_each(|account_key| accounts.push(account_key.to_string()));
                if let Some(address_table_lookups) = address_table_lookups {
                    for lookup in address_table_lookups {
                        let account_key = lookup.account_key.clone();
                        let account = self
                            .rpc_client
                            .get_account(&Pubkey::from_str(&account_key).unwrap())
                            .await
                            .unwrap();
                        if let Ok(atl) = AddressLookupTable::deserialize(&account.data) {
                            let addresses = atl
                                .addresses
                                .iter()
                                .map(|address| address.to_string())
                                .collect::<Vec<_>>();
                            let lookup_writable = lookup
                                .writable_indexes
                                .iter()
                                .map(|index| addresses.get(*index as usize).unwrap().to_string())
                                .collect::<Vec<_>>();
                            writable.extend(lookup_writable);
                            let lookup_readonly = lookup
                                .readonly_indexes
                                .iter()
                                .map(|index| addresses.get(*index as usize).unwrap().to_string())
                                .collect::<Vec<_>>();
                            readonly.extend(lookup_readonly);
                        }
                    }
                }
            }
        }

        if let OptionSerializer::Some(loaded_addresses) = &meta.loaded_addresses {
            loaded_addresses
                .writable
                .iter()
                .for_each(|address| accounts.push(address.to_string()));
            loaded_addresses
                .readonly
                .iter()
                .for_each(|address| accounts.push(address.to_string()));
        }

        accounts.extend(writable);
        accounts.extend(readonly);

        accounts
    }

    pub async fn extract_readonly_instructions(&self) -> Vec<ReadOnlyInstruction> {
        let accounts = self.extract_accounts().await;
        let mut tx_instructions = Vec::new();
        if let EncodedTransaction::Json(transaction) = self.encoded_transaction.transaction.clone()
        {
            match transaction.message.clone() {
                UiMessage::Parsed(UiParsedMessage { instructions, .. }) => {
                    for instruction in instructions {
                        match instruction {
                            UiInstruction::Compiled(_) => {}
                            UiInstruction::Parsed(parsed) => match parsed {
                                UiParsedInstruction::PartiallyDecoded(decoded) => {
                                    tx_instructions.push(ReadOnlyInstruction {
                                        program_id: decoded.program_id,
                                        data: decoded.data,
                                        accounts: decoded.accounts,
                                        inner_instructions: Vec::new(),
                                    });
                                }
                                UiParsedInstruction::Parsed(parsed) => {
                                    tx_instructions.push(ReadOnlyInstruction {
                                        program_id: parsed.program_id,
                                        data: parsed.parsed.to_string(),
                                        accounts: Vec::new(),
                                        inner_instructions: Vec::new(),
                                    });
                                }
                            },
                        }
                    }
                }
                UiMessage::Raw(UiRawMessage { instructions, .. }) => {
                    for instruction in instructions {
                        let program_id = accounts
                            .get(instruction.program_id_index as usize)
                            .unwrap()
                            .to_string();
                        let instruction_accounts = instruction
                            .accounts
                            .iter()
                            .map(|account_index| {
                                accounts.get(*account_index as usize).unwrap().to_string()
                            })
                            .collect::<Vec<_>>();
                        tx_instructions.push(ReadOnlyInstruction {
                            program_id,
                            data: instruction.data,
                            accounts: instruction_accounts,
                            inner_instructions: Vec::new(),
                        });
                    }
                }
            }
        }

        if let Some(meta) = self.encoded_transaction.meta.clone() {
            if let OptionSerializer::Some(inner_instructions) = meta.inner_instructions {
                inner_instructions.iter().for_each(|inner_instruction| {
                    let parent_instruction = tx_instructions
                        .get_mut(inner_instruction.index as usize)
                        .unwrap();

                    inner_instruction
                        .instructions
                        .iter()
                        .for_each(|instruction| {
                            if let UiInstruction::Compiled(instruction) = instruction {
                                let program_id = accounts
                                    .get(instruction.program_id_index as usize)
                                    .unwrap()
                                    .to_string();
                                let instruction_accounts = instruction
                                    .accounts
                                    .iter()
                                    .map(|account_index| {
                                        accounts.get(*account_index as usize).unwrap().to_string()
                                    })
                                    .collect::<Vec<_>>();
                                let inner_readonly_instruction = ReadOnlyInstruction {
                                    program_id,
                                    data: instruction.data.clone(),
                                    accounts: instruction_accounts,
                                    inner_instructions: Vec::new(),
                                };

                                if let Some(stack_height) = instruction.stack_height {
                                    if stack_height.eq(&2) {
                                        parent_instruction
                                            .inner_instructions
                                            .push(inner_readonly_instruction);
                                    } else {
                                        let mut target = parent_instruction.borrow_mut();
                                        for _ in 2..stack_height {
                                            target = target.inner_instructions.last_mut().unwrap();
                                        }
                                        target.inner_instructions.push(inner_readonly_instruction);
                                    }
                                }
                            }
                        });
                });
            }
        }

        tx_instructions
    }

    pub fn extract_raw_logs(&self) -> Vec<String> {
        let mut logs = Vec::new();
        let meta = &self.encoded_transaction.clone().meta.unwrap();

        if let OptionSerializer::Some(log_messages) = &meta.log_messages {
            log_messages.iter().for_each(|log| logs.push(log.clone()));
        }

        logs
    }
}
