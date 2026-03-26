use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(first_counter_contract);

// pub fn first_counter_contract(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8],
// ) -> ProgramResult {
//     Ok(())
// }

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    pub count: u32,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

pub fn first_counter_contract(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // let acc = next_account_info(&mut accounts.iter());
    // ? is used to propagate the error if there is one account in the list
    let acc = next_account_info(&mut accounts.iter())?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

    // match instruction_type {
    //     let counter_data = Counter
    // }
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(value) => {
            msg!("Incrementing by {}", value);
            // let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;
            counter_data.count += value;
        }
        InstructionType::Decrement(value) => {
            msg!("Decrementing by {}", value);
            counter_data.count -= value;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut())?;

    msg!("New counter value: {}", counter_data.count);

    Ok(())
}
