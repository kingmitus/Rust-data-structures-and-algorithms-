pub fn print_array(arr: &[i32]){
    println!("{:?}", arr);
}

pub fn reverse_array(arr: &mut [i32]){
    arr.reverse();
    println!("{:?}", arr);
}

pub fn max_array(arr: &[i32]) {
    let max= arr.iter().max();
    match max {
        Some(&val) => println!("The maximum value is: {}", val),
        None       => println!("The array is empty!"),
    }
}

pub fn min_array(arr: &[i32]){
    let min = arr.iter().min();
    match min {
        Some(&val) => println!("The min value is: {}", val),
        None       => println!("The array is empty!"),
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n{
            for k  in (i+1)..n{
                if  nums[i] + nums[k] == target {
                    return vec![i as i32, k as i32];
                }
            }
        }
        vec![]
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty(){
            return String::new();
        }
        let mut prefix = strs[0].clone();
        for string in strs.iter().skip(1){
            while !string.starts_with(&prefix){
                if prefix.is_empty(){
                    return String::new();
                }
                prefix.pop();
            }
        }
        prefix
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.sort();
        nums.dedup();
        nums.len() as i32
    }
