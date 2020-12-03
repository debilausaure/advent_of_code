use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(res_lines) = read_lines("./input") {
        let topologies = res_lines
            .filter_map(|res_line| res_line.ok())
            .map(|topology| {
                topology.chars()
                .map(|topology_char| topology_char == '#')
                .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();

        let mut width_index = 0;
        let mut trees_hit = 0;
        let topology_width = topologies[0].len();
        for topology in topologies {
            if topology[width_index] {
                trees_hit += 1;
            }
            width_index = (width_index + 3) % topology_width;
        }
        println!("{}", trees_hit);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
