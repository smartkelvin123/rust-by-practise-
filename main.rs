// fn main() {
//     println!("Hello, world!");
// }

// /

// Fix the error below with least amount of modification to the code
// fn main() {
//     let x: i32; // Uninitialized but used, ERROR !
//     let y: i32; // Uninitialized but also unused, only a Warning !

//     assert_eq!(x, 5);
//     println!("Success!");
// }

// fn main() {
//     let x: i32 = 5; // uninitialized but using, ERROR !
//     let y: i32; // uninitialized but also unusing, only warning
//     println!("{} is equal to 5", x);
// }

// Fix the error below with least amount of modification
// fn main() {
//     let x: i32 = 10;
// }
//         let y: i32 = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
    
//     }
//     println!("The value of y is {}" , y ); 
// }


// fn main() {
//     let x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
//     }
//     println!("The value of x is {}", x);
// }


// Fix the error with the use of define_x
// fn main() {
//     println!("{}, world", x); 
// }

// fn define_x() {
//     let x = "hello";
// }

// fn main() {
//     let x = define_x();
//     println!("{}, world", x);
// }

// fn define_x() -> String {
//     let x = "hello".to_string();
//     x
// }


// SHOWDONING
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 5);
//     }
//     assert_eq!(x, 12);
//     let x = 42;
//     println!("{}", x); // Prints "42".
// }
   


// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }

//     assert_eq!(x, 5);

//     let x = 42;
//     println!("{}", x); // Prints "42".
// }

// Remove a line in the code to make it compile
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // Shadowing and re-binding
//     let x = x; 
//     // x += 3;


//     let y = 4;
//     // Shadowing
//     let y = "I can also be bound to text!"; 

//     println!("Success!");
// }

// destructuring
// Fix the error below with least amount of modification
// fn main() {
//     let (x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);

//     println!("Success!");
// }

// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
//     println!("fork!");
// }



// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     [..,z]  = [6, 8];
//     // Fill the blank to make the code work
//     assert_eq!([x,y,z], __);

//     println!("Success!");
// }

// fn main() {
//     let (x, y, z);
//     (x, ..) = (3, 4);
//     [.., y] = [1, 2];
//     [z,..]  = [6, 8];
//     // fill the blank to make the code work
//     assert_eq!([x, y, z], [3, 2,6]);
//     println!("Success!");
// }
// NUMBER
// interger



// Remove something to make it work
// fn main() {
//     let x: i32 = 5;
//     let mut y: u32 = 5;

//     y = x;
    
//     let z = 10; // Type of z ? 

//     println!("Success!");
// }
// fn main() {
//     let x: i32 = 5;
//     let mut y = 5;

//     y = x;
    
//     let z = 10; // type of z : i32
//     println!("Success!");
// }


//  Fill the blank
// fn main() {
//     let v: u16 = 38_u8 as __;

//     println!("Success!");
// }
// fn main() {
//     let v: u16 = 38_u8 as u16;
//     println!("Success!");
// }


// Modify `assert_eq!` to make it work
// fn main() {
//     let x = 5;
//     assert_eq!("u32".to_string(), type_of(&x));
//     println!("Success!");
// }
// // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// fn main() {
//     let x = 5;
//     assert_eq!("i32".to_string(), type_of(&x));
//     println!("Success!");
// }

// // get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// Fill the blanks to make it work
// fn main() {
//     assert_eq!(i8::MAX, __); 
//     assert_eq!(u8::MAX, __); 

//     println!("Success!");
// }
// fn main() {
//     assert_eq!(i8::MAX, 127); 
//     assert_eq!(u8::MAX, 255); 
//     println!("Success!");
// }

// Modify `assert!` to make it work
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1579);
//     println!("Success!");

// // }
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1597);
//     println!("Success!");
//  }

// Floating-Point
//  Replace ? with your answer
// fn main() {
//     let x = 1_000.000_1; // ?
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64

//     println!("Success!");
// }
// fn main() {
//     let x = 1_000.000_1; // f64
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64;// f64
//     println!("Success!");
//  }

// Range
// Two goals: 1. Modify assert! to make it work 2. Make println! output: 97 - 122
// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -3);

//     for c in 'a'..='z' {
//         println!("{}",c);
//     }
// }

// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -5);

//     for c in 'a'..='z' {
//         println!("{}",c as u8);
//     }
// }

// Fill the blanks
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..__), Range{ start: 1, end: 5 });
//     assert_eq!((1..__), RangeInclusive::new(1, 5));

//     println!("Success!");
// }

// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range{ start: 1, end: 5 });
//     assert_eq!((1..=5), RangeInclusive::new(1, 5));
//     println!("Success!");
// }

// Fill the blanks and fix the errors
// fn main() {
//     // Integer addition
//     assert!(1u32 + 2 == __);

//     // Integer subtraction
//     assert!(1i32 - 2 == __);
//     assert!(1u8 - 2 == -1); 
    
//     assert!(3 * 50 == __);

//     assert!(9.6 / 3.2 == 3.0); // error ! make it work

//     assert!(24 % 5 == __);
//     // Short-circuiting boolean logic
//     assert!(true && false == __);
//     assert!(true || false == __);
//     assert!(!true == __);

//     // Bitwise operations
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }

// fn main() {
//     // Integer addition
//     assert!(1u32 + 2 == 3);

//     // Integer subtraction
//     assert!(1i32 - 2 == -1);
//     assert!(1i8 - 2 == -1);
    
//     assert!(3 * 50 == 150);

//     assert!(9 / 3 == 3); // error ! make it work

//     assert!(24 % 5 == 4);
//     // Short-circuiting boolean logic
//     assert!(true && false == false);
//     assert!(true || false == true);
//     assert!(!true == false);

//     // Bitwise operations
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }