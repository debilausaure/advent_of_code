use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
pub struct Policy{
    range : Range<usize>,
    character : char,
}

impl FromStr for Policy {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.split(' ').map(|s| s.trim());

        let range_str = s_iter.next().unwrap();
        let range_bounds = range_str.split('-')
                                    .map(|usize_str| usize_str.parse::<usize>())
                                    .collect::<Result<Vec<usize>, _>>()?;
        let range : Range<usize> = Range {
            start : range_bounds[0],
            end : range_bounds[1] + 1,
        };

        let character_str =  s_iter.next().unwrap();
        let character = character_str.chars().next().unwrap();

        Ok(Self{range, character})
    }
}

#[derive(Debug)]
pub struct Password{
    policy : Policy,
    password : String,
}

impl FromStr for Password{
    type Err = Box<dyn std::error::Error>;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.split(':').map(|s| s.trim());
        let policy = s_iter.next().unwrap().parse()?;
        let password = s_iter.next().unwrap().to_string();

        Ok(Self {
            policy,
            password,
        })
    }
}

fn main() {
    if let Ok(res_lines) = read_lines("./input") {
        let passwords = res_lines
                              .filter_map(|res_line| res_line.ok())
                              .filter_map(move |line| line.parse::<Password>().ok())
                              .collect::<Vec<Password>>();
        let policy_matches = passwords.iter()
                             .map(|password_s| { 
                                  let range = &password_s.policy.range;
                                  let password = &password_s.password;
                                  let character = password_s.policy.character;
                                  range.contains(
                                      &password.match_indices(character)
                                               .count()
                                  )
                             })
                             .filter(|predicate| *predicate)
                             .count();
        println!("{}", policy_matches);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
