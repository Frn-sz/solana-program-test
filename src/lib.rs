mod constants;
mod entrypoint;
mod error;
mod instructions;
mod processor;
mod state;

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use borsh::BorshSerialize;
    use litesvm::LiteSVM;
    use solana_pubkey::Pubkey;
    use solana_sdk::{
        instruction::Instruction, signature::read_keypair_file, transaction::Transaction,
    };

    use crate::instructions::CreateAta;

    fn setup() -> LiteSVM {
        let svm = litesvm::LiteSVM::new();
        svm
    }

    #[test]
    fn test_create_ata() {
        let mut svm = setup();

        let keypair = read_keypair_file("/home/fernando/.config/solana/id.json");

        let mut data = Vec::new();
        let wallet = Pubkey::from_str("7mt5NjG1EBXkYDbrN9Gqz5eZ5Vz5vgjvsXBeG9rBELiP").unwrap();
        let mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();

        svm.set_account(wallet, );
        let _ = CreateAta {
            wallet: wallet.to_bytes(),
            mint: mint.to_bytes(),
            discriminator: 0,
        }
        .serialize(&mut data);

        let ix = Instruction {
            program_id: Pubkey::from_str("6t6XmPzhjQB2QuH9uje3SmfX88JiRUdaJFJeQ6QKbn8U").unwrap(),
            accounts: vec![],
            data,
        };

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&wallet),
            &keypair,
            svm.latest_blockhash(),
        );

        let response = svm.send_transaction(tx);

        match response {
            Ok(metadata) => {
                println!("{:?}", metadata);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
