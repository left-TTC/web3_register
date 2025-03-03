//a validation library for checking the validity of functions
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

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
}



