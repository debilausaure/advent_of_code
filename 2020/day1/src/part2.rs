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
        let mut double_ended_iter = values.iter();

        while let Some(smallest) = double_ended_iter.next() {
            // clone the iterator
            let mut inner_double_ended_iter = double_ended_iter.clone();
            let mut intermediate = inner_double_ended_iter.next().unwrap();
            let mut biggest = inner_double_ended_iter.next_back().unwrap();

            while intermediate <= biggest {
                let sum = smallest + intermediate + biggest;
                if sum <= 2020 {
                    if sum == 2020 {
                        println!("{}", smallest * intermediate * biggest);
                        return;
                    } else {
                        intermediate =  match inner_double_ended_iter.next() {
                                            None => break,
                                            Some(value) => value,
                                        };
                    }
                } else {
                    biggest = match inner_double_ended_iter.next_back() {
                                  None => break,
                                  Some(value) => value,
                              };
                }
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
