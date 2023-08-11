pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    // max variable will be used as the return as the consecutive values will be stored there
    // i variable will be used as the counter for the consecutive values
    // loop through each value in vector
    // check if the value is one the i counter will increment
    // if the value is different from one and i counter is greater than max:
        // set max to the value of the i counter
        // reinitialize the i counter to 0 for continuing the iteration
    // if the value is neither 1 or i is < max set i counter to 0
    // finally return max
    
    let mut max = 0;
    let mut i = 0;
    for val in nums {
        match val {
            1 => i += 1,
            x if (x != 1) && i > max => {
                max = i;
                i = 0;
            }
            _ => i = 0,
        }
    }
    if i > max {
        max = i;
    }
    max
}

pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut i = 0;

    for val in nums {
        match is_even_digit(&val) {
            true => i += 1,
            _ => (),
        }
    }

    i
}

fn is_even_digit(val: &i32) -> bool {
    let digit = ((*val as f32).log10() as i32) + 1;
    if digit % 2 == 0 {
        return true;
    }
    false
}

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut res = vec![0; nums.len()];

    for i in (0..nums.len()).rev() {
        if nums[left].abs() > nums[right].abs() {
            res[i] = i32::pow(nums[left], 2);
            left += 1;
        } else {
            res[i] = i32::pow(nums[right], 2);
            right -= 1;
        }
    }

    res
}

pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let length = arr.len();
    let mut index: i32 = 0;
    let mut zero_counter = 0;

    loop {
        if arr[index as usize] == 0 && index + zero_counter < (length - 1) as i32 {
            zero_counter += 1;
        }
        if index + zero_counter >= (length - 1) as i32 {
            break;
        }
        index += 1;
    }

    while zero_counter > 0 && index >= 0 {
        if arr[index as usize] == 0 && index + zero_counter < (length - 1) as i32 {
            arr[index as usize + zero_counter as usize] = arr[index as usize];
            zero_counter -= 1;
            arr[index as usize + zero_counter as usize] = arr[index as usize];
        } else {
            arr[index as usize + zero_counter as usize] = arr[index as usize];
        }
        index -= 1;
    }
}


pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
           let mut ptr1: i32 = m - 1;
           let mut ptr2: i32 = n - 1;

            loop{
                if (ptr1 < 0) || (ptr2 < 0) || (n==0) {
                    break;
                }

                if nums1[ptr1 as usize] > nums2[ptr2 as usize]{
                    nums1[ptr1 as usize + ptr2 as usize + 1] = nums1[ptr1 as usize];
                    ptr1 -= 1;
                }else {
                    nums1[ptr1 as usize + ptr2 as usize + 1] = nums2[ptr2 as usize];
                    ptr2 -= 1;
                }
            }
            loop {
                if ptr2 <  0 {
                    break;
                }
                nums1[ptr2 as usize] = nums2[ptr2 as usize];
                ptr2 -= 1;
            }
}


pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut ptr1: i32 = 0;
        let mut ptr2: i32 = (nums.len() - 1) as i32;

        loop {
            
            if ptr1 > ptr2 {
                return ptr1;
            }

            if nums[ptr1 as usize] == val {
                if nums[ptr2 as usize] == val {
                    nums[ptr2 as usize] = 0;
                    ptr2 -= 1;
                    continue;
                }
                nums[ptr1 as usize] = nums[ptr2 as usize];
                nums[ptr2 as usize] = 0;
                ptr2 -= 1;
                continue;           
            }
            ptr1 += 1;
        }
}


#[test]
fn find_max_consecutive_ones_test() {
    let x = vec![-4, -1, 0, 3, 10];
    let res = [0, 0, 0, -4, 10];

    let nums = vec![1, 1, 0, 1, 1, 1];
    let nums2 = vec![1, 0, 1, 1, 0, 1];
    assert_eq!(find_max_consecutive_ones(nums), 3);
    assert_eq!(find_max_consecutive_ones(nums2), 2);
}

#[test]

fn find_numbers_test() {
    let nums = vec![12, 345, 2, 6, 7896];
    let nums2 = vec![555, 901, 482, 1771];
    assert_eq!(find_numbers(nums), 2);
    assert_eq!(find_numbers(nums2), 1);
}
