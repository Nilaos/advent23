use aoc_driver::*;

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(session.as_str(), 2023:2:2, part_2).unwrap();
    println!("Well done! 🦀🦀🦀")
}
