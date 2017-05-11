use errors::*;
use libc::{c_int, c_uint, c_void};
use libmodbus_sys;


/// To handle the mapping of your Modbus data, you must use this struct
///
pub struct ModbusMapping {
    pub modbus_mapping_t: libmodbus_sys::modbus_mapping_t,
}

impl ModbusMapping {
    /// `new` - allocate four arrays of bits and registers
    ///
    /// # Parameters
    ///
    /// * `number_bits` - How many bits sould allocated
    /// * `number_input_bits` - How many bits sould allocated
    /// * `number_registers` - How many registers sould allocated
    /// * `number_input_registers` - How many input registers sould allocated
    ///
    pub fn new(number_bits: c_int, number_input_bits : c_int, number_registers: c_int, number_input_registers: c_int) -> Result<ModbusMapping> {
        unsafe {
            use std::ptr;

            let modbus_mapping_t = libmodbus_sys::modbus_mapping_new(number_bits, number_input_bits , number_registers, number_input_registers);
            match *modbus_mapping_t {
                _ => Ok(ModbusMapping { modbus_mapping_t: *modbus_mapping_t }),
            }
        }
    }
}
