#[cfg(test)]

mod test_memory {
    use crate::memory::{best_fit_allocate, MemoryBlock};

    #[test]
    fn test_best_fit_allocate() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();
        let mb = MemoryBlock {
            start_address: 0,
            end_address: 1023,
            segment_size: 1024,
            process_id: 0,
        };

        memory_map.push(mb);
        let request_size = 10;
        let process_id = 32;

        let mb_allocated: MemoryBlock =
            best_fit_allocate(&mut memory_map, request_size, process_id);

        assert_eq!(memory_map.len(), 2);
        assert!(if_equal(&mb_allocated, 0, 9, 10, 32));
        assert!(if_equal(&memory_map[0], 0, 9, 10, 32));
        assert!(if_equal(&memory_map[1], 10, 1023, 1014, 0));
    }

    fn if_equal(
        mb: &MemoryBlock,
        start_address: i32,
        end_address: i32,
        segment_size: i32,
        process_id: i32,
    ) -> bool {
        if mb.start_address == start_address 
            && mb.end_address == end_address
            && mb.segment_size == segment_size
            && mb.process_id == process_id {
            return true;
        }
        else {
            return false;
        }
    }
}
