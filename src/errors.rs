error_chain!{
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
    }

    errors {
        ILFU {
            description("Illegal function")
            display("Illegal function")
        }
        ILADD {
            description("Illegal data address")
            display("Illegal data address")
        }
        ILVAL {
            description("Illegal data value")
            display("Illegal data value")
        }
        SFAIL {
            description("Slave device or server failure")
            display("Slave device or server failure")
        }
        ACK {
            description("Acknowledge")
            display("Acknowledge")
        }
        SBUSY {
            description("Slave device or server is busy")
            display("Slave device or server is busy")
        }
        NACK {
            description("Negative acknowledge")
            display("Negative acknowledge")
        }
        MEMPAR {
            description("Memory parity error")
            display("Memory parity error")
        }
        GPATH {
            description("Gateway path unavailable")
            display("Gateway path unavailable")
        }
        GTAR {
            description("Target device failed to respond")
            display("Target device failed to respond")
        }
        BADCRC {
            description("Invalid CRC")
            display("Invalid CRC")
        }
        BADDATA {
            description("Invalid data")
            display("Invalid data")
        }
        BADEXC {
            description("Invalid exception code")
            display("Invalid exception code")
        }
        MDATA {
            description("Too many data")
            display("Too many data")
        }
        BADSLAVE {
            description("Response not from requested slave")
            display("Response not from requested slave")
        }
        IncompatibleAPI {
            description("libmodbus API incompatible response")
            display("libmodbus API incompatible response")
        }
        TooManyData(desc: &'static str) {
            description("Too many data")
            display("Too many data: '{}'", desc)
        }
        InvalidSlaveID(id: u8) {
            description("Invalid slave ID")
            display("Invalid slave ID: '{}'", id)
        }
        InvalidParameter(desc: &'static str) {
            description("Invalid parameter given")
            display("Invalid parameter given: '{}'", desc)
        }
    }

}
