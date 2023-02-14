#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // 001
    // println!("What is you name?");
    // let mut name = String::new();
    // let greeting = "Nice to meet you!";

    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("didn't receive inputs!");

    // println!("hello {}! {}", name.trim_end(), greeting);

    // 002
    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    // let age = "23";
    // let mut age: u32 = age.trim().parse().expect("age wasn't assigned a number");

    // age += 1;
    // println!("I'm {} and I want ${}", age, ONE_MIL);

    //003
    // println!("max u32: {}", u32::MAX);
    // println!("max u32: {}", u64::MAX);
    // println!("max u32: {}", u128::MAX);
    // println!("max u32: {}", usize::MAX);

    //004
    // let is_true = true;
    // let my_grade = 'A';

    //005
    // let num_1: f32 = 1.111111111111111;
    // println!("f32: {}", num_1 + 0.111111111111111);

    // let num_2: f64 = 1.111111111111111;
    // println!("f64: {}", num_2 + 0.111111111111111);

    //006
    // let num_1: u32 = 5;
    // let num_2: u32 = 4;

    // println!("5 + 3 = {}", num_1 + num_2);

    //007
    // let random_num = rand::thread_rng().gen_range(1..101);

    // println!("random number: {}", random_num);

    //008
    // let age = 8;
    // if (age >= 1) && (age <= 18) {
    //     println!("Import and Birthday");
    // }

    // let mut num = 47;
    // let is_true = num >= 18;

    // println!("num: {} is bigger than 18? -> {}", num, is_true);

    //009
    // let age = 8;
    // match age {
    //     1..=18 => println!("Import and Birthday"),
    //     21 | 50 => println!("Import and Birthday"),
    //     65..=u32::MAX => println!("Un Import and Birthday"),
    //     _ => println!("doesn't matter"),
    // };

    // let my_age = 18;
    // let voting_age = 18;
    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("can't vote!"),
    //     Ordering::Greater => println!("can vote!"),
    //     Ordering::Equal => println!("can absolutely vote!"),
    // };

    //010 -> arrays
    // let arr_1 = [1, 2, 2, 3];
    // let len = arr_1.len();

    // let mut i = 0;

    // loop {
    //     if i >= len {
    //         break;
    //     }

    //     println!("{}\n", arr_1[i]);
    //     i += 1;
    //     continue;
    // }

    // while i < len {
    //     println!("{}\n", arr_1[i]);
    //     i += 1;
    // }

    // for val in arr_1.iter() {
    //     println!("{} ", val);
    // }

    //011 -> tuples
    // let tup: (u8, String, f64) = (47, "name".to_string(), 10.2131);

    // println!("Name: {}, Age: {}", tup.1, tup.0);

    // 012 -> string
    // let mut str_1 = String::new();
    // str_1.push('0');
    // str_1.push_str(" -> zero");

    // for word in str_1.split_whitespace() {
    //     println!("{}", word);
    // }

    // let str_2 = str_1.replace("0", "1");
    // println!("{}", str_2);

    // 013
    // let str = String::from("x, r, t, b, h, k, k, a, m, c");
    // let mut v1: Vec<char> = str.chars().collect();

    // v1.sort();
    // v1.dedup();
    // for char in v1 {
    //     println!("{}", char);
    // }

    // let str1: &str = "Random string";
    // let mut str2 = str1.to_string();
    // println!("{}", str2);

    // let byte_arr = str2.as_bytes();
    // for char in byte_arr {
    //     println!("{}", char);
    // }

    // let str3 = &str2[0..3];
    // println!("{}", str3);

    // let str5 = String::from("helo wrld");
    // let str6: String = String::from(" anything");
    // let str4 = str5 + &str6;

    // println!("{}", str4);

    ////////////////////////////
    // let int_u8: u8 = 5;
    // let int2_u8: u8 = 4;
    // let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    ///////////////////////////
    // enum Day {
    //     Monday,
    //     Tuesday,
    //     Wednesday,
    //     Saturday,
    //     Sunday,
    // }

    // impl Day {
    //     fn is_weekend(&self) -> bool {
    //         match self {
    //             Day::Saturday | Day::Sunday => true,
    //             _ => false,
    //         }
    //     }
    // }

    // let today: Day = Day::Monday;
    // match today {
    //     Day::Monday => println!("Everyone hates Mondays!"),
    //     _ => println!("I don't know what to do"),
    // };

    // println!("Is today weekend: {}", today.is_weekend());

    //////////////////////////////
    // let vec1: Vec<i32> = Vec::new();
    // let mut vec2: Vec<i32> = vec![1, 2, 3, 4];
    // vec2.push(5);
    // println!("pushed element: {}", vec2[vec2.len() - 1]);

    // let second: &i32 = &vec2[1];
    // match vec2.get(1) {
    //     Some(second) => println!("2nd: {}", second),
    //     None => println!("No second vlues"),
    // };

    // for i in &mut vec2 {
    //     *i *= 2;
    // }

    // for i in &vec2 {
    //     println!("{} ", i);
    // }

    // println!("popped: {:?}", vec2.pop());
}
