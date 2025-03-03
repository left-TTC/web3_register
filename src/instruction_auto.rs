
use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq , FromPrimitive)]
pub enum ProgramInstruction{
    //regiser a web3 domain
    CREATE,
    //deprecate a domain
    DELETE,
    //tranfer a's domain to b's account
    TRANSFER,
    //retrun Pubkey by check web3 domain
    FINDUSER,
    //retrun domain of account
    FINDDOMAIN,
}

