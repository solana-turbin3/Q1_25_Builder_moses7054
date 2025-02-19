use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct NgoAccount {
    #[max_len(20)]
    pub name: String, // Fixed-size NGO name (25 bytes)
    pub completed_projects: u32, // Number of completed projects (4 bytes)
    pub merkel_root: Option<[u8; 32]>, // Merkle root of all completed projects
    pub ngo_bump: u8,            // PDA bump seed
}
