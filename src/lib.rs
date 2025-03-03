use bonfida_utils::declare_id_with_central_state;

pub mod entrypoint;
pub mod error;
pub mod processor;
pub mod instruction_auto;
pub mod cip;
pub mod utils;

//declare an ID and create a central state account
//this is global para,by central_state::KEY to use iit
#[cfg(not(feature = "devnet"))]
declare_id_with_central_state!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
//use devnet
#[cfg(feature = "devnet")]
declare_id_with_central_state!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");

#[cfg(not(feature = "devnet"))]

//define various important constants
pub mod constants{
    use solana_program::{pubkey, pubkey::Pubkey};
    use phf::phf_map;

    //use to be the vault
    pub const VAULT_ACCOUNT: Pubkey = pubkey!("vault_account");
    //domain account pubkey
    pub const ROOT_DOMAIN_ACCOUNT: Pubkey = pubkey!("root_domain");
    //pyth account
    pub const PYTH_MAPPING_ACCOUNT: Pubkey = pubkey!("pyth_account");
    //SNS—register create a whitelist to get referrer
    /*Should our recommendation mechanism be more advanced?*/
    pub const REFERRER_WHITELIST: Pubkey = pubkey!("referrer");
    //SNS-register reward referrers proportionally
    /*should our reward mechanism be more advanced?*/
    
    //phf is Perfect Hash Map Library
    //static global
    //the u8 is the number of digits after the decimal point
    pub static TOKENS_SYM_MINT_DECIMALS: phf::Map<&'static str, (Pubkey, u8)> = phf_map!{
        "USDC" => (pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),6),
        "USDT" => (pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"),6),
        "SOL"  => (pubkey!("So11111111111111111111111111111111111111112"),9),
        "PYTH" => (pubkey!("HZ1JovNiVvGrGNiiYvEozEVgZ58xaU3RKwX8eACQBCt3"), 6),
        "FIDA" => (pubkey!("EchesyfXePKdLtoiZSL8pBe8Myagyy8ZRqsACNCFGnvp"),6),
        //"WAF" => (pubkey!("our_coin_pubkey"),6),
    };
    //Next, these are the SNS-register fee policies
    //The percentage of fees charged at each level (or "tiers")
        //pub const FEES: &[u64] = &[500, 300, 200, 150, 100];
    //The number of FIDA tokens required for different "discount accounts"
        //pub const FEE_TIERS: [u64; 4] = [500_000_000, 5_000_000_000, 20_000_000_000, 40_000_000_000];
    //the token creater
        //pub const FIDA_MINT: Pubkey = pubkey!("EchesyfXePKdLtoiZSL8pBe8Myagyy8ZRqsACNCFGnvp");
    //the pubkey of administrator
        //pub const ADMIN: Pubkey = pubkey!("VBx642K1hYGLU5Zm1CHW1uRXAtFgxN5mRqyMcXnLZFW");
    //the pubkey of auction program
        //pub const AUCTION_PROGRAM_ID: Pubkey = pubkey!("AVWV7vdWbLqXiLKFaP19GhYurhwxaLp2qRBSjT5tR5vT");
    //Public keys associated with collectibles
        //pub const WOLVES_COLLECTION: Pubkey = pubkey!("Dw74YSxTKVXsztPm3TmwbnfLK8KVaCZw69jVu4LE6uJe");
    //Public key representing the metadata of the WOLVES_COLLECTION
        //pub const WOLVES_COLLECTION_METADATA: Pubkey = pubkey!("72aLKvXeV4aansAQtxKymeXDevT5ed6sCuz9iN62ugPT");
    //special discount map
}

