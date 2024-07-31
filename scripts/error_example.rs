use super::TransactionService;
use crate::{models::transaction::TransactionType, services::DomainError};
use async_trait::async_trait;
use log::{error, info};
use mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::{CollectionDetails, Creator, DataV2},
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_program::system_program::ID as SYSTEM_PROGRAM_ID;
use solana_sdk::{
    signature::Keypair,
    signer::{EncodableKey, Signer},
    system_instruction, sysvar,
    transaction::Transaction,
};
use std::str::FromStr;

const TOKEN_PROGRAM_PUBKEY: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

pub struct TransactionServiceImpl {
    rpc_client: RpcClient,
    update_authority: Keypair,
}

impl TransactionServiceImpl {
    pub fn new() -> Self {
        let update_authority = Keypair::read_from_file("my_wallet.json") // TODO: bring this from config
            .expect("Error loading update authority Keypair");
        let rpc_url = "https://api.devnet.solana.com".to_string();
        info!(
            "The update authority is: {}",
            update_authority.pubkey().to_string()
        );

        Self {
            rpc_client: RpcClient::new(rpc_url), // TODO: bring this from config and save it in service
            update_authority,
        }
    }

    fn get_transfer_transaction(&self) -> Result<Vec<u8>, DomainError> {
        let from_pubkey = Pubkey::from_str("SM8ru5sNeUtAdSn7nAS7TtLPqihZfLA74AQidm44FuH")
            .map_err(|_| DomainError::InvalidPubkey)?;
        let to_pubkey = Pubkey::from_str("AfdWemKsT1RTK8m4tVuPcFizBWZ3oJpNUbPUhZZyeo63")
            .map_err(|_| DomainError::InvalidPubkey)?;
        let amount = 1_000_000_0;

        let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, amount);

        let transaction = Transaction::new_with_payer(&[instruction], Some(&from_pubkey));

        bincode::serialize(&transaction).map_err(|e| {
            error!("Error serializing transacion: {e:?}");
            DomainError::SerializeTransactionError
        })
    }

    async fn get_create_collection_nft_transaction(&self) -> Result<Vec<u8>, DomainError> {
        let from_pubkey = Pubkey::from_str("SM8ru5sNeUtAdSn7nAS7TtLPqihZfLA74AQidm44FuH")
            .map_err(|_| DomainError::InvalidPubkey)?;
        let mint_authority = Keypair::new();
        let mint = Keypair::new();

        let (metadata, _) = Pubkey::find_program_address(
            &[
                b"metadata",
                &mpl_token_metadata::ID.to_bytes(),
                &mint.pubkey().to_bytes(),
            ],
            &solana_program::pubkey::Pubkey::from(mpl_token_metadata::ID.to_bytes()),
        );

        let metadata_account = CreateMetadataAccountV3 {
            metadata,
            mint: mint.pubkey(),
            mint_authority: mint_authority.pubkey(),
            payer: from_pubkey,
            update_authority: (self.update_authority.pubkey(), true),
            system_program: SYSTEM_PROGRAM_ID,
            rent: Some(sysvar::rent::ID),
        };

        let creators = vec![Creator {
            address: self.update_authority.pubkey(),
            verified: true,
            share: 100,
        }];

        let data = DataV2 {
            name: "NFT Rust".to_string(),
            symbol: "CNR".to_string(),
            uri: "https://example.com".to_string(),
            seller_fee_basis_points: 5,
            creators: Some(creators),
            collection: None,
            uses: None,
        };

        let metadata_instruction_args = CreateMetadataAccountV3InstructionArgs {
            data,
            is_mutable: false,
            collection_details: Some(CollectionDetails::V1 { size: 1 }),
        };

        let metadata_instruction = metadata_account.instruction(metadata_instruction_args);

        let (edition, _) = Pubkey::find_program_address(
            &[
                b"metadata",
                &mpl_token_metadata::ID.to_bytes(),
                &mint.pubkey().to_bytes(),
                b"edition",
            ],
            &solana_program::pubkey::Pubkey::from(mpl_token_metadata::ID.to_bytes()),
        );

        //TODO: If I use spl_token crate I have conflicts with dep solana_sdk
        let token_program =
            Pubkey::from_str(TOKEN_PROGRAM_PUBKEY).map_err(|_| DomainError::InvalidPubkey)?;
        let master_edition_account = CreateMasterEditionV3 {
            edition,
            mint: mint.pubkey(),
            update_authority: self.update_authority.pubkey(),
            mint_authority: mint_authority.pubkey(),
            payer: from_pubkey,
            metadata,
            token_program,
            system_program: SYSTEM_PROGRAM_ID,
            rent: Some(sysvar::rent::ID),
        };

        let master_edition_instruction_args = CreateMasterEditionV3InstructionArgs {
            max_supply: Some(1),
        };
        let master_edition_instruction =
            master_edition_account.instruction(master_edition_instruction_args);

        let mut transaction = Transaction::new_with_payer(
            &[metadata_instruction, master_edition_instruction],
            Some(&from_pubkey),
        );

        // TODO: IF THIS WORKS ADD THE INSTRUCTION TO CALL MY PROGRAM AND INITIALIZE THE MintInfo ACCOUNT
        let recent_blockhash = self.rpc_client.get_latest_blockhash().await.map_err(|e| {
            error!("Error geting latest blockhash {e:?}");
            DomainError::FetchBlockhashError
        })?;

        transaction.message.recent_blockhash = recent_blockhash;
        transaction.partial_sign(&[&mint_authority, &self.update_authority], recent_blockhash);

        bincode::serialize(&transaction).map_err(|e| {
            error!("Error serializing transacion: {e:?}");
            DomainError::SerializeTransactionError
        })
    }
}

#[async_trait]
impl TransactionService for TransactionServiceImpl {
    async fn get_transaction(
        &self,
        transaction_type: TransactionType,
    ) -> Result<Vec<u8>, DomainError> {
        match transaction_type {
            TransactionType::Transfer => self.get_transfer_transaction(),
            TransactionType::CreateCollectionNFT => {
                self.get_create_collection_nft_transaction().await
            }
            TransactionType::MintNFT => todo!(),
        }
    }
}
