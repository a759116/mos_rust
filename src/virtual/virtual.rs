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

pub fn count_page_faults_fifo(
    page_table: &mut Vec<PTE>,
    page_references: Vec<i32>,
    frame_pool: &mut Vec<i32>,
) -> i32 {
    let mut faults: i32 = 0;
    let table_cnt = page_table.len();
    let reference_count = page_references.len();
    let mut current_timestamp = 0;

    for i in 0..table_cnt {
        if page_table[i].arrival_timestamp > current_timestamp {
            current_timestamp = page_table[i].arrival_timestamp;
        }
    }
    current_timestamp = current_timestamp + 1;

    for r in 0..reference_count {
        let page_number = page_references[r] as usize;

        if page_table[page_number].is_valid == true {
            page_table[page_number].last_access_timestamp = current_timestamp;
            page_table[page_number].reference_count = page_table[page_number].reference_count + 1;
            current_timestamp = current_timestamp + 1;
        } else if frame_pool.len() > 0 {
            let frame = match frame_pool.pop() {
                Some(t) => t,
                None => -1,
            };
            allocate_memory(page_table, page_number, frame, current_timestamp);
            faults = faults + 1;
            current_timestamp = current_timestamp + 1;
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
                let frame = page_table[position as usize].frame_number;
                replace_memory(position, page_table, page_number, frame, current_timestamp);
            }
            faults = faults + 1;
            current_timestamp = current_timestamp + 1;
        }
    }

    return faults;
}

pub fn process_page_access_lru(
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
        let mut smallest_last_access_timestamp = i32::MAX;
        let mut position: i32 = -1;
        for i in 0..table_cnt {
            if page_table[i].is_valid == true
                && page_table[i].last_access_timestamp < smallest_last_access_timestamp
            {
                smallest_last_access_timestamp = page_table[i].last_access_timestamp;
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

pub fn count_page_faults_lru(
    page_table: &mut Vec<PTE>,
    page_references: Vec<i32>,
    frame_pool: &mut Vec<i32>,
) -> i32 {
    let mut faults: i32 = 0;
    let table_cnt = page_table.len();
    let reference_count = page_references.len();
    let mut current_timestamp = 0;

    for i in 0..table_cnt {
        if page_table[i].arrival_timestamp > current_timestamp {
            current_timestamp = page_table[i].arrival_timestamp;
        }
    }
    current_timestamp = current_timestamp + 1;

    for r in 0..reference_count {
        let page_number = page_references[r] as usize;

        if page_table[page_number].is_valid == true {
            page_table[page_number].last_access_timestamp = current_timestamp;
            page_table[page_number].reference_count = page_table[page_number].reference_count + 1;
            current_timestamp = current_timestamp + 1;
        } else if frame_pool.len() > 0 {
            let frame = match frame_pool.pop() {
                Some(t) => t,
                None => -1,
            };
            allocate_memory(page_table, page_number, frame, current_timestamp);
            faults = faults + 1;
            current_timestamp = current_timestamp + 1;
        } else {
            let mut smallest_last_access_timestamp = i32::MAX;
            let mut position: i32 = -1;
            for i in 0..table_cnt {
                if page_table[i].is_valid == true
                    && page_table[i].last_access_timestamp < smallest_last_access_timestamp
                {
                    smallest_last_access_timestamp = page_table[i].last_access_timestamp;
                    position = i as i32;
                }
            }

            if position > -1 {
                let frame = page_table[position as usize].frame_number;
                replace_memory(position, page_table, page_number, frame, current_timestamp);
            }
            faults = faults + 1;
            current_timestamp = current_timestamp + 1;
        }
    }

    return faults;
}

pub fn process_page_access_lfu(
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
        let mut smallest_reference_count = i32::MAX;

        let mut positions: Vec<usize> = Vec::new();
        for i in 0..table_cnt {
            if page_table[i].is_valid == true {
                if page_table[i].reference_count < smallest_reference_count {
                    positions.clear();
                    positions.push(i);
                    smallest_reference_count = page_table[i].reference_count;
                } else if page_table[i].reference_count == smallest_reference_count {
                    positions.push(i);
                }
            }
        }

        if positions.len() > 0 {
            let mut smallest_arrival_timestamp = i32::MAX;
            let mut position: i32 = -1;
            for idx in positions {
                if page_table[idx].arrival_timestamp < smallest_arrival_timestamp {
                    smallest_arrival_timestamp = page_table[idx].arrival_timestamp;
                    position = idx as i32;
                }
            }
            frame = page_table[position as usize].frame_number;
            replace_memory(position, page_table, page_number, frame, current_timestamp);
        }
    }

    frame
}

pub fn count_page_faults_lfu(
    page_table: &mut Vec<PTE>,
    page_references: Vec<i32>,
    frame_pool: &mut Vec<i32>,
) -> i32 {
    let mut faults: i32 = 0;
    let table_cnt = page_table.len();
    let reference_count = page_references.len();
    let mut current_timestamp = 0;

    for i in 0..table_cnt {
        if page_table[i].arrival_timestamp > current_timestamp {
            current_timestamp = page_table[i].arrival_timestamp;
        }
    }
    current_timestamp = current_timestamp + 1;

    for r in 0..reference_count {
        let page_number = page_references[r] as usize;

        if page_table[page_number].is_valid == true {
            page_table[page_number].last_access_timestamp = current_timestamp;
            page_table[page_number].reference_count = page_table[page_number].reference_count + 1;
            current_timestamp = current_timestamp + 1;
        } else if frame_pool.len() > 0 {
            let frame = match frame_pool.pop() {
                Some(t) => t,
                None => -1,
            };
            allocate_memory(page_table, page_number, frame, current_timestamp);
            faults = faults + 1;
            current_timestamp = current_timestamp + 1;
        } else {
            let mut smallest_reference_count = i32::MAX;

            let mut positions: Vec<usize> = Vec::new();
            for i in 0..table_cnt {
                if page_table[i].is_valid == true {
                    if page_table[i].reference_count < smallest_reference_count {
                        positions.clear();
                        positions.push(i);
                        smallest_reference_count = page_table[i].reference_count;
                    } else if page_table[i].reference_count == smallest_reference_count {
                        positions.push(i);
                    }
                }
            }

            if positions.len() > 0 {
                let mut smallest_arrival_timestamp = i32::MAX;
                let mut position: i32 = -1;
                for idx in positions {
                    if page_table[idx].arrival_timestamp < smallest_arrival_timestamp {
                        smallest_arrival_timestamp = page_table[idx].arrival_timestamp;
                        position = idx as i32;
                    }
                }
                let frame = page_table[position as usize].frame_number;
                replace_memory(position, page_table, page_number, frame, current_timestamp);
            }
            faults = faults + 1;
            current_timestamp = current_timestamp + 1;
        }
    }

    return faults;
}
