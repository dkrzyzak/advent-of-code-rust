type Disk = Vec<Option<usize>>;

pub fn restore_disk_blocks(disk_map: &Vec<u8>) -> Disk {
    let mut current_id = 0usize;
    let mut disk: Disk = Vec::new();
    let mut file = true;

    for item in disk_map.iter() {
        if file {
            disk.extend(std::iter::repeat(Some(current_id)).take(*item as usize));
            current_id += 1;
        } else {
            disk.extend(std::iter::repeat(None).take(*item as usize));
        }

        file = !file;
    }

    disk
}

pub fn calculate_checksum(disk: &Disk) -> usize {
    let mut checksum = 0;
    let mut position = 0;

    for item in disk.iter() {
        if item.is_some() {
            checksum += position * disk[position].unwrap();
        }

        position += 1;
    }

    checksum
}

// PART 1
pub fn fragment_disk_blocks(disk: &mut Disk) {
    let mut left = 0usize;
    let mut right = disk.len() - 1;

    loop {
        // moving left cursor to the first empty block
        while disk[left].is_some() {
            left += 1;
        }

        // moving right cursor to the first taken block
        while disk[right].is_none() {
            right -= 1;
        }

        if left >= right {
            break;
        }

        disk.swap(left, right);
    }
}

// PART 2
pub fn fragment_disk_files(disk: &mut Disk) {
    let mut left = 0usize;
    let mut right = disk.len() - 1;

    loop {
        // moving left cursor to the first empty block
        while disk[left].is_some() {
            left += 1;
        }

        // moving right cursor to the first taken block
        while disk[right].is_none() {
            right -= 1;
        }

        if left >= right {
            break;
        }

        disk.swap(left, right);
    }
}
