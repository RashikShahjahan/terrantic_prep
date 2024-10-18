//Write a Rust function that computes the factorial of a number using both iterative and recursive approaches.
fn factorial_iterative(num:u32)->u32{
    let mut result = 1;
    if num == 0{
        return 1;
    }
    for curr in 1..=num{
        result*=curr;
    }
    return result;
}


fn factorial_recursive(num:u32)->u32{
    if num <= 1{
        return num;
    }
    return num*factorial_recursive(num-1);
}

fn main(){
    println!("{}",factorial_iterative(5));
    println!("{}",factorial_recursive(5));

}