fn main() {    let mut v = vec![1, 2, 3];    let ptr = v.as_mut_ptr();    unsafe {        *ptr = 10; //This is where the undefined behavior happens    }    println!("The first element is: {}", v[0]);}