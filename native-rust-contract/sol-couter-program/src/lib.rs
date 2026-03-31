use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    entrypoint,
    msg,
    pubkey::Pubkey,
};

entrypoint!(counter_contract);

#[derive(BorshDeserialize, BorshSerialize)]
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}

pub fn counter_contract(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut account_iter = accounts.iter();
    let account = next_account_info(&mut account_iter)?;
    // let account = next_account_info(&mut accounts.iter())?;

    let mut counter_data = Counter::try_from_slice(&account.data.borrow())?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

    match instruction_type {
        InstructionType::Increment(amount) => {
            counter_data.count += amount;
        }
        InstructionType::Decrement(amount) => {
            counter_data.count -= amount;
        }
    }

    counter_data.serialize(&mut *account.data.borrow_mut())?;

    msg!("Counter updated to {}", counter_data.count);

    Ok(())
}
