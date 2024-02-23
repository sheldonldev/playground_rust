pub fn generate_fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        generate_fibonacci(n-1) + generate_fibonacci(n-2)
    }
}

pub fn test_generate_fibonacci() {
    let mut n: u32 = 0;
    loop {
        n = n+1;
        println!("{n}th fibonacci is {}", generate_fibonacci(n));
        if n > 10 {
            break;
        }
    }
}