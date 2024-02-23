use std::any::type_name;

pub fn variables_and_mutability() {
    println!(">>> Variables and Mutability");

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
    println!("<<<")
}

pub fn scalar_data_types() {
    println!(">>> Scalar Data Types");

    {
        println!("1. Integer");

        let i8_min: i8 = std::i8::MIN;
        let i8_max: i8 = std::i8::MAX;
        println!("i8 minimum is {}, i8 maximum is {}", i8_min, i8_max);

        let u8_min: u8 = std::u8::MIN;
        let u8_max: u8 = std::u8::MAX;
        println!("u8 minimum is {}, u8 maximum is {}", u8_min, u8_max);

        let i16_min: i16 = std::i16::MIN;
        let i16_max: i16 = std::i16::MAX;
        println!("i16 minimum is {}, i16 maximum is {}", i16_min, i16_max);

        let u16_min: u16 = std::u16::MIN;
        let u16_max: u16 = std::u16::MAX;
        println!("u16 minimum is {}, u16 maximum is {}", u16_min, u16_max);

        let i32_min: i32 = std::i32::MIN;
        let i32_max: i32 = std::i32::MAX;
        println!("i32 minimum is {}, i32 maximum is {}", i32_min, i32_max);

        let u32_min: u32 = std::u32::MIN;
        let u32_max: u32 = std::u32::MAX;
        println!("u32 minimum is {}, u32 maximum is {}", u32_min, u32_max);

        let i64_min: i64 = std::i64::MIN;
        let i64_max: i64 = std::i64::MAX;
        println!("i64 minimum is {}, i64 maximum is {}", i64_min, i64_max);

        let u64_min: u64 = std::u64::MIN;
        let u64_max: u64 = std::u64::MAX;
        println!("u64 minimum is {}, u64 maximum is {}", u64_min, u64_max);

        let i128_min: i128 = std::i128::MIN;
        let i128_max: i128 = std::i128::MAX;
        println!("i128 minimum is {}, i128 maximum is {}", i128_min, i128_max);

        let u128_min: u128 = std::u128::MIN;
        let u128_max: u128 = std::u128::MAX;
        println!("u128 minimum is {}, u128 maximum is {}", u128_min, u128_max);

        println!("Handling overflow");
        let b: u8 = 2;
        {
            let mut a: u8 = u8_max;
            println!("a is {a}");
            a = a + b;
            println!("a plus 2 is {a}");
        }
        {
            let mut a: u8 = u8_max;
            println!("a is {a}");
            a = a.wrapping_add(b);
            println!("a wrapping add 2 is {a}");
        }
        {
            let a: u8 = u8_max;
            println!("a is {a}");
            let res: Option<u8> = match a.checked_add(b) {
                None => None,
                Some(num) => Some(num),
            };
            match res {
                None => println!("Overflow occurred"),
                Some(val) => println!("a checked add 2 is {val}"),
            }
        }
    }

    {
        println!("2. Float Point Types");

        let f64_num: f64 = 42.0;
        println!("Type of {f64_num} is {}", type_name::<f64>());
        let f32_num: f32 = 42.0;
        println!("Type of {f32_num} is {}", type_name::<f32>());
    }

    {
        println!("3. Numerical Operations");

        let a: i32 = -5;
        let b: i32 = 3;
        let sum: i32 = a + b;
        let diff: i32 = a - b;
        let prod: i32 = a * b;
        let quot: i32 = a / b;
        let remain: i32 = a % b;
        println!(
            "a is {a}, b is {b}, \
            sum is {sum}, \
            difference is {diff}, \
            production is {prod}, \
            quotation is {quot}, \
            remainder is {remain}."
        );
    }

    {
        println!("4. The Boolean Type");
        let t: bool = true;
        let f: bool = false;
        println!("{t}, {f}");

        println!("5. The Character Type");
        let c: char = 'z';
        let z: char = 'ℤ';
        let heart_eyed_cat: char = '😻';
        println!("Examples: {c}, {z}, {heart_eyed_cat}");

        let hex: Option<char> = std::char::from_u32(0x1111);
        match hex {
            None => println!("The value can not transfer to char."),
            Some(char) => println!("Char from hex: {char}"),
        }

        fn iter_all_chars() {
            println!("Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive");
            // Iterate over all possible Unicode scalar values
            for i in 0..=0x10FFFF {
                // Attempt to convert the scalar value to a char
                if let Some(_ch) = std::char::from_u32(i) {
                    // TODO: save to file
                    // println!("Unicode Scalar Value: {:X}, Character: {}", i, ch);
                    continue;
                }
            }
        }
        iter_all_chars();
    }

    println!("<<<")
}

pub fn functions_and_control_flow() {
    println!(">>> Functions");
    println!(
        "- Statements are instructions that perform some action and do not return a value. \
        - Expressions evaluate to a resultant value and return the value. \
        - Expressions do not include ending semicolons."
    );

    let y: i32 = {
        let x: i32 = 3;
        x + 1 // Expressions do not include ending semicolons and return the value!
    };

    println!("The value of y is: {y}");
    println!("<<<")
}
