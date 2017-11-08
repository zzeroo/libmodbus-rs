error_chain!{

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
    }

    errors {
        IllegalFunction {
            description("Function code received in the query is not recognized or allowed by slave")
            display("Illegal function")
        }
        IllegalDataAddress {
            description("Data address of some or all the required entities are not allowed or do not exist in slave")
            display("Illegal data address")
        }
        IllegalDataValue {
            description("Value is not accepted by slave")
            display("Illegal data value")
        }
        SlaveDeviceOrServerFailure {
            description("Unrecoverable error occurred while slave was attempting to perform requested action")
            display("Slave device or server failure")
        }
        Acknowledge {
            description("Acknowledge")
            display("Acknowledge")
        }
        SlaveDeviceOrServerIsBusy {
            description("Slave device or server is busy")
            display("Slave device or server is busy")
        }
        NegativeAcknowledge {
            description("Negative acknowledge")
            display("Negative acknowledge")
        }
        MemoryParityError {
            description("Memory parity error")
            display("Memory parity error")
        }
        GatewayPathUnavailable {
            description("Gateway path unavailable")
            display("Gateway path unavailable")
        }
        TargetDeviceFailedToRespond {
            description("Target device failed to respond")
            display("Target device failed to respond")
        }
        InvalidCRC {
            description("Invalid CRC")
            display("Invalid CRC")
        }
        InvalidData {
            description("Invalid data")
            display("Invalid data")
        }
        InvalidExceptionCode {
            description("Invalid exception code")
            display("Invalid exception code")
        }
        TooManyData {
            description("Too many data requested")
            display("Too many data")
        }
        ResponseNotFromRequestedSlave {
            description("Response not from requested slave")
            display("Response not from requested slave")
        }
        BADSLAVE {
            description("Response not from requested slave")
            display("Response not from requested slave")
        }
        IncompatibleAPI {
            description("libmodbus API incompatible response")
            display("libmodbus API incompatible response")
        }
        InvalidSlaveID(id: u8) {
            description("Invalid slave ID")
            display("Invalid slave ID: '{}'", id)
        }
        InvalidParameter(desc: &'static str) {
            description("Invalid parameter given")
            display("Invalid parameter given: '{}'", desc)
        }
        UnitTestClientFailure {
            description("Unit test client failed")
            display("Unit test client failed")
        }
    }

}
