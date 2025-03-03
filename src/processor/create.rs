//use to create a domian and account to record the related info about the domain

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::msg;
use std::str::FromStr;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::{next_account_info, AccountInfo},pubkey::Pubkey,entrypoint::ProgramResult,};
use crate::processor::check::Check;
#[cfg(feature = "Debug")]
use solana_program::msg;


#[derive(BorshDeserialize, BorshSerialize, Debug)]
//used to encapsulates the command parameters for creating a domain name account
pub struct Params {
    //this is the name of domain,like f.web3's f 
    pub name: String,
    //the storage space required for domain account
    pub space: u32,
    //this is a optional para
    //referrer's index
    pub referrer_idx_opt: Option<u16>,
}

//Customize the method of processing data
impl Params{
    pub fn get_params(instruction_data: &[u8]) -> Result<Self,ProgramError>{
        let data_str = match String::from_utf8(instruction_data.to_vec()) {
            Ok(data) => data,
            Err(_) => {
                #[cfg(feature = "Debug")]
                msg!("Failed to convert instruction data to UTF-8");
                return Err(ProgramError::InvalidInstructionData);
            }
        };
        //split the data by ","
        let parts: Vec<&str> = data_str.split(',').collect();
        //get domain name from part 1
        let name = parts[0].to_string();
        //get required space from part 2
        let space = match u32::from_str(parts[1]) {
            Ok(v) => v,
            Err(_) => {
                #[cfg(feature = "Debug")]
                msg!("can't read the required space");
                return Err(ProgramError::InvalidInstructionData);
            }
        };
        //get referrer <option>
        let referrer_idx_opt = if parts[2].starts_with("0x"){
            match u16::from_str_radix(&parts[2][2..], 16) {
                Ok(v) => Some(v),
                Err(_) => {
                    #[cfg(feature = "Debug")]
                    msg!("referrer error");
                    return Err(ProgramError::InvalidAccountData);
                }
            }
        }else{
            None
        };
        //return the struct params 
        Ok(Params{
            name,
            space,
            referrer_idx_opt,
        })
    }
}

pub struct Accounts<'a, T>{
    //flag of cpi
    //pubkey of smart contract
    pub naming_service_program: &'a T,
    //the root domain account
    //such as f.web3, the root_domain account should be the pubkey of .web3 account            
    pub root_domain: &'a T,
    //the domain's account
    //solana is possible to perform write operations on this account
    pub name: &'a T,
    //for example: pubkey A have domain f.web3,you can use this account find A by f.web3
    pub reverse_lookup: &'a T,
    //system program account: The core program of the Solana network,can used to create account,transfer balnace
    pub system_program: &'a T,
    //this account used to save core state of contract
    pub central_state: &'a T,
    //buyer account
    pub buyer: &'a T,
    //buyer's token account,used to pay for the domain name,such as USDT account
    pub buyer_token_source: &'a T,
    //Pyth is a decentralized oracle network，use it to query specific market data
    /*should we use the same oracle net to decide our domain price? */
    //maooing account used to get the price of a specific asset (such as Bitcoin, Ethereum, USDT, etc.)
    pub pyth_mapping_acc: &'a T,
    //Used to store metadata related to a specific asset product
    pub pyth_product_acc: &'a T,
    //Accounts storing real-time prices
    pub pyth_price_acc: &'a T,
    //vault account
    //Stores the fees or escrow funds paid by users when registering domain names or other related operations
    pub vault: &'a T,
    //if the contract needs to issue and manage tokens itself
    //we need a spl_token_program
    pub spl_token_program: &'a T,
    //The Rent Sysvar Account is used to store parameters related to the Solana network rental mechanism.
    //Check the current rental fee
    pub rent_sysvar: &'a T,
    /*Do we need to bid on domain names? It's obvious */
    //Stores auction status, current bidding information, and other data related to the auction process
    pub state: &'a T,
    //referrer's account
    pub referrer_account_opt: Option<&'a T>,
}

//extract solana account information from the account array and encapsulate them into accounts structure
//a is the life of account structure,b is the lifetime of type accountinfo
//the reference in AccountInfo<'b> cannot have a shorter lifetime than the reference in Accounts<'a, AccountInfo<'b>>
//means we can't destory the accountInfo in the lifetime of account's lifetime
impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    //parse the accoutinfo and extract it
    pub fn parse(
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Self, ProgramError> {
        //create a iter to traverse the passed in accountinfo array
        let accounts_iter = &mut accounts.iter();
        Ok(Accounts{
            //"?" means if accountinfo does't have the para,return a error 
            naming_service_program: next_account_info(accounts_iter)?,
            root_domain: next_account_info(accounts_iter)?,
            name: next_account_info(accounts_iter)?,
            reverse_lookup: next_account_info(accounts_iter)?,
            system_program: next_account_info(accounts_iter)?,
            central_state: next_account_info(accounts_iter)?,
            buyer: next_account_info(accounts_iter)?,
            buyer_token_source: next_account_info(accounts_iter)?,
            pyth_mapping_acc: next_account_info(accounts_iter)?,
            pyth_product_acc: next_account_info(accounts_iter)?,
            pyth_price_acc: next_account_info(accounts_iter)?,
            vault: next_account_info(accounts_iter)?,
            spl_token_program: next_account_info(accounts_iter)?,
            rent_sysvar: next_account_info(accounts_iter)?,
            state: next_account_info(accounts_iter)?,
            //".ok() transfer it into a option type"
            referrer_account_opt: next_account_info(accounts_iter).ok(),
        })
    }

    //Check whether the account information is legal
    pub fn check(&self) -> Result<(), ProgramError>{
        //check the program id -- fixed
        if !Check::check_accout_key(self.naming_service_program){
            #[cfg(feature = "Debug")]
            msg!("program id is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        //check the root domain: web3 -- fixed
        if !Check::check_accout_key(self.root_domain){
            #[cfg(feature = "Debug")]
            msg!("root_domain is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        //check the solana net -- fixed
        if !Check::check_accout_key(self.system_program){
            #[cfg(feature = "Debug")]
            msg!("solana core id is invalid");
            return Err(ProgramError::InvalidAccountData);
        }//fixed
        if !Check::check_accout_key(self.central_state){
            #[cfg(feature = "Debug")]
            msg!("central_state id is invalid");
            return Err(ProgramError::InvalidAccountData);
        }//fixed
        if !Check::check_accout_key(self.spl_token_program){
            #[cfg(feature = "Debug")]
            msg!("program id is invalid");
            return Err(ProgramError::InvalidAccountData);
        }//fixed
        if !Check::check_accout_key(self.rent_sysvar){
            #[cfg(feature = "Debug")]
            msg!("program id is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if !Check::check_account_owner(self.name){
            #[cfg(feature = "Debug")]
            msg!("domain name error");
            return Err(ProgramError::InvalidAccountData);
        }
        if !Check::check_account_owner(self.vault){
            #[cfg(feature = "Debug")]
            msg!("vault error");
            return Err(ProgramError::InvalidAccountData);
        }
        if !Check::check_account_owner(self.name){
            #[cfg(feature = "Debug")]
            msg!("domain name error");
            return Err(ProgramError::InvalidAccountData);
        }
        if !Check::check_signer(self.buyer){
            #[cfg(feature = "Debug")]
            msg!("buyer;s signature error");
            return Err(ProgramError::InvalidAccountData);
        }
        
        Ok(())
    }
}

//External interface, call this to perform a domain name registration service
pub fn process_create(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: Params,
) -> ProgramResult {
    let accounts = match Accounts::parse(accounts) {
        Ok(accounts) => accounts,
        Err(error) =>{
            #[cfg(feature = "Debug")]
            msg!("accounts parse failed");
            return Err(ProgramError::InvalidArgument); 
        }
    };
    create(program_id, accounts, params)
}

//The function that actually performs the creation operation
//programResult -> Result<ok(),ProgramError>
pub fn create<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: Accounts<'a, AccountInfo<'b>>,
    params: Params,
) -> ProgramResult {
    //check account info
    accounts.check()?;
    //check funding
    Check::check_vault_token_account_owner(accounts.vault)?;
    //Process the requested domain name
    if true {
        #[cfg(feature = "Debug")]
        msg!("Invalid domain");
        return Err(ProgramError::InvalidArgument);
    }
    //

    Ok(())
}




