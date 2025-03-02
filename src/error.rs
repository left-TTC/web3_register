use solana_program::program_error::ProgramError;

//used to Debug
use thiserror::Error;
//we will use this create to transfer custom error to Decodeerror
use num_derive::FromPrimitive;

#[cfg(feature = "Debug")]
use solana_program::decode_error::DecodeError;
#[cfg(feature = "Debug")]
use solana_program::msg;
#[cfg(feature = "Debug")]
use solana_program::program_error::PrintProgramError;

//clone: Avoiding mistakes caused by ownership
//Debug: allow to print err
//Eq,partiqlEq: allow to equal
//FromPrimitive: transfer custom err to decodeerroe
//Error: this is PrintProgramError,allow msg! to print Error
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
/* Rust allows you to use the #[derive(...)] macro on a struct or enum */
/* to let the compiler automatically implement some common traits.  */

pub enum Error {
    #[error("Overflow")]
    Overflow = 0,
    #[error("The domain name is already registered")]
    AlreadyRegistered = 1,
}

//Convert custom error types to Solana's standard error types
//impl <T> for type: Implementing a trait for a type
impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        //Coercion
        ProgramError::Custom(e as u32)
    }
}

#[cfg(feature = "Debug")]
//use "Debug" feature to ensure we won't compile it in our final program
//DecodeError: Returns a string representing the name of the error type.
//Debug use
//function: implement DecodeError trait for Error
impl<T> DecodeError<T> for Error {
    //method to ruturn the name of current error type
    fn type_of() -> &'static str {
        "CustomError"
    }
}

#[cfg(feature = "Debug")]
//PrintProgramError: trait in solana,print error info when Error occured
//function: make struct Error can customize error message,and printed by msg
//impl trait for struct
impl PrintProgramError for Error {
    //&Self means it is Error's method,can use self to match different errors 
    fn print<E>(&self)
        //Valid throughout the program life cycle
        //Satisfy other traits
        where
            E: 'static + std::error::Error + solana_program::decode_error::DecodeError<E> + PrintProgramError + num_traits::FromPrimitive {
        match self {
            Error::Overflow =>{
                msg!("Error: Numerical overflow")
            }
            Error::AlreadyRegistered =>{
                msg!("Error: The domain name is already registered")
            }
        }
    }
}



