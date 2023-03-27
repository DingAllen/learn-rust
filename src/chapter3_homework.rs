// 打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分（编写循环）
pub fn print_christmas_song() {
    let mut day = 1;
    while day <= 12 {
        println!("On the {} day of Christmas", day);
        println!("My true love sent to me");
        match day {
            1 => println!("A partridge in a pear tree"),
            2 => println!("Two turtle doves"),
            3 => println!("Three french hens"),
            4 => println!("Four calling birds"),
            5 => println!("Five golden rings"),
            6 => println!("Six geese a-laying"),
            7 => println!("Seven swans a-swimming"),
            8 => println!("Eight maids a-milking"),
            9 => println!("Nine ladies dancing"),
            10 => println!("Ten lords a-leaping"),
            11 => println!("Eleven pipers piping"),
            12 => println!("Twelve drummers drumming"),
            _ => println!(""),
        }
        day += 1;
    }
}

// 生成n阶斐波那契数列
pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// 相互转换摄氏和华氏温度
pub fn parse_temperature() {
    let c = 100.0;
    let f = 222.0;
    println!("{} celsius is {} fahrenheit", c, celsius_to_fahrenheit(c));
    println!("{} fahrenheit is {} celsius", f, fahrenheit_to_celsius(f));
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    1.8 * c + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}