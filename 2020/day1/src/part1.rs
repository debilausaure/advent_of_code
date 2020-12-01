use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(res_option_lines) = read_lines("./input") {
        let mut values = res_option_lines
                              .filter_map(|res_option_string| res_option_string.ok())
                              .filter_map(|option_string| option_string.parse::<usize>().ok())
                              .collect::<Vec<usize>>();
        values.sort_unstable();

        //create an iterator that can iterate over the beggining and the end of the vector
        let mut double_ended_iter = values.into_iter();
        let mut smallest = double_ended_iter.next().unwrap();
        let mut biggest = double_ended_iter.next_back().unwrap();

        while smallest <= biggest {
            if smallest + biggest <= 2020 {
                if smallest + biggest == 2020 {
                    println!("{}", smallest * biggest);
                    break;
                } else {
                    smallest = double_ended_iter.next().unwrap();
                }
            } else {
                biggest = double_ended_iter.next_back().unwrap();
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
