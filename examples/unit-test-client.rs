#[allow(non_camel_case_types)]


enum Backend {
    TCP,
    TCP_RI,
    RTU,
}

struct UnitTestClient;

impl UnitTestClient {
    fn test_server(&self, backend: Backend) {}
}



fn main() {
    assert!(true);
}
