#![allow(dead_code)]

pub const SERVER_ID: u8 = 17;
pub const INVALID_SERVER_ID: i32 = 18;

pub const BITS_ADDRESS: u16 = 0x130;
pub const BITS_NB: u16 = 0x25;
pub const BITS_TAB: &[u8] = &[0xCD, 0x6B, 0xB2, 0x0E, 0x1B];

pub const INPUT_BITS_ADDRESS: u16 = 0x1C4;
pub const INPUT_BITS_NB: u16 = 0x16;
pub const INPUT_BITS_TAB: &[u8] = &[0xAC, 0xDB, 0x35];

pub const REGISTERS_ADDRESS: u16 = 0x160;
pub const REGISTERS_NB: u16 = 0x3;
pub const REGISTERS_NB_MAX: u16 = 0x20;
pub const REGISTERS_TAB: &[u16] = &[0x022B, 0x0001, 0x0064];

// Raise a manual exception when this address is used for the first byte
pub const REGISTERS_ADDRESS_SPECIAL: u16 = 0x170;
// The response of the server will contains an invalid TID or slave
pub const REGISTERS_ADDRESS_INVALID_TID_OR_SLAVE: u16 = 0x171;
// The server will wait for 1 second before replying to test timeout
pub const REGISTERS_ADDRESS_SLEEP_500_MS: u16 = 0x172;
// The server will wait for 5 ms before sending each byte
pub const REGISTERS_ADDRESS_BYTE_SLEEP_5_MS: u16 = 0x173;

// If the following value is used, a bad response is sent.
// It's better to test with a lower value than
// REGISTERS_NB_POINTS to try to raise a segfault.
pub const REGISTERS_NB_SPECIAL: u16 = 0x2;

pub const INPUT_REGISTERS_ADDRESS: u16 = 0x108;
pub const INPUT_REGISTERS_NB: u16 = 0x1;
pub const INPUT_REGISTERS_TAB: &[u16] = &[0x000A];

pub const REAL: f32 = 123456.00;

pub const IREAL_ABCD: u32 = 0x0020F147;
pub const IREAL_DCBA: u32 = 0x47F12000;
pub const IREAL_BADC: u32 = 0x200047F1;
pub const IREAL_CDAB: u32 = 0xF1470020;

// pub const IREAL_ABCD: u32 = 0x47F12000);
// pub const IREAL_DCBA: u32 = 0x0020F147;
// pub const IREAL_BADC: u32 = 0xF1470020;
// pub const IREAL_CDAB: u32 = 0x200047F1;

#[derive(Eq, PartialEq)]
pub enum Backend {
    TCP,
    TCPPI,
    RTU,
}


fn main() {}
