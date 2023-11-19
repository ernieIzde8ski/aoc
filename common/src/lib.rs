#[macro_export]
macro_rules! read_input {
    () => {
        $crate::read_input!(impl match cfg!(debug_assertions) {
            true => {
                eprintln!("Reading sample input!");
                format!("{}/input.sample.txt", env!("CARGO_MANIFEST_DIR"))
            },
            false => format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")),
        }, "couldn't open input.txt!")
    };
    ($s:expr) => {
        $crate::read_input!(impl $s, "couldn't open input file!")
    };
    (impl $s:expr, $m:literal) => {
        std::fs::read_to_string($s).expect($m)
    }
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
