
use solana_program::{
    account_info::AccountInfo, pubkey::Pubkey, entrypoint::ProgramResult, msg,
};
use solana_program::entrypoint;
use crate::processor::Processor;

#[cfg(feature = "Debug")]
use solana_program::program_error::PrintProgramError;
#[cfg(feature = "Debug")]
use error::Error;
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(WEB3_Start);

pub fn WEB3_Start(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
    ) -> ProgramResult{
    msg!("this is the first step of web3 domian name!");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        
        //Custom Errors,we should transfer it to DecodeError
        #[cfg(feature = "Debug")]
        error.print::<Error>();

        //return err info to requester        
        return Err(error);
    }
    //return the acquired information
    Ok(())
}

