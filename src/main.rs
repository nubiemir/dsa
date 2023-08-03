use arrays::duplicate_zeros;

mod arrays;

fn main() {
    let mut x = vec![1, 5, 2, 0, 6, 8, 0, 6, 0];

    let (y, z) = duplicate_zeros(&mut x);
    println!("incr ==> {}, index ==> {}", y, z);

    //     let res = duplicate_zeros(&mut x);
    //     println!("res => {:?}", res);
}
