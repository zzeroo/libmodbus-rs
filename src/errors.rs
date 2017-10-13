error_chain!{
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
    }

    errors {
        InvalidSlaveID(id: u8) {
            description("Invalid slave ID")
            display("Invalid slave ID: '{}'", id)
        }
    }

}
