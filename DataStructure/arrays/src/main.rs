

mod array;  // array file
fn main() {
    let arr1 = [1,2,3,4,5];
    array::print_array(&arr1);

    let mut arr2: [i32; 5] = [1,2,3,4,5];
    array::reverse_array(&mut arr2);

    array::max_array(&arr1);

    array::min_array(&arr1);

    let nums = vec![1,2,3,4,5,6,7,8,9,10];
    let target = 6;
    let result = array::two_sum(nums, target);
    println!("Output: {:?}", result);

    let words = vec!{
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    };

    let prefix_result = array::longest_common_prefix(words);
    println!("lcp: {:?}", prefix_result);
}
