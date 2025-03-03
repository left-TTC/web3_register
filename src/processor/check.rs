//a validation library for checking the validity of functions
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;

use spl_token::state::Account;
use crate::constants::{VAULT_ACCOUNT};

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



