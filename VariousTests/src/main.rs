// use core::num;
use std::io;
fn main() {
    // testing shadowing
    let x: i8 = 2;
    let x: String = "hola".to_string();
    // testing var mutability
    let mut y: i8 = 8;
    y = 10;
    println!("{x} and {y}");
    //testing loops and vectors
    let new_vector: Vec<i32> = looping(5);
    println!("{:?}", new_vector);
    //testing the farenheit and celcius functions
    //input from the user
    let temperature: String = user_input("Enter your temperature to change");
    let temperature: f32 = temperature.trim().parse::<f32>().unwrap_or(0.0);
    println!(
        "{} celcius is {} farenheit",
        temperature,
        farenheit(temperature)
    );
    println!(
        "{} farenheit is {} celcius",
        temperature + 20.0,
        celcius(temperature + 20.0)
    );
    //testing fibbonaci input and functions
    //fibo is recursive
    //fibo is a loop
    println!("Which Fibonacci number do you want?");
    let mut fibo_num = String::new();
    io::stdin()
        .read_line(&mut fibo_num)
        .expect("Failed to read line");
    let fibo_num: i32 = match fibo_num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("The fib number {0} is {1}", fibo_num, fibo2(fibo_num));
    println!("The fib number {0} is {1}", fibo_num, fibo(fibo_num));

    //testing my input function
    let rnd_int:i32 = testing_my_knowledge("Type an int");
    println!("This is your int: {}",rnd_int);
    //finishing the program
    println!("Press enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn looping(array_len: i32) -> Vec<i32> {
    let mut ouput_array: Vec<i32> = vec![10; array_len as usize];
    let mut index: usize = 0;
    while index < ouput_array.len() {
        ouput_array[index] = (index * 2) as i32;
        index += 1;
    }
    ouput_array
}
fn celcius(f: f32) -> f32 {
    let c: f32 = (f - 32.0) * 5.0 / 9.0;
    c
}

fn farenheit(c: f32) -> f32 {
    let f: f32 = (c * 9.0 / 5.0) + 32.0;
    f
}

fn fibo(n: i32) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibo(n - 1) + fibo(n - 2)
}

fn fibo2(n: i32) -> i64 {
    /*function that
    computes the nth fibo
    number.
    */
    let mut f_1: i64 = 0;
    let mut f_2: i64 = 1;
    if n == 0 {
        return f_1;
    }

    for _i in 1..n {
        let temp = f_1 + f_2;
        f_1 = f_2;
        f_2 = temp;
    }
    f_2
}

fn user_input(shown_text: &str) -> String {
    println!("{shown_text}");
    let mut user_text = String::new();
    io::stdin()
        .read_line(&mut user_text)
        .expect("Something Went Wrong");
    return user_text;
}

fn testing_my_knowledge(text_to_user: &str) -> i32 {
    //print some text to tell the user what to do
    println!("{}",text_to_user);
    //create a variable to read that text
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).
    expect("expected an input");
    user_input.trim().parse::<i32>().unwrap_or(0)
}
