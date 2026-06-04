use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {//explanation of it returns Vec<u8> and takes a hex string reference

    let hex_str = hex_str.trim();// this remove space from hex_str and return it in hex_str

    //this perform a modulus of 2 and if return a value that is not 0 its sees it as an odd number 
    if hex_str.len() % 2 != 0 {
        return Err("Hex string is not even".to_string());
    }

    hex_str
        .as_bytes() //this convers it to byts
        .chunks(2) // this slice it into 2
        .map(|chunk| {
            let s = std::str::from_utf8(chunk)
                .map_err(|_| "Invalid UTF-8 in hex string".to_string())?;

            u8::from_str_radix(s, 16).map_err(|_| format!("invalid hex byte: {}", s))// after slicing it pass the values as base 16
        })
        .collect() // collect all the rsult into a single result into a byte vector
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().rev().cloned().collect() //based on research its needed so as to give a display version of this TXID
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    encode(bytes) // the encode comes from the hex crate and it is imported
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    decode(hex) // this does the opposite of encode
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] { // this returns the fix size array of 4 bytes 
    num.to_le_bytes()// this line of code changes the integer to little-endian bytes
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {// takes a borrowed str and it returns valid satoshi amount
    input
        .trim() //this remove the space
        .parse::<u64>()
        .map_err(|_| "Invalid satoshi amount".to_string())
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    if script.starts_with(&[0x76, 0xa9, 0x14]) {
        ScriptType::P2PKH
    } else if script.starts_with(&[0x00, 0x14]) {
        ScriptType::P2WPKH
    } else {
        ScriptType::Unknown //any unrecognized scripts maps to unknown
    }
}

pub struct Outpoint(pub String, pub u32);

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    let len = script[1] as usize;
    &script[2..2 + len]
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    *balance -= fee;
}

pub fn move_txid(txid: String) -> String {
    format!("txid: {}", txid)
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            _ => Err(format!("Invalid opcode: {:#04x}", byte)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    utxo
}
