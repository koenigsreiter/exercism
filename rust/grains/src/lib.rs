pub fn square(s: u32) -> u64 {
    if s > 64 || s < 1 {
        panic!("Square must be between 1 and 64");
    }
    2_u64.pow(s - 1)
    // unimplemented!("grains of rice on square {}", s);
}

pub fn total() -> u64 {
    (1..65).map(square).sum::<u64>()
}
