#[cfg(test)]

mod test_virtual {
    use crate::r#virtual::r#virtual::{
        count_page_faults_fifo, count_page_faults_lru, process_page_access_fifo,
        process_page_access_lru, PTE, process_page_access_lfu, count_page_faults_lfu,
    };

    fn set_pte(
        is_valid: bool,
        frame_number: i32,
        arrival_timestamp: i32,
        last_access_timestamp: i32,
        reference_count: i32,
    ) -> PTE {
        let pte = PTE {
            is_valid,
            frame_number,
            arrival_timestamp,
            last_access_timestamp,
            reference_count,
        };
        pte
    }

    #[test]
    pub fn test_process_page_access_fifo() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[2] = set_pte(true, 10, 3, 3, 1);
        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);
        let page_number: usize = 0;
        let current_timestamp: i32 = 12;

        let frame = process_page_access_fifo(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }
        expected_page_table[0] = set_pte(true, 30, 12, 12, 1);
        expected_page_table[2] = set_pte(true, 10, 3, 3, 1);
        expected_page_table[5] = set_pte(true, 20, 2, 4, 2);
        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }
        assert_eq!(page_table.len(), 8);
        assert_eq!(frame_pool.len(), 0);
        assert_eq!(frame, 30);
    }

    #[test]
    pub fn test_process_page_access_fifo_page_in_memory() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[2] = set_pte(true, 10, 3, 3, 1);
        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);
        let page_number: usize = 2;
        let current_timestamp: i32 = 14;

        let frame = process_page_access_fifo(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }

        expected_page_table[2] = set_pte(true, 10, 3, 14, 2);
        expected_page_table[5] = set_pte(true, 20, 2, 4, 2);
        expected_page_table[7] = set_pte(true, 30, 1, 1, 1);

        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }

        assert_eq!(page_table.len(), 8);
        assert_eq!(frame_pool.len(), 0);
        assert_eq!(frame, 10);
    }

    #[test]
    pub fn test_process_page_access_fifo_page_not_in_memory_frame_available() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);

        let page_number: usize = 3;
        frame_pool.push(10);
        let current_timestamp: i32 = 15;

        let frame = process_page_access_fifo(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }

        expected_page_table[3] = set_pte(true, 10, 15, 15, 1);
        expected_page_table[5] = set_pte(true, 20, 2, 4, 2);
        expected_page_table[7] = set_pte(true, 30, 1, 1, 1);

        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }

        assert_eq!(page_table.len(), 8);
        assert_eq!(frame_pool.len(), 0);
        assert_eq!(frame, 10);
    }

    #[test]
    pub fn test_process_page_access_fifo_page_not_in_memory_no_frame_available() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[3] = set_pte(true, 10, 15, 15, 1);
        page_table[5] = set_pte(true, 20, 12, 16, 2);
        page_table[7] = set_pte(true, 30, 14, 14, 1);

        let page_number: usize = 2;
        let current_timestamp: i32 = 17;

        let frame = process_page_access_fifo(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }

        expected_page_table[2] = set_pte(true, 20, 17, 17, 1);
        expected_page_table[3] = set_pte(true, 10, 15, 15, 1);
        expected_page_table[7] = set_pte(true, 30, 14, 14, 1);

        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }

        assert_eq!(page_table.len(), 8);
        assert_eq!(frame_pool.len(), 0);
        assert_eq!(frame, 20);
    }

    #[test]
    pub fn test_count_page_faults_fifo() {
        let mut page_table: Vec<PTE> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        let reference_string = vec![0, 3, 2, 6, 3, 4, 5, 2, 4, 5, 6];
        let mut frame_pool: Vec<i32> = vec![0, 1, 2];

        let faults = count_page_faults_fifo(&mut page_table, reference_string, &mut frame_pool);

        assert_eq!(faults, 8);
    }

    #[test]
    pub fn test_count_page_faults_fifo_case2() {
        let mut page_table: Vec<PTE> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[2] = set_pte(true, 10, 3, 3, 1);
        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);

        let reference_string = vec![2, 5, 0, 7, 4, 2, 3, 5, 1, 2, 6, 0];
        let mut frame_pool: Vec<i32> = Vec::new();

        let faults = count_page_faults_fifo(&mut page_table, reference_string, &mut frame_pool);

        assert_eq!(faults, 10);
    }

    #[test]
    pub fn test_process_page_access_lru() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[2] = set_pte(true, 10, 3, 3, 1);
        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);
        let page_number: usize = 0;
        let current_timestamp: i32 = 12;

        let frame = process_page_access_lru(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }
        expected_page_table[0] = set_pte(true, 30, 12, 12, 1);
        expected_page_table[2] = set_pte(true, 10, 3, 3, 1);
        expected_page_table[5] = set_pte(true, 20, 2, 4, 2);
        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }
        assert_eq!(page_table.len(), 8);
        assert_eq!(frame, 30);
    }

    #[test]
    pub fn test_process_page_access_lru_page_in_memory() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[2] = set_pte(true, 10, 3, 3, 1);
        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);
        let page_number: usize = 2;
        let current_timestamp: i32 = 14;

        let frame = process_page_access_lru(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }

        expected_page_table[2] = set_pte(true, 10, 3, 14, 2);
        expected_page_table[5] = set_pte(true, 20, 2, 4, 2);
        expected_page_table[7] = set_pte(true, 30, 1, 1, 1);

        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }

        assert_eq!(page_table.len(), 8);
        assert_eq!(frame_pool.len(), 0);
        assert_eq!(frame, 10);
    }

    #[test]
    pub fn test_process_page_access_lru_page_not_in_memory_frame_available() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);

        let page_number: usize = 3;
        frame_pool.push(10);
        let current_timestamp: i32 = 15;

        let frame = process_page_access_lru(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }

        expected_page_table[3] = set_pte(true, 10, 15, 15, 1);
        expected_page_table[5] = set_pte(true, 20, 2, 4, 2);
        expected_page_table[7] = set_pte(true, 30, 1, 1, 1);

        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }

        assert_eq!(page_table.len(), 8);
        assert_eq!(frame_pool.len(), 0);
        assert_eq!(frame, 10);
    }

    #[test]
    pub fn test_process_page_access_lru_page_not_in_memory_no_frame_available() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[3] = set_pte(true, 10, 15, 15, 1);
        page_table[5] = set_pte(true, 20, 12, 16, 2);
        page_table[7] = set_pte(true, 30, 14, 14, 1);

        let page_number: usize = 2;
        let current_timestamp: i32 = 17;

        let frame = process_page_access_lru(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }

        expected_page_table[2] = set_pte(true, 30, 17, 17, 1);
        expected_page_table[3] = set_pte(true, 10, 15, 15, 1);
        expected_page_table[5] = set_pte(true, 20, 12, 16, 2);

        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }

        assert_eq!(page_table.len(), 8);
        assert_eq!(frame_pool.len(), 0);
        assert_eq!(frame, 30);
    }

    #[test]
    pub fn test_count_page_faults_lru() {
        let mut page_table: Vec<PTE> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        let reference_string = vec![0, 3, 2, 6, 3, 4, 5, 2, 4, 6, 5];
        let mut frame_pool: Vec<i32> = vec![0, 1, 2];

        let faults = count_page_faults_lru(&mut page_table, reference_string, &mut frame_pool);

        assert_eq!(faults, 9);
    }

    #[test]
    pub fn test_count_page_faults_lru_case2() {
        let mut page_table: Vec<PTE> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[2] = set_pte(true, 10, 3, 3, 1);
        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);

        let reference_string = vec![2,5,0,2,5,2,3,5,1,2,6,0];
        let mut frame_pool: Vec<i32> = Vec::new();

        let faults = count_page_faults_lru(&mut page_table, reference_string, &mut frame_pool);

        assert_eq!(faults, 6);
    }

    #[test]
    pub fn test_process_page_access_lfu() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[2] = set_pte(true, 10, 3, 3, 1);
        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);
        let page_number: usize = 0;
        let current_timestamp: i32 = 12;

        let frame = process_page_access_lfu(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }
        expected_page_table[0] = set_pte(true, 30, 12, 12, 1);
        expected_page_table[2] = set_pte(true, 10, 3, 3, 1);
        expected_page_table[5] = set_pte(true, 20, 2, 4, 2);
        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }
        assert_eq!(page_table.len(), 8);
        assert_eq!(frame, 30);
    }

    #[test]
    pub fn test_process_page_access_lfu_page_in_memory() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[2] = set_pte(true, 10, 3, 3, 1);
        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);
        let page_number: usize = 2;
        let current_timestamp: i32 = 14;

        let frame = process_page_access_lfu(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }

        expected_page_table[2] = set_pte(true, 10, 3, 14, 2);
        expected_page_table[5] = set_pte(true, 20, 2, 4, 2);
        expected_page_table[7] = set_pte(true, 30, 1, 1, 1);

        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }

        assert_eq!(page_table.len(), 8);
        assert_eq!(frame_pool.len(), 0);
        assert_eq!(frame, 10);
    }

    #[test]
    pub fn test_process_page_access_lfu_page_not_in_memory_frame_available() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);

        let page_number: usize = 3;
        frame_pool.push(10);
        let current_timestamp: i32 = 15;

        let frame = process_page_access_lfu(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }

        expected_page_table[3] = set_pte(true, 10, 15, 15, 1);
        expected_page_table[5] = set_pte(true, 20, 2, 4, 2);
        expected_page_table[7] = set_pte(true, 30, 1, 1, 1);

        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }

        assert_eq!(page_table.len(), 8);
        assert_eq!(frame_pool.len(), 0);
        assert_eq!(frame, 10);
    }

    #[test]
    pub fn test_process_page_access_lfu_page_not_in_memory_no_frame_available() {
        let mut page_table: Vec<PTE> = Vec::new();
        let mut frame_pool: Vec<i32> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[3] = set_pte(true, 10, 15, 15, 1);
        page_table[5] = set_pte(true, 20, 12, 16, 2);
        page_table[7] = set_pte(true, 30, 14, 17, 4);

        let page_number: usize = 2;
        let current_timestamp: i32 = 18;

        let frame = process_page_access_lfu(
            &mut page_table,
            page_number,
            &mut frame_pool,
            current_timestamp,
        );

        let mut expected_page_table: Vec<PTE> = Vec::new();
        for _i in 0..8 {
            expected_page_table.push(PTE::default());
        }

        expected_page_table[2] = set_pte(true, 10, 18, 18, 1);
        expected_page_table[5] = set_pte(true, 20, 12, 16, 2);
        expected_page_table[7] = set_pte(true, 30, 14, 17, 4);

        for i in 0..8 {
            assert_eq!(page_table[i], expected_page_table[i]);
        }

        assert_eq!(page_table.len(), 8);
        assert_eq!(frame_pool.len(), 0);
        assert_eq!(frame, 10);
    }

    #[test]
    pub fn test_count_page_faults_lfu() {
        let mut page_table: Vec<PTE> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        let reference_string = vec![0, 3, 2, 6, 3, 4, 5, 2, 6, 4, 5];
        let mut frame_pool: Vec<i32> = vec![0, 1, 2];

        let faults = count_page_faults_lfu(&mut page_table, reference_string, &mut frame_pool);

        assert_eq!(faults, 10);
    }

    #[test]
    pub fn test_count_page_faults_lfu_case2() {
        let mut page_table: Vec<PTE> = Vec::new();

        for _i in 0..8 {
            page_table.push(PTE::default());
        }

        page_table[2] = set_pte(true, 10, 3, 3, 1);
        page_table[5] = set_pte(true, 20, 2, 4, 2);
        page_table[7] = set_pte(true, 30, 1, 1, 1);

        let reference_string = vec![2,5,0,2,5,2,3,5,1,2,6,0];
        let mut frame_pool: Vec<i32> = Vec::new();

        let faults = count_page_faults_lfu(&mut page_table, reference_string, &mut frame_pool);

        assert_eq!(faults, 5);
    }

}
