//! None type dependend tool function to work with the modbus data

pub use modbus::{set_bits_from_byte, set_bits_from_bytes, get_byte_from_bits, get_float_abcd, set_float_abcd,
                       get_float_badc, set_float_badc, get_float_cdab, set_float_cdab, get_float_dcba, set_float_dcba};

pub use failure::Error;
