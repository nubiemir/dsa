use arrays::{duplicate_zeros, remove_duplicates};

use crate::arrays::check_if_exist;

mod arrays;

fn main() {
    let mut x = vec![10, 2, 5, 3];

    let y = check_if_exist(x);
    println!("{}", y);
}
