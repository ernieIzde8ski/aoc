#[macro_export]
macro_rules! read_input {
    () => {
        $crate::read_input!(impl format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")), "expected input.txt")
    };
    ($s:expr) => {
        $crate::read_input!(impl $s, "expected an input file")
    };
    (impl $s:expr, $m:literal) => {
        std::fs::read_to_string($s).expect($m)
    }
}
