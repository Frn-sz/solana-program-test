use pinocchio::{
    account_info::AccountInfo,
    get_account_info,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::find_program_address,
    ProgramResult,
};

use crate::{
    constants::{SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID, SYSTEM_PROGRAM_ID},
    instructions::CreateAta,
};

pub struct Processor {}

impl Processor {
    pub fn process_create_ata(create_ata: &CreateAta, accounts: &[AccountInfo]) -> ProgramResult {
        let ata = Self::derive_ata(&create_ata.wallet, &create_ata.mint);

        let ix = Instruction {
            program_id: &SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
            data: &[],
            accounts: &[
                AccountMeta::new(&create_ata.wallet, true, true),
                AccountMeta::new(&ata, true, false),
                AccountMeta::readonly(&create_ata.wallet),
                AccountMeta::readonly(&create_ata.mint),
                AccountMeta::readonly(&SYSTEM_PROGRAM_ID),
                AccountMeta::readonly(&SPL_TOKEN_PROGRAM_ID),
            ],
        };

        let accounts = [
            get_account_info!(accounts, 0 as usize),
            get_account_info!(accounts, 1 as usize),
            get_account_info!(accounts, 2 as usize),
            get_account_info!(accounts, 3 as usize),
            get_account_info!(accounts, 4 as usize),
            get_account_info!(accounts, 5 as usize),
        ];

        invoke(&ix, &accounts)?;

        Ok(())
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
