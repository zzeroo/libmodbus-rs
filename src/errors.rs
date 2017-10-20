error_chain!{
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
    }

    errors {
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
    }

}
