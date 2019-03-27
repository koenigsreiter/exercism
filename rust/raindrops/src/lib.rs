pub fn raindrops(n: u32) -> String {
    match (n, n % 3, n % 5, n % 7) {
        (_, 0, 0, 0) => String::from("PlingPlangPlong"),
        (_, 0, _, 0) => String::from("PlingPlong"),
        (_, _, 0, 0) => String::from("PlangPlong"),
        (_, 0, 0, _) => String::from("PlingPlang"),
        (_, 0, _, _) => String::from("Pling"),
        (_, _, 0, _) => String::from("Plang"),
        (_, _, _, 0) => String::from("Plong"),
        (n, _, _, _) => n.to_string()
    }
    //unimplemented!("what sound does Raindrop #{} make?", n)
}
