mod problems;

use crate::problems::p665;


fn main() {
    let result = p665::check_possibility(vec![4,2,1]);
    println!("{}", result);
}
