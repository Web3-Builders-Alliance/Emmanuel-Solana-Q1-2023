# Code Journal 1

In this code journal 1 will be going over the program account-data.
This program is designed to process address data and persist it into an account related to the address.
The code is found in this link https://github.com/solana-developers/program-examples/tree/main/basics/account-data/native

 ---

### 1. Program overview

The program "account-data" is split into below logical units;

a.
lib.rs
This defines the entrypoint of the program and registers all the modules
b.
modules
i. instructions which contains create.rs and mod.rs
ii. processor i.e processor.rs
iii. state which contains address_info.rs and mod.rs

---

### 2. Mechanism


The program uses cross-program invocation mechanism.
Create address_info is done through cross-program invocation by calling solana_program::system_instruction::create_account

---

### 3. Program modules

### a. lib.rs

// Import module solana_program::entrypoint
use solana_program::entrypoint;
// Import module processor and its function process_instruction
use processor::process_instruction;
// Declare modules(instructions,processor and state) as public
pub mod instructions;
pub mod processor;
pub mod state;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);
---

### b. processor.rs

// Import crate borsh
use borsh::{ BorshDeserialize, BorshSerialize };
// Import crate solana_program
use solana_program::{
    account_info::AccountInfo, 
    entrypoint::ProgramResult, 
    program_error::ProgramError,
    pubkey::Pubkey,
};
// Import modules instructions and state
use crate::instructions;
use crate::state::AddressInfo;

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the account-data program was loaded into
    accounts: &[AccountInfo], // The account to be used for the address
    instruction_data: &[u8], // The address data
) -> ProgramResult {
	// Deserialize this instance from a slice of bytes and match it against struct AddressInfo
    match AddressInfo::try_from_slice(&instruction_data) {
		// Create address_info through function create_address_info which
		// makes cross-program invocation by calling solana_program::system_instruction::create_account
        Ok(address_info) => return instructions::create::create_address_info(
            program_id, accounts, address_info
        ),
        Err(_) => {}, // Throw an error if the match fails
    };
	// Throw an error indicating that instruction data is invalid
    Err(ProgramError::InvalidInstructionData)
}

### c. address_info.rs

// Import crate borsh
use borsh::{ BorshDeserialize, BorshSerialize };

// Define the type of state stored in accounts
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
}
// This block accepts aurguments in method new and returns a newly created instance of struct AddressInfo
impl AddressInfo {

    pub fn new(
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Self {
        AddressInfo {
            name,
            house_number,
            street,
            city,
        }
    }
}

### d. mod.rs

// Define module address_info as public
pub mod address_info;
// Import module address_info and all its public members and methods
pub use address_info::*;

### e. create.rs

// Import crate borsh
use borsh::{ BorshDeserialize, BorshSerialize };
// Import crate solana_program
use solana_program::{
    account_info::{ AccountInfo, next_account_info },
    entrypoint::ProgramResult, 
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    system_program,
    sysvar::Sysvar,
};
// Import module state
use crate::state::AddressInfo;


pub fn create_address_info(
    program_id: &Pubkey, // Public key of the account the account-data program was loaded into
    accounts: &[AccountInfo], // The account to be used for the address
    address_info: AddressInfo, The address data
) -> ProgramResult {
	// Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();
	// Get the account to be be used for the address
    let address_info_account = next_account_info(accounts_iter)?;
	// Get the payer for the gas fees
    let payer = next_account_info(accounts_iter)?;
	// Get the account where the account-data program was loaded into
    let system_program = next_account_info(accounts_iter)?;
	// Serialize address_info and then get its length
    let account_span = (address_info.try_to_vec()?).len();
	// Get the minimum rent fees in lamports required for the account_span
    let lamports_required = (Rent::get()?).minimum_balance(account_span);
	
	// Create address_info by making cross-program invocation through calling solana_program::system_instruction::create_account
    invoke(
        &system_instruction::create_account(
            &payer.key,
            &address_info_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            payer.clone(), address_info_account.clone(), system_program.clone()
        ]
    )?;
    // Add the serialized address_info data into the address_info_account
    address_info.serialize(&mut &mut address_info_account.data.borrow_mut()[..])?;
	// gracefully exit the function
    Ok(())
}

### f. mod.rs

// Define module create as public
pub mod create;
// Import module create and all its public members and methods
pub use create::*;

### Final Thoughts

This a smart contract(program) that processes and persists address data, it helped me understand the logical flow of data and how its stored in the address accounts. 
