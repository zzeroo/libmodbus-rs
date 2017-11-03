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
        SlaveFOrServerFailure {
            description("Unrecoverable error occurred while slave was attempting to perform requested action")
            display("Slave device or server failure")
        }
        TooManyData {
            description("Too many data requested")
            display("Too many data")
        }
        TooManyData2(desc: &'static str) {
            description("Too many data")
            display("Too many data: '{}'", desc)
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
    }

}
