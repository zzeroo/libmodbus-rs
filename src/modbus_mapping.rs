use errors::*;
use libc::c_uint;
use libmodbus_sys;
use std::io::Error;


/// To handle the mapping of your Modbus data, you must use this struct
///
#[derive(Debug)]
pub struct ModbusMapping {
    pub modbus_mapping: *mut libmodbus_sys::modbus_mapping_t,
}

impl ModbusMapping {
    /// `new` - allocate four arrays of bits and registers
    ///
    /// # Return values
    ///
    /// The function returns a Result containing the new allocated structure if successful. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `number_bits` - How many bits sould allocated
    /// * `number_input_bits` - How many bits sould allocated
    /// * `number_registers` - How many registers sould allocated
    /// * `number_input_registers` - How many input registers sould allocated
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// let modbus_mapping = ModbusMapping::new(500, 500, 500, 500).unwrap();
    /// ```
    pub fn new(number_bits: i32, number_input_bits: i32, number_registers: i32, number_input_registers: i32)
               -> Result<ModbusMapping> {
        unsafe {
            let modbus_mapping = libmodbus_sys::modbus_mapping_new(number_bits,
                                                                   number_input_bits,
                                                                   number_registers,
                                                                   number_input_registers);
            if modbus_mapping.is_null() {
                bail!(Error::last_os_error())
            } else {
                Ok(ModbusMapping { modbus_mapping: modbus_mapping })
            }
        }
    }

    /// `mapping_new_start_address` - allocate four arrays of bits and registers accessible from their starting addresses
    ///
    /// The modbus_mapping_new_start_address() function shall allocate four arrays to store bits, input bits, registers and inputs registers. The pointers are stored in modbus_mapping_t structure. All values of the arrays are initialized to zero.
    /// The different starting adresses make it possible to place the mapping at any address in each address space. This way, you can give access to values stored at high adresses without allocating memory from the address zero, for eg. to make available registers from 10000 to 10009, you can use:
    /// mb_mapping = modbus_mapping_offset_start_address(0, 0, 0, 0, 10000, 10, 0, 0);
    /// With this code, only 10 registers (uint16_t) are allocated.
    /// If it isn’t necessary to allocate an array for a specific type of data, you can pass the zero value in argument, the associated pointer will be NULL.
    /// This function is convenient to handle requests in a Modbus server/slave.
    ///
    /// # Return value
    ///
    /// The function returns a Result containing the new allocated structure if successful. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `start_bits`              - start address of bits array
    /// * `number_bits`             - How many bits sould allocated
    /// * `start_input_bits`        - start address of input bits array
    /// * `number_input_bits`       - How many bits sould allocated
    /// * `start_registers`         - start address of register array
    /// * `number_registers`        - How many registers sould allocated
    /// * `start_input_registers`   - start address of input register array
    /// * `number_input_registers`  - How many input registers sould allocated
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// let modbus_mapping = ModbusMapping::new_start_address(10, 500, 10, 500, 10, 500, 10, 500).unwrap();
    /// ```
    pub fn new_start_address(start_bits: u16, number_bits: u16,
                             start_input_bits: u16, number_input_bits: u16,
                             start_registers: u16, number_registers: u16,
                             start_input_registers: u16, number_input_registers: u16)
               -> Result<ModbusMapping> {
        unsafe {
            let modbus_mapping = libmodbus_sys::modbus_mapping_new_start_address(start_bits as c_uint, number_bits as c_uint,
                                                                                 start_input_bits as c_uint, number_input_bits as c_uint,
                                                                                 start_registers as c_uint, number_registers as c_uint,
                                                                                 start_input_registers as c_uint, number_input_registers as c_uint);
            if modbus_mapping.is_null() {
                bail!(Error::last_os_error())
            } else {
                Ok(ModbusMapping { modbus_mapping: modbus_mapping })
            }
        }
    }

    /// `mapping_free` - free a modbus_mapping_t structure
    ///
    /// The function shall free the four arrays of mb_mapping_t structure and finally the mb_mapping_t referenced by
    /// mb_mapping.
    ///
    /// **It should not nessesary to call these function. Because rusts drop trait handles that for you!**
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::ModbusMapping;
    /// let mut modbus_mapping = ModbusMapping::new(500, 500, 500, 500).unwrap();
    ///
    /// modbus_mapping.free();
    /// ```
    pub fn free(&mut self) {
        unsafe {
            libmodbus_sys::modbus_mapping_free(self.modbus_mapping);
        }
    }
}

impl Drop for ModbusMapping {
    fn drop(&mut self) {
        self.free()
    }
}
