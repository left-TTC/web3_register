//use to delete a domain info and related account
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
/// The required parameters for the `delete` instruction
pub struct Params {}