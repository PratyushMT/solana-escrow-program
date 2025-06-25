use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64, //offer id
    pub maker: Pubkey, //maker of the offer
    pub token_mint_a: Pubkey, //token mint of the token being offered
    pub token_mint_b: Pubkey, //token mint of the token wanted
    pub token_b_wanted_amount: u64, //amount of token b being wanted
    pub bump: u8, //bump for PDA
}
