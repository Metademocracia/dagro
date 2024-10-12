use solana_program::account_info::next_account_info;

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program::invoke_signed,
};
use spl_token::{instruction as token_instruction};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey, // Prefix unused variables with an underscore to silence warnings
    accounts: &[AccountInfo],
    _instruction_data: &[u8], // Same here
) -> ProgramResult {
    msg!("DAGRO Token Program");

    let account_iter = &mut accounts.iter();
    let payer_account = next_account_info(account_iter)?;
    let mint_account = next_account_info(account_iter)?;
    let mint_authority_account = next_account_info(account_iter)?;

    let rent_sysvar = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;

    // Create Mint account with 987 million tokens, 8 decimals
    let mint_amount: u64 = 987_000_000_000_000; // Use u64 for large values

    // Call the SPL Token's `initialize_mint` instruction to create the token mint
    let ix = token_instruction::initialize_mint(
        &token_program.key,
        &mint_account.key,
        &mint_authority_account.key,
        None,
        8, // Decimals
    )?;

    invoke_signed(
        &ix,
        &[
            payer_account.clone(),
            mint_account.clone(),
            mint_authority_account.clone(),
            rent_sysvar.clone(),
            token_program.clone(),
        ],
        &[],
    )?;

    msg!("DAGRO Token Minted with supply: {}", mint_amount);
    Ok(())
}
