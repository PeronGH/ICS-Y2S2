#[derive(Debug, Clone)]
pub struct BankersAlgorithm {
    available: Vec<i32>,
    allocation: Vec<Vec<i32>>,
    need: Vec<Vec<i32>>,
}

impl BankersAlgorithm {
    pub fn new(available: Vec<i32>, max: Vec<Vec<i32>>, allocation: Vec<Vec<i32>>) -> Self {
        let need = allocation
            .iter()
            .zip(max.iter())
            .map(|(alloc_row, max_row)| {
                alloc_row
                    .iter()
                    .zip(max_row.iter())
                    .map(|(alloc, max)| max - alloc)
                    .collect()
            })
            .collect();

        BankersAlgorithm {
            available,
            allocation,
            need,
        }
    }

    // Safety Check Function
    pub fn is_safe(&self) -> bool {
        let mut work = self.available.clone();
        let mut finish = vec![false; self.need.len()];

        loop {
            let mut found = false;
            for i in 0..self.need.len() {
                if finish[i] {
                    continue;
                }

                if self.need[i].iter().zip(work.iter()).all(|(n, w)| n <= w) {
                    for j in 0..work.len() {
                        work[j] += self.allocation[i][j];
                    }
                    finish[i] = true;
                    found = true;
                }
            }

            if !found {
                break;
            }
        }

        // If all processes could be brought to completion, the state is safe
        finish.into_iter().all(|f| f)
    }

    // Resource Request Algorithm
    pub fn request_resources(
        &mut self,
        process_index: usize,
        request: &[i32],
    ) -> Result<(), &'static str> {
        // First, check if the request exceeds the need
        if request
            .iter()
            .zip(self.need[process_index].iter())
            .any(|(r, n)| r > n)
        {
            return Err("Error: Request exceeds need.");
        }

        // Then, check if the request exceeds the available resources
        if request
            .iter()
            .zip(self.available.iter())
            .any(|(r, a)| r > a)
        {
            return Err("Error: Request exceeds available resources.");
        }

        // Tentatively allocate the resources for this request
        for i in 0..request.len() {
            self.available[i] -= request[i];
            self.allocation[process_index][i] += request[i];
            self.need[process_index][i] -= request[i];
        }

        // Check if the new state is safe
        if !self.is_safe() {
            // If not, rollback the allocation and deny the request
            for i in 0..request.len() {
                self.available[i] += request[i];
                self.allocation[process_index][i] -= request[i];
                self.need[process_index][i] += request[i];
            }

            return Err("Error: Request would lead to unsafe state.");
        }

        // If we've made it this far, the request can be granted safely
        Ok(())
    }
}

#[test]
fn test_bankers_algorithm() {
    let available = vec![3, 3, 2];
    let max = vec![
        vec![7, 5, 3],
        vec![3, 2, 2],
        vec![9, 0, 2],
        vec![2, 2, 2],
        vec![4, 3, 3],
    ];
    let allocation = vec![
        vec![0, 1, 0],
        vec![2, 0, 0],
        vec![3, 0, 2],
        vec![2, 1, 1],
        vec![0, 0, 2],
    ];

    let banker = BankersAlgorithm::new(available, max, allocation);
    println!("{:?}", banker);

    assert!(banker.is_safe());

    // Request resources
    assert!(banker.clone().request_resources(1, &[1, 0, 2]).is_ok());
    assert!(banker.clone().request_resources(4, &[3, 3, 0]).is_err());
    assert!(banker.clone().request_resources(0, &[0, 2, 0]).is_ok());
}
