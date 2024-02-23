static GIFT_ARRAY: [&str; 12] = [
    "partridge in a pear tree",
    "turtle doves",
    "French hens",
    "calling birds",
    "gold rings",
    "geese a-laying",
    "swans a-swimming",
    "maids a-milking",
    "ladies dancing",
    "lords a-leaping",
    "pipers piping",
    "drummers drumming",
];

pub fn print_lyrics() {
    let mut n: u32 = 1;
    while n < 13 {
        println!("{}", get_nth_day_lyrics(n));
        n = n + 1;
    }
}

pub fn get_nth_day(n: u32) -> String {
    let n_str: String = n.to_string();
    if n == 1 {
        String::from("1st")
    } else if n == 2 {
        String::from("2nd")
    } else if n == 3 {
        String::from("3rd")
    } else {
        n_str + "th"
    }
}

pub fn get_nth_gift(n: u32) -> String {
    let n_size = n as usize;
    if n == 1 {
        format!("a {}.\n", GIFT_ARRAY[0])
    } else if n == 2 {
        format!("2 {},\nand a {}.\n", GIFT_ARRAY[1], GIFT_ARRAY[0])
    } else {
        format!(
            "{} {},\n{}",
            n,
            GIFT_ARRAY[n_size - 1],
            get_nth_gift(n - 1)
        )
    }
}

pub fn get_nth_day_lyrics(n: u32) -> String {
    if n < 1 || n > 12 {
        String::from("n must between 1 and 12.")
    } else {
        format!(
            "On the {} day of Christmas,\nmy true love sent to me\n{}",
            get_nth_day(n),
            get_nth_gift(n)
        )
    }
}
