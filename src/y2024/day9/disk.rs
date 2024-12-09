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
    let mut right = disk.len() - 1;

    'fragmentation: loop {
        // moving right cursor to the first taken block
        while disk[right].is_none() {
            right -= 1;
        }

        let file_start_index = right;
        let file_id = disk[right].unwrap();
        let mut file_size = 1usize;

        while disk[right - 1] == disk[right] {
            file_size += 1;
            right -= 1;
        }
        right -= 1;

        let mut left = 0usize;
        let mut is_space_left = false;
        'find_space_for_current_file: while left < right {
            // moving left cursor to the first empty block
            while disk[left].is_some() {
                left += 1;

                // left cursor met with the right one
                if left >= right {
                    if is_space_left {
                        // if there is still some space, we just skip this file, maybe some another file will fit
                        break 'find_space_for_current_file;
                    } else {
                        // if there is no space left - time to terminate fragmentation
                        break 'fragmentation;
                    }
                }
            }

            // we successfully moved to an empty block without hitting right cursor - there is still some space
            is_space_left = true;

            let space_start_index = left;
            let mut space_size = 1usize;
            while disk[left + 1].is_none() {
                space_size += 1;
                left += 1;
            }

            if space_size >= file_size {
                for offset in 0..file_size {
                    disk.swap(space_start_index + offset, file_start_index - offset);
                }

                // we've found space for this file - we can move on
                break 'find_space_for_current_file;
            } else {
                // move to the next file segment
                while disk[left].is_none() {
                    left += 1;
                }
            }
        }
    }
}
