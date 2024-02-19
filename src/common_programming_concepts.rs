pub fn variables_and_mutability() {
    println!("1. Mutability");
    let mut x: i32 = 5;
    println!("The value is {x}");
    x = 6;
    println!("The value is {x}");

    println!("2. Constants");
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    println!("3. Shadowing");
    {
        let x: i32 = x * 2;
        println!("Inner value is {x}");
    }
    println!("Outer value is {x}");
}
