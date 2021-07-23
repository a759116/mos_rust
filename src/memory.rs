#[derive(Debug, Default, Clone, Copy)]
pub struct MemoryBlock {
    pub(crate) start_address: i32,
    pub(crate) end_address: i32,
    pub(crate) segment_size: i32,
    pub(crate) process_id: i32, //0 indicates a free block
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
        memory_map.insert(p+1, remaining_mb);
        memory_block = memory_map[p];
    }

    return memory_block;
}
