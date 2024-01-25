use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program::invoke_signed,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
    },
    spl_token::{instruction::transfer_checked, state::Mint},
};

solana_program::entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let source_info = next_account_info(account_info_iter)?;
    let mint_to = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let token_programm_info = next_account_info(account_info_iter)?;

    let (expected_authority, bump_seed) = Pubkey::find_program_address(&[b"authority"], program_id);

    if expected_authority != *authority_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // get amount from instruction data
    let amount = u64::from_be_bytes(
        instruction_data
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    let mint = Mint::unpack(&mint_to.try_borrow_data()?)?;
    let decimals = mint.decimals;

    invoke_signed(
        &transfer_checked(
            token_programm_info.key,
            source_info.key,
            mint_to.key,
            destination_info.key,
            authority_info.key,
            &[],
            amount,
            decimals,
        )
        .unwrap(),
        &[
            source_info.clone(),
            mint_to.clone(),
            destination_info.clone(),
            authority_info.clone(),
            token_programm_info.clone(),
        ],
        &[&[b"authority", &[bump_seed]]],
    )
}
