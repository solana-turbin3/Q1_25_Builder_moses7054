use anchor_lang::prelude::*;

#[account]
pub struct NgoAccount {
    pub name: [u8; 25],          // Fixed-size NGO name (25 bytes)
    pub completed_projects: u32, // Number of completed projects (4 bytes)
    pub merkel_root: [u8; 32],   // Merkle root of all completed projects
    pub ngo_bump: u8,                // PDA bump seed
}

impl Space for NgoAccount {
    const INIT_SPACE: usize = 8  // Discriminator
    + 25  // Name
    + 4   // Completed projects
    + 32  // Merkle root
    + 1; // Bump
}
