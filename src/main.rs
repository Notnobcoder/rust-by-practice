// entry point --main
// println!("prints the value in the next line")


// fn main() {
//     println!("Hello, world!");
// }

// variables
// assigned using let keyword
// print -- prints a  Value
// println! -- prints a new values in println 
//

// fn main() {
//     // variable is only used when it is initialized
//     let x:i32 =3;
//     // its a macro which takes two arguments 
//     assert_eq!(x,5);
//     println!("program success")
//     // prorgam will compile
// }

// Scope
// means {} inside this funciton the value inintailzed is only valid for the inside argument
// //
//
// fn main() {
//     let a:i32 =5;
//     {
//         let b:i32 =4;
//         println!("the value of a is {} and b is {}", a,b)
//     }
// }


// Define other Functions
// ---note -- a funciton can be defined at any place before or after the main function becuse
// always the main funciton in initialid first afteht the other function
//
//
// fn definex() {
//     let x:&str = "tushar";
//     assert_eq!(x,"tushar");
//     println!("hello my name is {}", x)
// }
//
// fn definey() {
//     println!("this is the value of  y{}", 5)
// }
// fn main() {
//     definey();
//     definex();
// }


// Shadowing
// simply means redeclaring the variable
// means you can change the value of the previos variable by redeclaring the varialble
//
// fn main() {
//     let x:i32 =4;
//     let x =5;
//     println!("{}",x);
//
// }


// but use this example to understand the shandownd and mutabilty properly

// fn main () {
//     // declaring the mutable variable
//     let mut x =2;
//     // redeclaring the varialb -- shadowing
//     x+=9; // 11
//     println!("{x}")
// }


// ---- Question 
// why is even shadowing necessary
// The shadowing feature in Rust provides flexibility and control over variable names within different scopes. It allows you to declare a new variable with the same name as an existing one, effectively "shadowing" the outer variable within a specific scope.
// Shadowing is useful for situations where you need to change the type or value of a variable without having to come up with a new name. It eliminates naming conflicts and makes code more readable and expressive.
// Additionally, shadowing can be used to control the visibility and lifetime of variables. By shadowing a variable within a narrower scope, you can limit its accessibility and ensure it is only used where necessary.
// Overall, the shadowing feature in Rust enhances code clarity, avoids naming clashes, and offers finer control over variable usage within different scopes.


// -- How to allow unused variables

// #[allow(unused_variables)] // this alow key words tells the compiler diffrent things like usused variable
// fn main () {
//     let x:&str ="tushar";
// }



// -- Destructuring
// we can desctucture the values using tupple
// syntax let (x,y) =(2,4)
// fn main() {
//     let (a,b,c,d) =(1,2,3,4);
//     println!("{a}");
//     println!("{b}");
//     println!("{c}");
//     println!("{d}");
// }


// Destructuring assignment
// means you can use tuple slice and struct pattern means declaring the valiables in tupple
// but this needs 1.59 rust or higher
//
//

// fn main () {
//     let (x,y) ;
//     {
//         let x:i32 =25;
//         let y:i32 =10;
//         assert_eq!(x,25);
//         assert_eq!(y,10);
//     }
//     println!("{x} and {y}")
// }

// fn main () {
//     let (x,y);
//     (x,..) =(3,4);
//     (..,y) =(2,2);
//     println!("{x} and {y}")
// }
//
// 27:09 finished -------------------------
//
//
// First part completed
// you need to prepare some exercises




// Number -- Integer type
// two types
// Signed and Unsigned Integer
// length 8 16 32 64 128 arch
// signed i8 i16 i32 i64 i128 isize  -- can be negative can be positive integer
// unsigned u8 u16 u32 u64 u128 usize -- always positive integer


// -- means important
// default types in rust integer i32 and -- for float f64
//
// To understatnd this we have to know --binary number system
// for example 42
// 4 as 10 power 1 and 2 is 10 power 0
// -- by 10 because in decimal system we have -- 10 distinct digits to represent number --from 0 to
// 9
// means 4 is in the tens and 2 is in the ones like --- once tens hundred thousand and so on
// to solve this 
// 42
// (4*10 power1) + (2*10 power 0)
// (4*10) + (2*1) --note always 10 power 0 is always 1
// 40 + 2
// 42
// learn about binary number system
// range of 8 bit integer -- note smallest possible 8 bit integer is 0 and the largest possible
// 8 bit integer is 225
//
// range of 16 bit integer --note smallest 16 bit integer is 0 (unsiggned) and largest should be
// 65'535 (unsigned)
//
// Signed integers
// can represent negative integers -- use concept calles  --Two complement
// its a 3 step process 
// 1st use binary number
// 2nd reverse the binary number menaing 0 will become 1 and 1 will become 0
// and then +1 will be added for one's place
//
//
//
// there are 2 more topics
// exerecises 43:00
//

// we cannot mutate a variable to other if there bit system is not same -- you cannot assign
// x value whick is u38 to y which is i32 and so on
// fn main() {
//     let x:i32 =5;
//     let mut y:i32 = 10;
//     y+=x;
//     println!("{y}")
//
// }


// we can anotate type directly to a value also --syntax let x = 23_u8;

// fn main () {
//     let v= 38_u16;
//     println!("{v}")
// }

// for that we can use as keyword
// fn main () {
//     let v:u32=32_u16 as u32;
//     println!("{v}")
// }
//

// :: MAX keyword provides the maximum number of data type
// fn main() {
//     assert_eq!(i8::MAX,127_i8);
//     println!("success")
// }

// for loops in rust;
// fn main() {
//     let mut sum:i32 =0;
//     for i in 0..=2 {
//         sum +=i;
//     }
//     assert!(sum == 3);
//     println!("{sum}");
//
//     for c in 'a'..'d' {
//         println!("{c}")
//     }
// }
