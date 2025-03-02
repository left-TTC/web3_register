
use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq , FromPrimitive)]
pub enum ProgramInstruction{
    //regiser a web3 domain
    REGISTER,
    //deprecate a domain
    DELETE,
    //tranfer a's domain to b's accout
    TRANSFER,
    //retrun Pubkey by check web3 domain
    FINDUSER,
    //retrun domain of accout
    FINDDOMAIN,
}

