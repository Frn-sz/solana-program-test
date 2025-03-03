use pinocchio::{account_info::AccountInfo, entrypoint, pubkey::Pubkey, ProgramResult};

use crate::{instructions::AtaInstructions, processor::Processor};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix_data = AtaInstructions::unpack(instruction_data);

    match ix_data {
        Ok(AtaInstructions::CreateATA(create_ata)) => {
            let res = Processor::process_create_ata(&create_ata, accounts);

            match res {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        Err(_) => {}
    }
    Ok(())
}
