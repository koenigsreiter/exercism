pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = factors
        .iter()
        .filter(|factor| u32::clone(factor) != 0)
        .map(|factor| (*factor..limit).step_by(*factor as usize))
        .flatten()
        .collect::<Vec<u32>>();
    multiples.sort_unstable();
    multiples.dedup();
    multiples.iter().sum::<u32>()
}
