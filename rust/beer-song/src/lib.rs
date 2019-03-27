pub fn verse(n: i32) -> String {
    match n {
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        n => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1)
    }
    //unimplemented!("emit verse {}", n)
}

pub fn sing(start: i32, end: i32) -> String {
    let mut output = verse(start);
    for line in (end..start).rev() {
        output = format!("{}\n{}", output, verse(line));
    }
    output
    //unimplemented!("sing verses {} to {}, inclusive", start, end)
}
