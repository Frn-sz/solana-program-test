use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    pubkey::find_program_address,
};

use crate::{
    constants::{SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID, SYSTEM_PROGRAM_ID},
    instructions::{AtaInstructions, CreateAta},
};

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &[u8; 32],
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) {
        let ix_data = AtaInstructions::unpack(instruction_data);

        match ix_data {
            Ok(AtaInstructions::CreateATA(create_ata)) => {
                Self::process_create_ata(&create_ata, accounts);
            }
            Err(_) => {}
        }
    }
    fn process_create_ata(create_ata: &CreateAta, accounts: &[AccountInfo]) {
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
        let mut acc_iter = accounts.iter();

        let accounts = [
            acc_iter.next().unwrap(),
            acc_iter.next().unwrap(),
            acc_iter.next().unwrap(),
            acc_iter.next().unwrap(),
            acc_iter.next().unwrap(),
            acc_iter.next().unwrap(),
        ];

        let res = invoke(&ix, &accounts);

        match res {
            Ok(_) => {
                msg!("Associated token account created successfully");
            }
            Err(_) => {}
        }
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
