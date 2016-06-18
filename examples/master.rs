extern crate libc;
use libc::{c_char, c_int, uint8_t, uint16_t, uint32_t, c_void, timeval, c_uint, ssize_t, fd_set};

pub const FALSE: c_int = 0;
pub const TRUE: c_int = 1;
pub const OFF: c_int = 0;
pub const ON: c_int = 1;

macro_rules! modbus_enum {
    (pub enum $name:ident { $($variants:tt)* }) => {
        #[allow(non_camel_case_types)]
        #[cfg(target_env = "msvc")]
        pub type $name = i32;
        #[allow(non_camel_case_types)]
        #[cfg(not(target_env = "msvc"))]
        pub type $name = u32;
        modbus_enum!(gen, $name, 0, $($variants)*);
    };
    (pub enum $name:ident: $t:ty { $($variants:tt)* }) => {
        #[allow(non_camel_case_types)]
        pub type $name = $t;
        modbus_enum!(gen, $name, 0, $($variants)*);
    };
    (gen, $name:ident, $val:expr, $variant:ident, $($rest:tt)*) => {
        #[allow(non_camel_case_types)]
        pub const $variant: $name = $val;
        modbus_enum!(gen, $name, $val+1, $($rest)*);
    };
    (gen, $name:ident, $val:expr, $variant:ident = $e:expr, $($rest:tt)*) => {
        #[allow(non_camel_case_types)]
        pub const $variant: $name = $e;
        modbus_enum!(gen, $name, $e+1, $($rest)*);
    };
    (gen, $name:ident, $val:expr, ) => {}
}
modbus_enum! {
    pub enum  modbus_backend_type_t {
        _MODBUS_BACKEND_TYPE_RTU=0,
        _MODBUS_BACKEND_TYPE_TCP,
    }
}

modbus_enum! {
    pub enum modbus_error_recovery_mode {
        MODBUS_ERROR_RECOVERY_NONE          = 0,
        MODBUS_ERROR_RECOVERY_LINK          = (1<<1),
        MODBUS_ERROR_RECOVERY_PROTOCOL      = (1<<2),
    }
}

modbus_enum! {
    pub enum msg_type_t {
        MSG_INDICATION,
        MSG_CONFIRMATION,
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct modbus_backend_t {
    backend_type: c_uint,
    header_length: c_uint,
    checksum_length: c_uint,
    max_adu_length: c_uint,
    set_slave: extern fn(ctx: *mut modbus_t, slave: c_int) -> c_int,
    build_request_basis: extern fn(ctx: *mut modbus_t, function: c_int, addr: c_int,
                                nb: c_int, req: *mut uint8_t) -> c_int,
    build_response_basis: extern fn(sft: *mut sft_t, rsp: *mut uint8_t) -> c_int,
    prepare_response_tid: extern fn(req: *const uint8_t, req_length: *mut c_int) -> c_int,
    send_msg_pre: extern fn(req: *mut uint8_t, req_length: c_int) -> c_int,
    send: extern fn(ctx: *mut modbus_t, req: *const uint8_t, req_length: c_int) -> ssize_t,
    receive: extern fn(ctx: *mut modbus_t, req: *mut uint8_t) -> c_int,
    recv: extern fn(ctx: *mut modbus_t, rsp: *mut uint8_t, rsp_length: c_int) -> ssize_t,
    check_integrity: extern fn(ctx: *mut modbus_t, msg: *mut uint8_t,
                            msg_length: *const c_int) -> c_int,
    pre_check_confirmation: extern fn(ctx: *mut modbus_t, req: *const uint8_t,
                                   rsp: *const uint8_t, rsp_length: c_int) -> c_int,
    connect: extern fn(ctx: *mut modbus_t) -> c_int,
    close: extern fn(ctx: *mut modbus_t),
    flush: extern fn(ctx: *mut modbus_t) -> c_int,
    select: extern fn(ctx: *mut modbus_t, rset: *mut fd_set, tv: *mut timeval, msg_length: c_int) -> c_int,
    free: extern fn(ctx: *mut modbus_t),
}

#[repr(C)]
pub struct sft_t {
    slave: c_int,
    function: c_int,
    t_id: c_int,
}

#[repr(C)]
pub struct _modbus {
    /* Slave address */
    slave: c_int,
    /* Socket or file descriptor */
    s: c_int,
    debug: c_int,
    error_recovery: c_int,
    response_timeout: timeval,
    byte_timeout: timeval,
    backend: *const modbus_backend_t,
    backend_data: *mut c_void,
}

#[allow(non_camel_case_types)]
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

// modbus-rtu.h
pub const MODBUS_RTU_MAX_ADU_LENGTH: c_int = 256;

pub const MODBUS_RTU_RS232: c_int = 0;
pub const MODBUS_RTU_RS485: c_int = 1;

pub const MODBUS_RTU_RTS_NONE: c_int = 0;
pub const MODBUS_RTU_RTS_UP: c_int   = 1;
pub const MODBUS_RTU_RTS_DOWN: c_int = 2;



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


    pub fn modbus_new_rtu(device: *const c_char, baud: c_int, parity: c_char, data_bit: c_int, stop_bit: c_int) -> *mut modbus_t;

    // modbus-rtu.h
    pub fn modbus_rtu_set_serial_mode(ctx: *mut modbus_t, mode: c_int) -> c_int;
    pub fn modbus_rtu_get_serial_mode(ctx: *mut modbus_t) -> c_int;

    pub fn modbus_rtu_set_rts(ctx: *mut modbus_t, mode: c_int) -> c_int;
    pub fn modbus_rtu_get_rts(ctx: *mut modbus_t) -> c_int;

    pub fn modbus_rtu_set_custom_rts(ctx: *mut modbus_t, set_rts: extern fn(ctx: *mut modbus_t, on: c_int)) -> c_int;

    pub fn modbus_rtu_set_rts_delay(ctx: *mut modbus_t, us: c_int) -> c_int;
    pub fn modbus_rtu_get_rts_delay(ctx: *mut modbus_t) -> c_int;

}


fn main() {
    let device: *const c_char = std::ffi::CString::new(std::env::args().nth(1).unwrap()).unwrap().as_ptr();
    let slave_id: i32 = std::env::args().nth(2).unwrap().parse().unwrap();

    unsafe {
        let mut tab_reg = vec![0u16; 32];

        let ctx = modbus_new_rtu(device, 9600, 'N' as c_char, 8, 1);
        modbus_set_slave(ctx, slave_id);
        modbus_set_debug(ctx, TRUE);

        modbus_rtu_set_serial_mode(ctx, MODBUS_RTU_RS485);
        if  modbus_connect(ctx) == -1 {
            modbus_free(ctx);
            panic!("Connection failed!");
        }
        // https://doc.rust-lang.org/std/primitive.pointer.html
        let rc = modbus_read_registers(ctx, 0, 19, tab_reg.as_mut_ptr());
        if rc == -1 {
            println!("Registers could not be read");
        }

        for i in 0..rc {
            println!("register[{}]=[{}] (0x{})", i, &tab_reg[i as usize], &tab_reg[i as usize]);
        }



    }
}
