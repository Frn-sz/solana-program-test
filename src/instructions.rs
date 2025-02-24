pub enum Instruction {
    CreateATA(CreateAta),
}

pub struct CreateAta {
    pub discriminator: u8,
    mint: [u8; 32],
}

impl Instruction {
    pub fn unpack(input: &) {

    }
}
