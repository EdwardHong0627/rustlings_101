// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    use colored::*;
    
    let vec0 = vec![22, 44, 66];
    println!("----------------");
    println!("The validation of {}", "call by reference!".blue());
    
    println!("----------------");
    println!("The original vec memory address is {}", format!("{:p}", vec0.as_ptr()).green());
    println!("----------------");
    let vec1 = fill_vec(vec0);
    println!("The new vec after function call fill_vec is {:?}", vec1);
    // Memory address validation
    println!("----------------");
    println!("The new vec memory address is {}", format!("{:p}", vec1.as_ptr()).green());
}
    



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
