use std::io;
use std::process;

fn main() {
    // mut mutation 让变量 name 成为可变的
    let mut name = "Eric Chen";
    println!("{}", name);

    name = "Alice Zh";
    println!("{}", name);

    let age = 18;
    let another_age = 20;
    println!("{} and {}", age, another_age);

    let first_name: String = "Eric".to_string();
    let last_name: String = "Chen".to_string();
    println!("{0} {1}", first_name, last_name);

    // 函数单次调用
    say_name(first_name, last_name);

    // 函数多次调用
    let cat_name = "Mimi".to_string();
    say_cat_name(&cat_name);
    say_cat_name(&cat_name);

    // 从终端中询问值
    say_line_name();

    // 处理错误情况
    // 加上 loop，会循环调用函数
    // loop {
    calculate_number();
    // }
}

// 函数调用
fn say_name(first_name: String, last_name: String) {
    println!("{} {}", first_name, last_name);
}
// cat_name 变成引用值，实现多次调用
fn say_cat_name(cat_name: &String) {
    println!("{}", cat_name);
}
// 从终端中询问值
fn say_line_name() {
    println!("Please enter your name: ");
    let mut line_name = String::new();
    io::stdin()
        .read_line(&mut line_name)
        .expect("Failed to read input");
    println!("Hello, {}", line_name);
}
// 处理错误输入情况
fn calculate_number() {
    println!("Please enter a first number: ");
    let a = read_user_input();

    println!("Please enter a second number: ");
    let b = read_user_input();

    println!("{} + {} = {}", a, b, a + b);
}
// 代码优化，从终端读取数据
fn read_user_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse() {
        Ok(val) => {
            if val == -1 {
                println!("Exiting program...");
                process::exit(0);
            } else {
                val
            }
        }
        Err(_err) => {
            println!("Not a valid number!");
            process::exit(1)
        }
    }
}
