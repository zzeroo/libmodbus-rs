#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_int, c_uint, uint8_t, uint16_t, uint32_t};

macro_rules! modbus_enum {
    (pub enum $name:ident { $($variants:tt)* }) => {
        #[cfg(target_env = "msvc")]
        pub type $name = i32;
        #[cfg(not(target_env = "msvc"))]
        pub type $name = u32;
        modbus_enum!(gen, $name, 0, $($variants)*);
    };
    (pub enum $name:ident: $t:ty { $($variants:tt)* }) => {
        pub type $name = $t;
        modbus_enum!(gen, $name, 0, $($variants)*);
    };
    (gen, $name:ident, $val:expr, $variant:ident, $($rest:tt)*) => {
        pub const $variant: $name = $val;
        modbus_enum!(gen, $name, $val+1, $($rest)*);
    };
    (gen, $name:ident, $val:expr, $variant:ident = $e:expr, $($rest:tt)*) => {
        pub const $variant: $name = $e;
        modbus_enum!(gen, $name, $e+1, $($rest)*);
    };
    (gen, $name:ident, $val:expr, ) => {}
}

pub enum _modbus {}
pub type modbus_t = _modbus;

#[repr(C)]
pub struct modbus_mapping_t {
    nb_bits: c_int,
    start_bits: c_int,
    nb_input_bits: c_int,
    start_input_bits: c_int,
    nb_input_registers: c_int,
    start_input_registers: c_int,
    nb_registers: c_int,
    start_registers: c_int,
    tab_bits: *mut uint8_t,
    tab_input_bits: *mut uint8_t,
    tab_input_registers: *mut uint16_t,
    tab_registers: *mut uint16_t,
}

modbus_enum! {
    pub enum modbus_error_recovery_mode {
        MODBUS_ERROR_RECOVERY_NONE          = 0,
        MODBUS_ERROR_RECOVERY_LINK          = (1<<1),
        MODBUS_ERROR_RECOVERY_PROTOCOL      = (1<<2),
    }
}


// modbus.h
#[link(name = "modbus")]
extern {
    pub fn modbus_set_slave(ctx: *mut modbus_t, slave: c_int) -> c_int;
    pub fn modbus_set_error_recovery(ctx: *mut modbus_t, error_recovery: modbus_error_recovery_mode) -> c_int;

    pub fn modbus_set_socket(ctx: *mut modbus_t, socket: c_int) -> c_int;
    pub fn modbus_get_socket(ctx: *mut modbus_t) -> c_int;

    pub fn modbus_get_response_timeout(ctx: *mut modbus_t, to_sec: *mut uint32_t, to_usec: *mut uint32_t) -> c_int;
    pub fn modbus_set_response_timeout(ctx: *mut modbus_t, to_sec: uint32_t, to_usec: uint32_t) -> c_int;

    pub fn modbus_get_byte_timeout(ctx: *mut modbus_t, to_sec: *mut uint32_t, to_usec: *mut uint32_t) -> c_int;
    pub fn modbus_set_byte_timeout(ctx: *mut modbus_t, to_sec: uint32_t, to_usec: uint32_t) -> c_int;

    pub fn modbus_get_header_length(ctx: *mut modbus_t) -> c_int;

    pub fn modbus_connect(ctx: *mut modbus_t) -> c_int;
    pub fn modbus_close(ctx: *mut modbus_t);

    pub fn modbus_free(ctx: *mut modbus_t);

    pub fn modbus_flush(ctx: *mut modbus_t) -> c_int;
    pub fn modbus_set_debug(ctx: *mut modbus_t, flag: c_int) -> c_int;

    pub fn modbus_strerror(errnum: c_int) -> *const c_char;

    pub fn modbus_read_bits(ctx: *mut modbus_t, addr: c_int, nb: c_int, dest: *mut uint8_t) -> c_int;
    pub fn modbus_read_input_bits(ctx: *mut modbus_t, addr: c_int, nb: c_int, dest: *mut uint8_t) -> c_int;
    pub fn modbus_read_registers(ctx: *mut modbus_t, addr: c_int, nb: c_int, dest: *mut uint16_t) -> c_int;
    pub fn modbus_read_input_registers(ctx: *mut modbus_t, addr: c_int, nb: c_int, dest: *mut uint16_t) -> c_int;
    pub fn modbus_write_bit(ctx: *mut modbus_t, coil_addr: c_int, status: c_int) -> c_int;
    pub fn modbus_write_register(ctx: *mut modbus_t, reg_addr: c_int, value: c_int) -> c_int;
    pub fn modbus_write_bits(ctx: *mut modbus_t, addr: c_int, nb: c_int, data: *const uint8_t) -> c_int;
    pub fn modbus_write_registers(ctx: *mut modbus_t, addr: c_int, nb: c_int, data: *const uint16_t) -> c_int;
    pub fn modbus_mask_write_register(ctx: *mut modbus_t, addr: c_int, and_mask: uint16_t, or_mask: uint16_t) -> c_int;
    pub fn modbus_write_and_read_registers(ctx: *mut modbus_t, write_addr: c_int, write_nb: c_int,
                                                   src: *const uint16_t, read_addr: c_int, read_nb: c_int,
                                                   dest: *mut uint16_t) -> c_int;
    pub fn modbus_report_slave_id(ctx: *mut modbus_t, max_dest: c_int, dest: *mut uint8_t) -> c_int;

    pub fn modbus_mapping_new_start_address(
        start_bits: c_uint, nb_bits: c_uint,
        start_input_bits: c_uint, nb_input_bits: c_uint,
        start_registers: c_uint, nb_registers: c_uint,
        start_input_registers: c_uint, nb_input_registers: c_uint) -> *mut modbus_mapping_t;

    pub fn modbus_mapping_new(nb_bits: c_int, nb_input_bits: c_int, nb_registers: c_int, nb_input_registers: c_int)  -> *mut modbus_mapping_t;
    pub fn modbus_mapping_free(mb_mapping: *mut modbus_mapping_t);

    pub fn modbus_send_raw_request(ctx: *mut modbus_t, raw_req: *mut uint8_t, raw_req_length: c_int) -> c_int;

    pub fn modbus_receive(ctx: *mut modbus_t, req: *mut uint8_t) -> c_int;

    pub fn modbus_receive_confirmation(ctx: *mut modbus_t, rsp: *mut uint8_t) -> c_int;

    pub fn modbus_reply(ctx: *mut modbus_t, req: *const uint8_t, req_length: c_int, mb_mapping: *mut modbus_mapping_t) -> c_int;
    pub fn modbus_reply_exception(ctx: *mut modbus_t, req: *const uint8_t, exception_code: c_uint) -> c_int;
}

// modbus-rtu.h
extern {
    pub fn modbus_new_rtu(device: *const c_char, baud: c_int, parity: c_int, data_bit: c_int, stop_bit: c_int) -> *mut modbus_t;
}
