use std::io::Read;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    pubkey::find_program_address,
};

use crate::constants::{
    SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID, SYSTEM_PROGRAM_ID,
};

pub struct Processor {}

impl Processor {
    pub fn process_create_ata(
        program_id: &[u8; 32],
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) {
        let ata = Self::derive_ata(wallet, mint);

        let ix = Instruction {
            program_id: &SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
            data: &[],
            accounts: &[
                AccountMeta::new(payer, true, true),
                AccountMeta::new(&ata, true, false),
                AccountMeta::readonly(wallet),
                AccountMeta::readonly(mint),
                AccountMeta::readonly(&SYSTEM_PROGRAM_ID),
                AccountMeta::readonly(&SPL_TOKEN_PROGRAM_ID),
            ],
        };
    }

    fn derive_ata(wallet: &[u8; 32], mint: &[u8; 32]) -> [u8; 32] {
        let seeds = [
            wallet.as_ref(),
            &SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
            mint.as_ref(),
        ];

        let (ata, _) = find_program_address(&seeds, &SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID);

        ata
    }
}
