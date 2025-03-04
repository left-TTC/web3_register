use solana_program::{blake3::hashv, pubkey::Pubkey};
//a validation library for checking the validity of functions
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use spl_token::state::Account;
use crate::constants::{VAULT_ACCOUNT, ROOT_DOMAIN_ACCOUNT};
use spl_name_service::state::{get_seeds_and_key, HASH_PREFIX};

pub struct Check{}

impl Check {
    pub fn check_accout_key(account: &AccountInfo/*, key: &Pubkey*/) -> bool{
        return true;
    }

    pub fn check_account_owner(account: &AccountInfo/*, key: &Pubkey*/) -> bool{
        return true;
    }
    
    pub fn check_signer(account: &AccountInfo) ->bool{
        return true;
    }

    //in SNS-register,this function return a Account,it's unnecessary
    pub fn check_vault_token_account_owner(account: &AccountInfo) -> Result<(), ProgramError>{
        //Check if the account is controlled by the spl_token::ID program
        if !Check::check_account_owner(account /*&spl_token::ID*/){
            return Err(ProgramError::InvalidArgument);
        }
        //Parse the account data (i.e. binary data) and convert it into an Account structure
        let token_account = Account::unpack_from_slice(&account.data.borrow())?;
        //Check if funds are from the vault
        if token_account.owner != VAULT_ACCOUNT  {
            return Err(ProgramError::IllegalOwner);
        }
        Ok(())
    }
}



//calculate the hash value of the domain name
pub fn get_hashed_name(domain_name: &str) -> Vec<u8> {
    //HASH_PREFIX is a fixed prefix string: SPL Name Service
    hashv(&[(HASH_PREFIX.to_owned() + domain_name).as_bytes()])
        //transfer to &[u8]
        .as_ref()
        //transfer to vec[u8]
        .to_vec()
}

//Confirm whether the domain name is correctly bound to the domain name account to be bound
pub fn get_name_key(domain_name: &str, parent: Option<&Pubkey>) -> Result<Pubkey, ProgramError>{
    let hashed_name = get_hashed_name(domain_name);
    //calculate the name account's PDA
    let (name_account_key, _) = get_seeds_and_key(
        &spl_name_service::id(),
        hashed_name,
        None,
        parent.map_or(Some(&ROOT_DOMAIN_ACCOUNT), Some),
    );
    Ok(name_account_key)
}
