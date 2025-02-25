use borsh::{BorshDeserialize, BorshSerialize};

pub enum AtaInstructions {
    CreateATA(CreateAta),
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CreateAta {
    pub discriminator: u8,
    pub mint: [u8; 32],
    pub wallet: [u8; 32],
}

impl AtaInstructions {
    pub fn unpack(input: &[u8]) -> Result<AtaInstructions, ()> {
        let (disc, rest) = input.split_first().unwrap();

        match disc {
            0 => {
                let mint: [u8; 32] = rest[0..32].try_into().unwrap();
                let wallet: [u8; 32] = rest[32..64].try_into().unwrap();

                let create_ata = CreateAta {
                    discriminator: *disc,
                    mint,
                    wallet,
                };

                Ok(AtaInstructions::CreateATA(create_ata))
            }
            _ => Err(()),
        }
    }
}
