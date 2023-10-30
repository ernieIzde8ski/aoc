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

#[macro_export]
macro_rules! benchmark {
    ($code:expr) => {
        $crate::benchmark!(1000, $code);
    };

    ($loops:literal, $code:expr ) => {
        let before = std::time::Instant::now();
        for _ in 0..($loops) {
            $code;
        }
        let elapsed = before.elapsed();
        println!("Iterations:   {}", $loops);
        println!("Time elapsed: {:.3?}", elapsed);
        println!("Average time: {:.3?}", elapsed / $loops);
    };
}
