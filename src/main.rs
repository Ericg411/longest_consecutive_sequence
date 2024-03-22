use std::{cmp::max, collections::HashSet};

fn main() {
    println!("Hello, world! Run 'cargo test' in order to run test scenarios");
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let hash_set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
    let mut answer: i32 = 1;
    for i in &hash_set {
        if hash_set.contains(&(i - 1)) {
            continue;
        }

        let mut counter: i32 = 1;
        while hash_set.contains(&(i + counter)) {
            counter += 1;
        }
        answer = max(counter, answer);
    }

    answer
}

#[cfg(test)]
mod tests {
    use crate::longest_consecutive;
    
    #[test]
    fn test_case_one() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }

    #[test]
    fn test_case_two() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(longest_consecutive(nums), 9);
    }
}