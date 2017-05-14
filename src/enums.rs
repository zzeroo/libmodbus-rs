
/// Modbus protocol exceptions
///
/// Documentation source: https://en.wikipedia.org/wiki/Modbus#Main_Modbus_exception_codes
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Exception {
    /// (1) Illegal Function - Function code received in the query is not recognized or allowed by slave
    ILLEGAL_FUNCTION = 1,
    /// (2) Illegal Data Address - Data address of some or all the required entities are not allowed or do not exist in
    /// slave
    ILLEGAL_DATA_ADDRESS,
    /// (3) Illegal Data Value - Value is not accepted by slave
    ILLEGAL_DATA_VALUE,
    /// (4) Slave Device Failure - Unrecoverable error occurred while slave was attempting to perform requested action
    SLAVE_OR_SERVER_FAILURE,
    /// (5) Acknowledge - Slave has accepted request and is processing it, but a long duration of time is required.
    /// This response is returned to prevent a timeout error from occurring in the master. Master can next issue a Poll
    /// Program Complete message to determine whether processing is completed
    ACKNOWLEDGE,
    /// (6) Slave Device Busy - Slave is engaged in processing a long-duration command. Master should retry later
    SLAVE_OR_SERVER_BUSY,
    /// (7) Negative Acknowledge - Slave cannot perform the programming functions. Master should request diagnostic or
    /// error information from slave
    NEGATIVE_ACKNOWLEDGE,
    /// (8) Memory Parity Error - Slave detected a parity error in memory. Master can retry the request, but service
    /// may be required on the slave device
    MEMORY_PARITY,
    /// (9) Not defined
    NOT_DEFINED,
    /// (10) Gateway Path Unavailable - Specialized for Modbus gateways. Indicates a misconfigured gateway
    GATEWAY_PATH,
    /// (11) Gateway Target Device Failed to Respond - Specialized for Modbus gateways. Sent when slave fails to respond
    GATEWAY_TARGET,
}



/// Modbus function codes
///
/// Documentation source: https://en.wikipedia.org/wiki/Modbus#Supported_function_codes
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

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ErrorRecoveryMode {
    NONE = 0,
    LINK = 2,
    PROTOCOL = 4,
}
