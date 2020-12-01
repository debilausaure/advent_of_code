fn main() {
    let input = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();
    
    let mut counter : i64 = 0;
    for (i, c) in input.into_iter().enumerate() {
        if (c == b'(') {
            counter += 1;
        } else if (c == b')') {
            counter -= 1;
            if counter < 0 {
                println!("{}", i+1);
                break;
            }
        }
    }
}
