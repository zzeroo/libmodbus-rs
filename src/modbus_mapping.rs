use failure::Error;
use libc::{c_int, c_uint};
use libmodbus_sys as ffi;


/// To handle the mapping of your Modbus data, you must use this struct
///
#[derive(Debug)]
pub struct ModbusMapping {
    pub modbus_mapping: *mut ffi::modbus_mapping_t,
}

impl ModbusMapping {
    /// `new` - create a new `ModbusMapping` containing four arrays of bits and registers
    ///
    /// The [`new()`](#method.new) function creates a new `ModbusMapping` struct with holds four arrays to store bits,
    /// input bits, registers and input registers.
    /// This function is equivalent to the [`new_start_address()`](#method.new_start_address) function called with all
    /// start addresses to 0.
    /// This struct is convenient to handle requests in a Modbus server/slave.
    ///
    /// # Return values
    ///
    /// The function returns a Result containing the new allocated structure if successful. Otherwise it contains an
    /// Error.
    ///
    /// # Parameters
    ///
    /// * `number_bits`             - How many bits sould allocated
    /// * `number_input_bits`       - How many bits sould allocated
    /// * `number_registers`        - How many registers sould allocated
    /// * `number_input_registers`  - How many input registers sould allocated
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// let modbus_mapping = ModbusMapping::new(500, 500, 500, 500).unwrap();
    /// ```
    pub fn new(number_bits: u32, number_input_bits: u32, number_registers: u32, number_input_registers: u32)
               -> Result<ModbusMapping, Error> {
        unsafe {
            let modbus_mapping =
                ffi::modbus_mapping_new(number_bits as c_int,
                                        number_input_bits as c_int,
                                        number_registers as c_int,
                                        number_input_registers as c_int);
            if modbus_mapping.is_null() {
                bail!(::std::io::Error::last_os_error())
            } else {
                Ok(ModbusMapping { modbus_mapping: modbus_mapping })
            }
        }
    }

    /// `new_start_address` - create a `ModbusMapping` with four arrays of bits and registers accessible from their
    /// starting
    /// addresses
    ///
    /// The [`[`new()`](#method.new)()`](#method.[`new()`](#method.new)) function creates a new `ModbusMapping` struct
    /// containing four arrays to store bits, input bits, registers and inputs registers.
    /// The different starting adresses make it possible to place the mapping at any address in each address space.
    /// This way, you can give access to values stored at high adresses without allocating memory from the address
    /// zero, for eg. to make available registers from 10000 to 10009, you can use:
    ///
    /// ```rust
    /// # extern crate libmodbus_rs;
    /// # use libmodbus_rs::ModbusMapping;
    /// # fn main() {
    /// let mapping = ModbusMapping::new_start_address(0, 0, 0, 0, 10000, 10, 0, 0);
    /// # }
    /// ```
    ///
    /// With this code, only 10 registers ([u16]) are allocated.
    /// If it isn’t necessary to allocate an array for a specific type of data, you can pass a `0` in argument.
    /// This function is convenient to handle requests in a Modbus server/slave.
    ///
    /// # Return values
    ///
    /// The function returns a Result containing the new allocated structure if successful. Otherwise it contains an
    /// Error.
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
    /// let modbus_mapping = ModbusMapping::new_start_address(0, 0, 0, 0, 10000, 10, 0, 0).unwrap();
    /// ```
    pub fn new_start_address(start_bits: u16, number_bits: u16, start_input_bits: u16, number_input_bits: u16,
                             start_registers: u16, number_registers: u16, start_input_registers: u16,
                             number_input_registers: u16)
                             -> Result<ModbusMapping, Error> {
        unsafe {
            let modbus_mapping = ffi::modbus_mapping_new_start_address(start_bits as c_uint,
                                                                       number_bits as c_uint,
                                                                       start_input_bits as c_uint,
                                                                       number_input_bits as c_uint,
                                                                       start_registers as c_uint,
                                                                       number_registers as c_uint,
                                                                       start_input_registers as c_uint,
                                                                       number_input_registers as c_uint);
            if modbus_mapping.is_null() {
                bail!(::std::io::Error::last_os_error())
            } else {
                Ok(ModbusMapping { modbus_mapping: modbus_mapping })
            }
        }
    }

    /// `free` - free a `ModbusMapping` structure
    ///
    /// The function shall free the four arrays of `mb_mapping_t` structure and finally the mb_mapping_t referenced by
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
            ffi::modbus_mapping_free(self.modbus_mapping);
        }
    }

    // TODO: Add better documentation
    /// `get_bits` - returns a slice constructed from the `bits` and `nb_bits` member of the orig. `ModbusMapping`
    /// struct
    ///
    /// `tab_bits` is an pointer, and this function returns a valid Rust slice to work with.
    ///
    /// # Return value
    ///
    /// This function returns an immutable slice of `u8`'s
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();
    ///
    /// assert_eq!(modbus_mapping.get_bits(), [0u8, 0, 0, 0, 0])
    /// ```
    pub fn get_bits(&self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts((*self.modbus_mapping).tab_bits, (*self.modbus_mapping).nb_bits as usize)
        }
    }

    // TODO: Add better documentation
    /// `get_bits_mut` - returns a mutable slice constructed from the `tab_bits` and `nb_bits` member of the orig.
    /// `ModbusMapping` struct
    ///
    /// `tab_bits` is an pointer, and this function returns a valid, mutable Rust slice to work with.
    ///
    /// # Return value
    ///
    /// This function returns an mutable slice of `u8`'s
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();
    ///
    /// assert_eq!(modbus_mapping.get_bits_mut(), [0u8, 0, 0, 0, 0])
    /// ```
    pub fn get_bits_mut(&self) -> &mut [u8] {
        unsafe {
            ::std::slice::from_raw_parts_mut((*self.modbus_mapping).tab_bits, (*self.modbus_mapping).nb_bits as usize)
        }
    }

    // TODO: Add better documentation
    /// `get_input_bits` - returns a slice constructed from the `tab_input_bits` and `nb_input_bits` member of the
    /// orig. `ModbusMapping` struct
    ///
    /// `tab_input_bits` is an pointer, and this function returns a valid Rust slice to work with.
    ///
    /// # Return value
    ///
    /// This function returns an immutable slice of `u8`'s
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();
    ///
    /// assert_eq!(modbus_mapping.get_input_bits(), [0u8, 0, 0, 0, 0])
    /// ```
    pub fn get_input_bits(&self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts((*self.modbus_mapping).tab_input_bits,
                                         (*self.modbus_mapping).nb_input_bits as usize)
        }
    }

    // TODO: Add better documentation
    /// `get_input_bits_mut` - returns a mutable slice constructed from the `tab_input_bits` and `nb_input_bits` member
    /// of the orig. `ModbusMapping` struct
    ///
    /// `tab_bits` is an pointer, and this function returns a valid, mutable Rust slice to work with.
    ///
    /// # Return value
    ///
    /// This function returns an mutable slice of `u8`'s
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();
    ///
    /// assert_eq!(modbus_mapping.get_input_bits_mut(), [0u8, 0, 0, 0, 0])
    /// ```
    pub fn get_input_bits_mut(&self) -> &mut [u8] {
        unsafe {
            ::std::slice::from_raw_parts_mut((*self.modbus_mapping).tab_input_bits,
                                             (*self.modbus_mapping).nb_input_bits as usize)
        }
    }

    // TODO: Add better documentation
    /// `get_input_registers` - returns a slice constructed from the `tab_input_registers` and `nb_input_registers`
    /// member of the orig. `ModbusMapping` struct
    ///
    /// `tab_input_registers` is an pointer, and this function returns a valid Rust slice to work with.
    ///
    /// # Return value
    ///
    /// This function returns an immutable slice of `u8`'s
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();
    ///
    /// assert_eq!(modbus_mapping.get_input_registers(), [0u16, 0, 0, 0, 0])
    /// ```
    pub fn get_input_registers(&self) -> &[u16] {
        unsafe {
            ::std::slice::from_raw_parts((*self.modbus_mapping).tab_input_registers,
                                         (*self.modbus_mapping).nb_input_registers as usize)
        }
    }

    // TODO: Add better documentation
    /// `get_input_registers_mut` - returns a mutable slice constructed from the `tab_input_registers` and
    /// `nb_input_registers` member of the orig. `ModbusMapping` struct
    ///
    /// `tab_input_registers` is an pointer, and this function returns a valid, mutable Rust slice to work with.
    ///
    /// # Return value
    ///
    /// This function returns an mutable slice of `u16`'s
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();
    ///
    /// assert_eq!(modbus_mapping.get_input_registers_mut(), [0u16, 0, 0, 0, 0])
    /// ```
    pub fn get_input_registers_mut(&self) -> &mut [u16] {
        unsafe {
            ::std::slice::from_raw_parts_mut((*self.modbus_mapping).tab_input_registers,
                                             (*self.modbus_mapping).nb_input_registers as usize)
        }
    }

    // TODO: Add better documentation
    /// `get_registers` - returns a slice constructed from the `tab_registers` and `nb_registers` member of the orig.
    /// `ModbusMapping` struct
    ///
    /// `tab_registers` is an pointer, and this function returns a valid Rust slice to work with.
    ///
    /// # Return value
    ///
    /// This function returns an immutable slice of `u8`'s
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();
    ///
    /// assert_eq!(modbus_mapping.get_registers(), [0u16, 0, 0, 0, 0])
    /// ```
    pub fn get_registers(&self) -> &[u16] {
        unsafe {
            ::std::slice::from_raw_parts((*self.modbus_mapping).tab_registers,
                                         (*self.modbus_mapping).nb_registers as usize)
        }
    }

    // TODO: Add better documentation
    /// `get_registers_mut` - returns a mutable slice constructed from the `tab_registers` and `nb_registers` member of
    /// the orig. `ModbusMapping` struct
    ///
    /// `tab_registers` is an pointer, and this function returns a valid, mutable Rust slice to work with.
    ///
    /// # Return value
    ///
    /// This function returns an mutable slice of `u16`'s
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();
    ///
    /// assert_eq!(modbus_mapping.get_registers_mut(), [0u16, 0, 0, 0, 0])
    /// ```
    pub fn get_registers_mut(&self) -> &mut [u16] {
        unsafe {
            ::std::slice::from_raw_parts_mut((*self.modbus_mapping).tab_registers,
                                             (*self.modbus_mapping).nb_registers as usize)
        }
    }
}

impl Drop for ModbusMapping {
    fn drop(&mut self) {
        self.free()
    }
}
