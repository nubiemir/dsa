pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
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

pub fn duplicate_zeros(arr: &mut Vec<i32>) {}

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
