
/// Modbus function codes
///
/// Documentation source: https://en.wikipedia.org/wiki/Modbus
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FunctionCode {
    /// 0x01 Read Coils
    READ_COILS = 1,
    /// 0x02 Read Discrete Inputs
    READ_DISCRETE_INPUTS = 2,
    /// 0x03 Read Multiple Holding Registers
    READ_HOLDING_REGISTERS = 3,
    /// 0x04 Read Input Registers
    READ_INPUT_REGISTERS = 4,
    /// 0x05 Write Single Coil
    WRITE_SINGLE_COIL = 5,
    /// 0x06 Write Single Holding Register
    WRITE_SINGLE_REGISTER = 6,
    /// 0x07 Read Exception Status
    READ_EXCEPTION_STATUS = 7,
    /// 0x15 Write Multiple Coils
    WRITE_MULTIPLE_COILS = 15,
    /// 0x16 Write Multiple Holding Registers
    WRITE_MULTIPLE_REGISTERS = 16,
    /// 0x17 Report Slave ID
    REPORT_SLAVE_ID = 17,
    /// 0x22 Mask Write Register
    MASK_WRITE_REGISTER = 22,
    /// 0x23 Read/Write Multiple Registers
    WRITE_AND_READ_REGISTERS = 23,
}
