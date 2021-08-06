
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy, PartialEq, Eq, )]
pub struct MemoryBlock {
    pub start_address: i32,
    pub end_address: i32,
    pub segment_size: i32,
    pub process_id: i32, //0 indicates a free block
}

#[allow(dead_code)]
impl MemoryBlock {
    fn get_start_address(&self) -> &i32 {
        &self.start_address
    }
    fn set_start_address(&mut self) -> &mut i32 {
        &mut self.start_address
    }
    fn get_end_address(&self) -> &i32 {
        &self.end_address
    }
    fn set_end_address(&mut self) -> &mut i32 {
        &mut self.end_address
    }
    fn get_segment_size(&self) -> &i32 {
        &self.segment_size
    }
    fn set_segment_size(&mut self) -> &mut i32 {
        &mut self.segment_size
    }
    fn get_process_id(&self) -> &i32 {
        &self.process_id
    }
    fn set_process_id(&mut self) -> &mut i32 {
        &mut self.process_id
    }
}

pub fn best_fit_allocate(
    memory_map: &mut Vec<MemoryBlock>,
    request_size: i32,
    process_id: i32,
) -> MemoryBlock {
    let mut memory_block: MemoryBlock = Default::default();
    let mut closest_mb_segment_size: i32 = i32::MAX;
    let mut position: i8 = -1;
    for (i, mb) in memory_map.iter().enumerate() {
        if mb.process_id == 0 {
            if request_size == mb.segment_size {
                memory_block = *mb;
                memory_block.process_id = process_id;
                memory_map[i].process_id = process_id;
                return memory_block;
            } else if request_size < mb.segment_size {
                if mb.segment_size < closest_mb_segment_size {
                    closest_mb_segment_size = mb.segment_size;
                    position = i as i8;
                }
            }
        }
    }

    if position > -1 {
        let p: usize = position as usize;
        memory_map[p].segment_size = request_size;
        memory_map[p].process_id = process_id;
        memory_map[p].end_address = memory_map[p].start_address + memory_map[p].segment_size - 1;
        let remaining_mb = MemoryBlock {
            start_address: memory_map[p].end_address + 1,
            segment_size: closest_mb_segment_size - request_size,
            end_address: memory_map[p].end_address + 1 + closest_mb_segment_size - request_size - 1,
            process_id: 0,
        };
        memory_map.insert(p + 1, remaining_mb);
        memory_block = memory_map[p];
    }

    return memory_block;
}

pub fn first_fit_allocate(
    memory_map: &mut Vec<MemoryBlock>,
    request_size: i32,
    process_id: i32,
) -> MemoryBlock {
    let mut memory_block: MemoryBlock = Default::default();
    let mut closest_mb_segment_size: i32 = i32::MAX;
    let mut position: i8 = -1;
    for (i, mb) in memory_map.iter().enumerate() {
        if mb.process_id == 0 {
            if request_size <= mb.segment_size {
                closest_mb_segment_size = mb.segment_size;
                position = i as i8;
                break;
            }
        }
    }

    if position > -1 {
        let p: usize = position as usize;
        if request_size == memory_map[p].segment_size {
            memory_block = memory_map[p];
            memory_block.process_id = process_id;
            memory_map[p].process_id = process_id;
        } else {
            memory_map[p].segment_size = request_size;
            memory_map[p].process_id = process_id;
            memory_map[p].end_address =
                memory_map[p].start_address + memory_map[p].segment_size - 1;
            let remaining_mb = MemoryBlock {
                start_address: memory_map[p].end_address + 1,
                segment_size: closest_mb_segment_size - request_size,
                end_address: memory_map[p].end_address + 1 + closest_mb_segment_size
                    - request_size
                    - 1,
                process_id: 0,
            };
            memory_map.insert(p + 1, remaining_mb);
            memory_block = memory_map[p];
        }
    }

    return memory_block;
}

pub fn worst_fit_allocate(
    memory_map: &mut Vec<MemoryBlock>,
    request_size: i32,
    process_id: i32,
) -> MemoryBlock {
    let mut memory_block: MemoryBlock = Default::default();
    let mut closest_mb_segment_size: i32 = 0;
    let mut position: i8 = -1;
    for (i, mb) in memory_map.iter().enumerate() {
        if mb.process_id == 0 {
            if request_size <= mb.segment_size {
                if mb.segment_size >= closest_mb_segment_size {
                    closest_mb_segment_size = mb.segment_size;
                    position = i as i8;
                }
            }
        }
    }

    if position > -1 {
        let p: usize = position as usize;
        if request_size == memory_map[p].segment_size {
            memory_block = memory_map[p];
            memory_block.process_id = process_id;
            memory_map[p].process_id = process_id;
        } else {
            memory_map[p].segment_size = request_size;
            memory_map[p].process_id = process_id;
            memory_map[p].end_address =
                memory_map[p].start_address + memory_map[p].segment_size - 1;
            let remaining_mb = MemoryBlock {
                start_address: memory_map[p].end_address + 1,
                segment_size: closest_mb_segment_size - request_size,
                end_address: memory_map[p].end_address + 1 + closest_mb_segment_size
                    - request_size
                    - 1,
                process_id: 0,
            };
            memory_map.insert(p + 1, remaining_mb);
            memory_block = memory_map[p];
        }
    }

    return memory_block;
}

pub fn next_fit_allocate(
    memory_map: &mut Vec<MemoryBlock>,
    request_size: i32,
    process_id: i32,
    last_address: i32,
) -> MemoryBlock {
    let mut memory_block: MemoryBlock = Default::default();
    let mut closest_mb_segment_size: i32 = 0;
    let mut position: i8 = -1;
    for (i, mb) in memory_map.iter().enumerate() {
        if memory_map[i].start_address < last_address {
            continue;
        } else if mb.process_id == 0 {
            if request_size <= mb.segment_size {
                if mb.segment_size >= closest_mb_segment_size {
                    closest_mb_segment_size = mb.segment_size;
                    position = i as i8;
                    break;
                }
            }
        }
    }

    if position > -1 {
        let p: usize = position as usize;
        if request_size == memory_map[p].segment_size {
            memory_block = memory_map[p];
            memory_block.process_id = process_id;
            memory_map[p].process_id = process_id;
        } else {
            memory_map[p].segment_size = request_size;
            memory_map[p].process_id = process_id;
            memory_map[p].end_address =
                memory_map[p].start_address + memory_map[p].segment_size - 1;
            let remaining_mb = MemoryBlock {
                start_address: memory_map[p].end_address + 1,
                segment_size: closest_mb_segment_size - request_size,
                end_address: memory_map[p].end_address + 1 + closest_mb_segment_size
                    - request_size
                    - 1,
                process_id: 0,
            };
            memory_map.insert(p + 1, remaining_mb);
            memory_block = memory_map[p];
        }
    }

    return memory_block;
}

pub fn release_memory(freed_block: MemoryBlock, memory_map: &mut Vec<MemoryBlock>) -> () {
    let mut position: i8 = -1;
    for (i, mb) in memory_map.iter().enumerate() {
        if (*mb) == freed_block {
            position = i as i8;
            break;
        }
    }

    if position > -1 {
        let p = position as usize;
        let mut merge_left_index = p;
        let mut merge_right_index = p;
        if memory_map[p - 1].process_id == 0 {
            merge_left_index = p - 1;
        }
        if p + 1 < memory_map.len() && memory_map[p + 1].process_id == 0 {
            merge_right_index = p + 1;
        }

        if merge_left_index == merge_right_index {
            memory_map[p].process_id = 0;
        } else {
            let mut merged_mb = MemoryBlock {
                start_address: memory_map[merge_left_index].start_address,
                end_address: memory_map[merge_right_index].end_address,
                segment_size: 0,
                process_id: 0,
            };
            let mut segment_sum = 0;
            for i in merge_left_index..merge_right_index + 1 {
                segment_sum += memory_map[i].segment_size;
            }
            merged_mb.segment_size = segment_sum;
            memory_map[merge_left_index] = merged_mb;
            for i in merge_left_index + 1..memory_map.len() - (merge_right_index - merge_left_index)
            {
                memory_map[i] = memory_map[i+merge_right_index-merge_left_index];
            }

            let length_after_merge = memory_map.len() - (merge_right_index - merge_left_index);
            
            while memory_map.len() > length_after_merge{
                memory_map.remove(memory_map.len()-1);
                
            }
        }
    }
}