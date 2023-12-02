#[macro_export]
macro_rules! read_input {
    () => {
        match cfg!(debug_assertions) {
            true => {
                eprintln!("Reading sample input!");
                $crate::read_input!("input.sample.txt")
            }
            false => $crate::read_input!("input.txt"),
        }
    };
    ($s:literal) => {
        std::fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), $s))
            .expect(&format!("reading from {}", $s))
    };
}

#[macro_export]
macro_rules! benchmark {
    ($code:expr) => {
        $crate::benchmark!(10_000, $code);
    };

    ($loops:literal, $code:expr) => {{
        let before = std::time::Instant::now();
        for _ in 0..($loops) {
            $code;
        }
        let elapsed = before.elapsed();
        eprintln!("Iterations:   {}", $loops);
        eprintln!("Time elapsed: {:.3?}", elapsed);
        eprintln!("Average time: {:.3?}", elapsed / $loops);
    }};
}
