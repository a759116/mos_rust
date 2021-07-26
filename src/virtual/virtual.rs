use std::usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PTE {
    pub is_valid: bool,
    pub frame_number: i32,
    pub arrival_timestamp: i32,
    pub last_access_timestamp: i32,
    pub reference_count: i32,
}

impl Default for PTE {
    fn default() -> Self {
        PTE {
            is_valid: false,
            frame_number: -1,
            arrival_timestamp: -1,
            last_access_timestamp: -1,
            reference_count: -1,
        }
    }
}

fn allocate_memory(
    page_table: &mut Vec<PTE>,
    page_number: usize,
    frame: i32,
    current_timestamp: i32,
) {
    page_table[page_number].arrival_timestamp = current_timestamp;
    page_table[page_number].frame_number = frame;
    page_table[page_number].is_valid = true;
    page_table[page_number].last_access_timestamp = current_timestamp;
    page_table[page_number].reference_count = 1;
}

fn replace_memory(
    position: i32,
    page_table: &mut Vec<PTE>,
    page_number: usize,
    frame: i32,
    current_timestamp: i32,
) {
    let p = position as usize;
    page_table[p] = PTE::default();
    allocate_memory(page_table, page_number, frame, current_timestamp);
}

pub fn process_page_access_fifo(
    page_table: &mut Vec<PTE>,
    page_number: usize,
    frame_pool: &mut Vec<i32>,
    current_timestamp: i32,
) -> i32 {
    let mut frame: i32 = -1;

    let table_cnt = page_table.len();

    if page_table[page_number].is_valid == true {
        page_table[page_number].last_access_timestamp = current_timestamp;
        page_table[page_number].reference_count = page_table[page_number].reference_count + 1;
        frame = page_table[page_number].frame_number;
    } else if frame_pool.len() > 0 {
        frame = match frame_pool.pop() {
            Some(t) => t,
            None => -1,
        };
        allocate_memory(page_table, page_number, frame, current_timestamp);
    } else {
        let mut smallest_arrival_timestamp = i32::MAX;
        let mut position: i32 = -1;
        for i in 0..table_cnt {
            if page_table[i].is_valid == true
                && page_table[i].arrival_timestamp < smallest_arrival_timestamp
            {
                smallest_arrival_timestamp = page_table[i].arrival_timestamp;
                position = i as i32;
            }
        }

        if position > -1 {
            frame = page_table[position as usize].frame_number;
            replace_memory(position, page_table, page_number, frame, current_timestamp);
        }
    }

    frame
}
