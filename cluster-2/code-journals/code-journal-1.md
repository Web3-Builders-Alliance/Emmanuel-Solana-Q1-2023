# Code Journal 1

In this code journal, I will be going over the anchor program account-data.
This program is designed to process address data and persist it into an account related to the address.
The code is found in this link https://github.com/solana-developers/program-examples/tree/main/basics/account-data/anchor

---

## 1. Program overview

The program "account-data" is split into below logical units;

a.
lib.rs
This defines the entrypoint of the program

b.
modules
i. instructions which contains create.rs and mod.rs
ii. state which contains address_info.rs and mod.rs

---

## 2. Mechanism


The program uses cross-program invocation mechanism.
Account for address_info is created through cross-program invocation by calling system_program::create_account

---

## 3. Program modules

### a. lib.rs

// Import to gain access to common anchor features

```
use anchor_lang::prelude::*;
```

// Import module instructions and all its members/functions

```
use instructions::*;
```

// Declare modules(instructions and state) as public

```
pub mod instructions;
pub mod state;
```

// Declare an id for your program that uniquely identifies the program

```
declare_id!("FFKtnYFyzPj1qFjE9epkrfYHJwZMdh8CvJrB6XsKeFVz");
```

```
// The program module is where you define your business logic
#[program]
pub mod anchor_program_example {
    use super::*;
	
	// function create_address_info accepts several arguments
    pub fn create_address_info(
        ctx: Context<CreateAddressInfo>, // Declare argument ctx of type Context that references CreateAddressInfo
        name: String, // Declare argument name of type string
        house_number: u8, // Declare argument house_number of type u8
        street: String, // Declare argument street of type string
        city: String, // Declare argument city of type string
    ) -> Result<()> {
        
		// local arguments are passed to function create_address_info existing in module create
        instructions::create::create_address_info(
            ctx,
            name,
            house_number,
            street,
            city,
        )
    }
}
```

---

### b. create.rs

// Import to gain access to common anchor features

```
use anchor_lang::prelude::*;
```

// Import module state and its member AddressInfo

```
use crate::state::AddressInfo;
```

// logical steps for function create_address_info

```
pub fn create_address_info( // Declare function create_address_info
    ctx: Context<CreateAddressInfo>, // Declare argument ctx of type Context that references CreateAddressInfo
    name: String, // Declare argument name of type string
    house_number: u8, // Declare argument house_number of type u8
    street: String, // Declare argument street of type string
    city: String, // Declare argument city of type string
) -> Result<()> {
	
	// Declare a new instance(address_info) of type struct AddressInfo with values 
    let address_info = AddressInfo::new(
        name,
        house_number,
        street,
        city,
    );

	// Serialize address_info and then get its length
    let account_span = (address_info.try_to_vec()?).len();
	// Get the minimum rent fees in lamports required for the account_span
    let lamports_required = (Rent::get()?).minimum_balance(account_span);
	
	// Create address_info account by making cross-program invocation through calling system_program::create_account
    system_program::create_account(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.address_info.to_account_info(),
            },
        ),
        lamports_required,
        account_span as u64,
        &ctx.accounts.system_program.key(),
    )?;

	// Declare a mutable instance of address_info account by referencing it to context CreateAddressInfo
    let address_info_account = &mut ctx.accounts.address_info;
	// Add the address_info data into the address_info_account
    address_info_account.set_inner(address_info);
	// gracefully exit the function
    Ok(())
}
```

```
// This defines the struct as an account and hence can be (de)serialized
#[derive(Accounts)]
//Declare struct CreateAddressInfo with lifetime
pub struct CreateAddressInfo<'info> {
    #[account(mut)] // This defines address_info as a mutable account
    address_info: Account<'info, AddressInfo>, // address_info is defined as an account of type struct AddressInfo
    #[account(mut)] // This defines payer as a mutable account and it will be the signer of transactions
    payer: Signer<'info>, // payer is defined as a signer
    system_program: Program<'info, System>, // This defines system_program as type Program
}
```

---

### c. mod.rs

// Define module create as public

```
pub mod create;
```

// Import module create and all its public members and methods

```
pub use create::*;
```

---

### d. address_info.rs

// Import to gain access to common anchor features

```
use anchor_lang::prelude::*;
```

// Define the type of state stored in struct AddressInfo marked as account and hence can be (de)serialized

```
#[account]
pub struct AddressInfo {
    pub name: String, // Declare argument name of type string
    pub house_number: u8, // Declare argument house_number of type u8
    pub street: String, // Declare argument street of type string
    pub city: String, // Declare argument city of type string
}
```

// This block accepts arguments in method new and returns a newly created instance of struct AddressInfo

```
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
```

---

### e. mod.rs

// Define module address_info as public

```
pub mod address_info;
```

// Import module address_info and all its public members and methods

```
pub use address_info::*;
```

---

## 4. Code Journal summary

### a. What are the concepts 
  - borrowing, ownership
### b. What is the organization?
       The code is organized through modules.
	   #### lib.rs
	   This defines the entrypoint of the program

	   #### modules
       - instructions which contains create.rs and mod.rs
       - state which contains address_info.rs and mod.rs
### c. 
#### i. What is the contract doing?
   This a smart contract(program) that processes and persists address data. 
#### ii. What is the mechanism? 
    The program uses cross-program invocation mechanism.
    Account for address_info is created through cross-program invocation by calling system_program::create_account
### d. How could it be better? More efficient? Safer?
  - The code could be safer and better if we added "owner: Pubkey" to "pub struct AddressInfo" in address_info.rs.
    This will allow us to conduct owner check before passing data to account in "fn create_address_info" within create.rs.
	Below is the code we will added;
	
	```
	if address_info_account .owner != *user.key {
        return Err(Errors::InvalidOwner.into());
    }
	```	
	
	The above code would ensure that only the owner of the account is the only one who could mutate the account 

---

## Final Thoughts

This a smart contract(program) that processes and persists address data, it helped me understand the logical flow of data and how its stored in the address accounts. 
