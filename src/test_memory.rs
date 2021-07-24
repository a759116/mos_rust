#[cfg(test)]

mod test_memory {
    use crate::memory::{
        best_fit_allocate, first_fit_allocate, next_fit_allocate, release_memory,
        worst_fit_allocate, MemoryBlock,
    };

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
            && mb.process_id == process_id
        {
            return true;
        } else {
            return false;
        }
    }

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

    #[test]
    fn test_best_fit_allocate_free_block_same_size_as_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 10,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 49,
            segment_size: 10,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 50,
            end_address: 1023,
            segment_size: 974,
            process_id: 0,
        });

        let request_size = 20;
        let process_id = 40;

        let mb_allocated: MemoryBlock =
            best_fit_allocate(&mut memory_map, request_size, process_id);

        assert_eq!(memory_map.len(), 4);
        assert!(
            if_equal(&mb_allocated, 20, 39, 20, 40),
            "Result: {:?}",
            mb_allocated
        );
        assert!(if_equal(&memory_map[0], 0, 19, 20, 10));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 40));
        assert!(if_equal(&memory_map[2], 40, 49, 10, 20));
        assert!(if_equal(&memory_map[3], 50, 1023, 974, 0));
    }

    #[test]
    fn test_best_fit_allocate_free_block_smaller_than_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 10,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 49,
            segment_size: 10,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 50,
            end_address: 1023,
            segment_size: 974,
            process_id: 30,
        });

        let request_size = 30;
        let process_id = 40;

        let mb_allocated: MemoryBlock =
            best_fit_allocate(&mut memory_map, request_size, process_id);

        assert!(if_equal(&memory_map[0], 0, 19, 20, 10));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 0));
        assert!(if_equal(&memory_map[2], 40, 49, 10, 20));
        assert!(if_equal(&memory_map[3], 50, 1023, 974, 30));
        assert_eq!(memory_map.len(), 4);
        assert!(
            if_equal(&mb_allocated, 0, 0, 0, 0),
            "Result: {:?}",
            mb_allocated
        );
    }

    #[test]
    fn test_best_fit_allocate_free_block_larger_than_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 54,
            segment_size: 15,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 55,
            end_address: 1023,
            segment_size: 969,
            process_id: 30,
        });

        let request_size = 10;
        let process_id = 40;

        let mb_allocated: MemoryBlock =
            best_fit_allocate(&mut memory_map, request_size, process_id);

        assert!(if_equal(&memory_map[0], 0, 19, 20, 0));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 20));
        assert!(if_equal(&memory_map[2], 40, 49, 10, 40));
        assert!(if_equal(&memory_map[3], 50, 54, 5, 0));
        assert!(if_equal(&memory_map[4], 55, 1023, 969, 30));
        assert_eq!(memory_map.len(), 5);
        assert!(
            if_equal(&mb_allocated, 40, 49, 10, 40),
            "Result: {:?}",
            mb_allocated
        );
    }

    #[test]
    fn test_first_fit_allocate() {
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
            first_fit_allocate(&mut memory_map, request_size, process_id);

        assert!(if_equal(&memory_map[0], 0, 9, 10, 32));
        assert!(if_equal(&memory_map[1], 10, 1023, 1014, 0));
        assert_eq!(memory_map.len(), 2);
        assert!(if_equal(&mb_allocated, 0, 9, 10, 32));
    }

    #[test]
    fn test_test_first_fit_allocate_free_block_same_size_as_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 10,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 49,
            segment_size: 10,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 50,
            end_address: 1023,
            segment_size: 974,
            process_id: 0,
        });

        let request_size = 20;
        let process_id = 40;

        let mb_allocated: MemoryBlock =
            first_fit_allocate(&mut memory_map, request_size, process_id);

        assert!(if_equal(&memory_map[0], 0, 19, 20, 10));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 40));
        assert!(if_equal(&memory_map[2], 40, 49, 10, 20));
        assert!(if_equal(&memory_map[3], 50, 1023, 974, 0));
        assert_eq!(memory_map.len(), 4);
        assert!(
            if_equal(&mb_allocated, 20, 39, 20, 40),
            "Result: {:?}",
            mb_allocated
        );
    }

    #[test]
    fn test_first_fit_allocate_free_block_smaller_than_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 10,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 49,
            segment_size: 10,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 50,
            end_address: 1023,
            segment_size: 974,
            process_id: 30,
        });

        let request_size = 30;
        let process_id = 40;

        let mb_allocated: MemoryBlock =
            first_fit_allocate(&mut memory_map, request_size, process_id);

        assert!(if_equal(&memory_map[0], 0, 19, 20, 10));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 0));
        assert!(if_equal(&memory_map[2], 40, 49, 10, 20));
        assert!(if_equal(&memory_map[3], 50, 1023, 974, 30));
        assert_eq!(memory_map.len(), 4);
        assert!(
            if_equal(&mb_allocated, 0, 0, 0, 0),
            "Result: {:?}",
            mb_allocated
        );
    }

    #[test]
    fn test_first_fit_allocate_free_block_larger_than_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 54,
            segment_size: 15,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 55,
            end_address: 1023,
            segment_size: 969,
            process_id: 30,
        });

        let request_size = 15;
        let process_id = 40;

        let mb_allocated: MemoryBlock =
            first_fit_allocate(&mut memory_map, request_size, process_id);

        assert!(if_equal(&memory_map[0], 0, 14, 15, 40));
        assert!(if_equal(&memory_map[1], 15, 19, 5, 0));
        assert!(if_equal(&memory_map[2], 20, 39, 20, 20));
        assert!(if_equal(&memory_map[3], 40, 54, 15, 0));
        assert!(if_equal(&memory_map[4], 55, 1023, 969, 30));
        assert_eq!(memory_map.len(), 5);
        assert!(
            if_equal(&mb_allocated, 0, 14, 15, 40),
            "Result: {:?}",
            mb_allocated
        );
    }

    #[test]
    fn test_worst_fit_allocate() {
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
            worst_fit_allocate(&mut memory_map, request_size, process_id);

        assert!(if_equal(&memory_map[0], 0, 9, 10, 32));
        assert!(if_equal(&memory_map[1], 10, 1023, 1014, 0));
        assert_eq!(memory_map.len(), 2);
        assert!(if_equal(&mb_allocated, 0, 9, 10, 32));
    }

    #[test]
    fn test_worst_fit_allocate_allocate_free_block_same_size_as_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 10,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 49,
            segment_size: 10,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 50,
            end_address: 1023,
            segment_size: 974,
            process_id: 30,
        });

        let request_size = 20;
        let process_id = 40;

        let mb_allocated: MemoryBlock =
            worst_fit_allocate(&mut memory_map, request_size, process_id);

        assert!(if_equal(&memory_map[0], 0, 19, 20, 10));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 40));
        assert!(if_equal(&memory_map[2], 40, 49, 10, 20));
        assert!(if_equal(&memory_map[3], 50, 1023, 974, 30));
        assert_eq!(memory_map.len(), 4);
        assert!(
            if_equal(&mb_allocated, 20, 39, 20, 40),
            "Result: {:?}",
            mb_allocated
        );
    }

    #[test]
    fn test_worst_fit_allocate_free_block_smaller_than_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 10,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 49,
            segment_size: 10,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 50,
            end_address: 1023,
            segment_size: 974,
            process_id: 30,
        });

        let request_size = 30;
        let process_id = 40;

        let mb_allocated: MemoryBlock =
            worst_fit_allocate(&mut memory_map, request_size, process_id);

        assert!(if_equal(&memory_map[0], 0, 19, 20, 10));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 0));
        assert!(if_equal(&memory_map[2], 40, 49, 10, 20));
        assert!(if_equal(&memory_map[3], 50, 1023, 974, 30));
        assert_eq!(memory_map.len(), 4);
        assert!(
            if_equal(&mb_allocated, 0, 0, 0, 0),
            "Result: {:?}",
            mb_allocated
        );
    }

    #[test]
    fn test_worst_fit_allocate_free_block_larger_than_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 54,
            segment_size: 15,
            process_id: 30,
        });

        memory_map.push(MemoryBlock {
            start_address: 55,
            end_address: 1023,
            segment_size: 969,
            process_id: 0,
        });

        let request_size = 10;
        let process_id = 40;

        let mb_allocated: MemoryBlock =
            worst_fit_allocate(&mut memory_map, request_size, process_id);

        assert!(if_equal(&memory_map[0], 0, 19, 20, 0));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 20));
        assert!(if_equal(&memory_map[2], 40, 54, 15, 30));
        assert!(if_equal(&memory_map[3], 55, 64, 10, 40));
        assert!(if_equal(&memory_map[4], 65, 1023, 959, 0));
        assert_eq!(memory_map.len(), 5);
        assert!(
            if_equal(&mb_allocated, 55, 64, 10, 40),
            "Result: {:?}",
            mb_allocated
        );
    }

    #[test]
    fn test_next_fit_allocate() {
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
        let last_address = 0;

        let mb_allocated: MemoryBlock =
            next_fit_allocate(&mut memory_map, request_size, process_id, last_address);

        assert!(if_equal(&memory_map[0], 0, 9, 10, 32));
        assert!(if_equal(&memory_map[1], 10, 1023, 1014, 0));
        assert_eq!(memory_map.len(), 2);
        assert!(if_equal(&mb_allocated, 0, 9, 10, 32));
    }

    #[test]
    fn test_next_fit_allocate_free_block_same_size_as_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 10,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 49,
            segment_size: 10,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 50,
            end_address: 1023,
            segment_size: 974,
            process_id: 30,
        });

        let request_size = 20;
        let process_id = 40;
        let last_address = 0;

        let mb_allocated: MemoryBlock =
            next_fit_allocate(&mut memory_map, request_size, process_id, last_address);

        assert!(if_equal(&memory_map[0], 0, 19, 20, 10));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 40));
        assert!(if_equal(&memory_map[2], 40, 49, 10, 20));
        assert!(if_equal(&memory_map[3], 50, 1023, 974, 30));
        assert_eq!(memory_map.len(), 4);
        assert!(
            if_equal(&mb_allocated, 20, 39, 20, 40),
            "Result: {:?}",
            mb_allocated
        );
    }

    #[test]
    fn test_next_fit_allocate_free_block_smaller_than_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 10,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 49,
            segment_size: 10,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 50,
            end_address: 1023,
            segment_size: 974,
            process_id: 30,
        });

        let request_size = 30;
        let process_id = 40;
        let last_address = 0;

        let mb_allocated: MemoryBlock =
            next_fit_allocate(&mut memory_map, request_size, process_id, last_address);

        assert!(if_equal(&memory_map[0], 0, 19, 20, 10));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 0));
        assert!(if_equal(&memory_map[2], 40, 49, 10, 20));
        assert!(if_equal(&memory_map[3], 50, 1023, 974, 30));
        assert_eq!(memory_map.len(), 4);
        assert!(
            if_equal(&mb_allocated, 0, 0, 0, 0),
            "Result: {:?}",
            mb_allocated
        );
    }

    #[test]
    fn test_next_fit_allocate_free_block_larger_than_request() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 54,
            segment_size: 15,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 55,
            end_address: 74,
            segment_size: 20,
            process_id: 30,
        });

        memory_map.push(MemoryBlock {
            start_address: 75,
            end_address: 1023,
            segment_size: 949,
            process_id: 0,
        });

        let request_size = 30;
        let process_id = 40;
        let last_address = 2;

        let mb_allocated: MemoryBlock =
            next_fit_allocate(&mut memory_map, request_size, process_id, last_address);

        assert!(if_equal(&memory_map[0], 0, 19, 20, 0));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 20));
        assert!(if_equal(&memory_map[2], 40, 54, 15, 0));
        assert!(if_equal(&memory_map[3], 55, 74, 20, 30));
        assert!(if_equal(&memory_map[4], 75, 104, 30, 40));
        assert!(if_equal(&memory_map[5], 105, 1023, 919, 0));
        assert_eq!(memory_map.len(), 6);
        assert!(
            if_equal(&mb_allocated, 75, 104, 30, 40),
            "Result: {:?}",
            mb_allocated
        );
    }

    #[test]
    pub fn test_release_memory() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 7,
            segment_size: 8,
            process_id: 12,
        });

        memory_map.push(MemoryBlock {
            start_address: 8,
            end_address: 15,
            segment_size: 8,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 16,
            end_address: 23,
            segment_size: 8,
            process_id: 13,
        });

        memory_map.push(MemoryBlock {
            start_address: 24,
            end_address: 27,
            segment_size: 4,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 28,
            end_address: 29,
            segment_size: 2,
            process_id: 11,
        });
        let freed_block = MemoryBlock {
            start_address: 16,
            end_address: 23,
            segment_size: 8,
            process_id: 13,
        };

        release_memory(freed_block, &mut memory_map);

        assert!(if_equal(&memory_map[0], 0, 7, 8, 12));
        assert!(if_equal(&memory_map[1], 8, 27, 20, 0));
        assert!(if_equal(&memory_map[2], 28, 29, 2, 11));
        assert_eq!(memory_map.len(), 3);
    }

    #[test]
    pub fn test_release_memory_adjacent_block_before_freed_block_merged() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 54,
            segment_size: 15,
            process_id: 40,
        });

        memory_map.push(MemoryBlock {
            start_address: 55,
            end_address: 74,
            segment_size: 20,
            process_id: 30,
        });

        memory_map.push(MemoryBlock {
            start_address: 75,
            end_address: 1023,
            segment_size: 949,
            process_id: 0,
        });
        let freed_block = MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 20,
        };

        release_memory(freed_block, &mut memory_map);

        assert!(if_equal(&memory_map[0], 0, 39, 40, 0));
        assert!(if_equal(&memory_map[1], 40, 54, 15, 40));
        assert!(if_equal(&memory_map[2], 55, 74, 20, 30));
        assert!(if_equal(&memory_map[3], 75, 1023, 949, 0));
        assert_eq!(memory_map.len(), 4);
    }

    #[test]
    pub fn test_release_memory_adjacent_block_after_freed_block_merged() {
        let mut memory_map: Vec<MemoryBlock> = Vec::new();

        memory_map.push(MemoryBlock {
            start_address: 0,
            end_address: 19,
            segment_size: 20,
            process_id: 0,
        });

        memory_map.push(MemoryBlock {
            start_address: 20,
            end_address: 39,
            segment_size: 20,
            process_id: 20,
        });

        memory_map.push(MemoryBlock {
            start_address: 40,
            end_address: 54,
            segment_size: 15,
            process_id: 40,
        });

        memory_map.push(MemoryBlock {
            start_address: 55,
            end_address: 74,
            segment_size: 20,
            process_id: 30,
        });

        memory_map.push(MemoryBlock {
            start_address: 75,
            end_address: 1023,
            segment_size: 949,
            process_id: 0,
        });
        let freed_block = MemoryBlock {
            start_address: 55,
            end_address: 74,
            segment_size: 20,
            process_id: 30,
        };

        release_memory(freed_block, &mut memory_map);

        assert!(if_equal(&memory_map[0], 0, 19, 20, 0));
        assert!(if_equal(&memory_map[1], 20, 39, 20, 20));
        assert!(if_equal(&memory_map[2], 40, 54, 15, 40));
        assert!(if_equal(&memory_map[3], 55, 1023, 969, 0));
        assert_eq!(memory_map.len(), 4);
    }
}
