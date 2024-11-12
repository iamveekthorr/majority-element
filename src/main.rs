fn main() {
    let nums = vec![3, 3, 4];

    majority_element(nums);
}

// Boyer-Moore Voting Algorithm
fn majority_element(nums: Vec<i32>) -> i32 {
    //  Set up a variable to store a candidate for the majority element and a counter to track its frequency.
    // 	Iterate through the array. If the counter is zero, set the current element as the candidate.
    // 	If the current element matches the candidate, increase the counter; otherwise, decrease it.
    // 	By the end of the iteration, the candidate will be the majority element.
    let mut counter = 0;
    let mut candidate = 0;

    for &el in nums.iter() {
        if counter == 0 {
            candidate = el;
            // When using the Boyer-Moore Voting Algorithm,
            // always increment the counter
            counter += 1;
        } else if el == candidate {
            counter += 1;
        } else {
            counter -= 1;
        }
    }

    return candidate;
}
