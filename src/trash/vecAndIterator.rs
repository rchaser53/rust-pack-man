fn main() {
    let hoge_str = "ab1de1gh1jk";
    let mut result: Vec<char> = Vec::new();
    let hoge_chars: Vec<char> = hoge_str.chars().collect();
    let nyan = hoge_chars.iter().enumerate();

    let aaa = nyan.fold(result, |mut stack, (cell_index, map_char)| {
        if *map_char == '1' {
            stack.push(*map_char);
        }
        return stack;
    });

    println!("{:?}", aaa);
}