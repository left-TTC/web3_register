
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg,
                     pubkey::Pubkey, program_error::ProgramError};
use num_traits::FromPrimitive;
use crate::instruction_auto::ProgramInstruction;


pub struct Processor{}

impl Processor{ 
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
        ) -> ProgramResult{
        #[cfg(feature = "Debug")]
        msg!("process start work");

        //use program_instruction's override FromPrimitive
        //get the instruction's frist byte to determine the instruction type
        //instruction is always a byte stream
        let instruction_type = FromPrimitive::from_u8(instruction_data[0])
            //this is a option method:
            //OK: return the result
            //None: return error
            .ok_or(ProgramError::InvalidInstructionData)?;
        //slice operation,we use this line to get the instruction info
        let instruction_data = &instruction_data[1..];

        #[cfg(feature = "Debug")]
        msg!("Instruction unpacked");

        match instruction_type {
            ProgramInstruction::REGISTER =>{

            }
            ProgramInstruction::DELETE =>{

            }
            ProgramInstruction::FINDDOMAIN =>{

            }
            ProgramInstruction::FINDUSER =>{

            }
            ProgramInstruction::TRANSFER =>{

            }
        }
        Ok(())
    }
} 