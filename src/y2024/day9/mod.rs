use crate::parse_input;

mod disk;
use disk::*;

pub fn task() {
    let lines = parse_input!();
    let first_line = lines.first().unwrap();

    let disk_map = first_line
        .chars()
        .map(|ch| ch.to_string().parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    
    // part1(&disk_map);
    part2(&disk_map);
    
}

fn part1(disk_map: &Vec<u8>) {
    let mut disk = restore_disk_blocks(disk_map);

    fragment_disk_blocks(&mut disk);

    let checksum = calculate_checksum(&disk);

    println!("Checksum: {}", checksum);
}

fn part2(disk_map: &Vec<u8>) {
    let mut disk = restore_disk_blocks(disk_map);

    fragment_disk_files(&mut disk);

    let checksum = calculate_checksum(&disk);

    println!("Checksum: {}", checksum);
}