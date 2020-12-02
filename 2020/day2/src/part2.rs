use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub struct Policy{
    positions : [usize ; 2],
    character : char,
}

impl FromStr for Policy {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.split(' ').map(|s| s.trim());

        let positions_str = s_iter.next().unwrap();
        let positions_vec = positions_str.split('-')
                                         .filter_map(|pos_str| pos_str.parse::<usize>().ok())
                                         .map(|pos| pos - 1)
                                         .collect::<Vec<usize>>();

        let character_str = s_iter.next().unwrap();
        let character = character_str.chars().next().unwrap();

        Ok(Self{positions : [positions_vec[0], positions_vec[1]], character})
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
                                  let positions = &password_s.policy.positions;
                                  let password = &password_s.password;
                                  let character = password_s.policy.character;
                                  let first_position_matches = password.chars().nth(positions[0]).unwrap() == character;
                                  let second_position_matches = password.chars().nth(positions[1]).unwrap() == character;
                                  first_position_matches ^ second_position_matches
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
