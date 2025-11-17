use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum ForgeInstruction {
    NewV1 = 0,
    MintV1 = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MintV1 {
    pub config_bump: u8,
    pub collection_authority_bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct NewV1 {
    pub name: [u8; 64],
    pub uri: [u8; 128],
    pub multiplier: u64,
    pub durability: u64,
    pub ingredients: [Pubkey; 3],
    pub amounts: [u64; 3],
    pub config_bump: u8,
    pub collection_authority_bump: u8,
    pub _padding: [u8; 6],
}

instruction!(ForgeInstruction, MintV1);
instruction!(ForgeInstruction, NewV1);
