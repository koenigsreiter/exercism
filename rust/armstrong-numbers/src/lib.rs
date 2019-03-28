pub fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len();
    num.to_string()
        .chars()
        .flat_map(|x| x.to_digit(10))
        .map(|x| x.pow(len as u32))
        .sum::<u32>()
        == num
    //unimplemented!("true if {} is an armstrong number", num)
}
