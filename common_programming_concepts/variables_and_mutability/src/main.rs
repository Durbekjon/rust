// fn incorrect() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }
// We can not mutate variable

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }
// but now we're able to mutate X by using "mut"

// fn main() {
//     let x: i8 = 6;
//     let x: i8 = x + 1;
//     {
//         let x: i8 = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// we are not able to mutate variable type when we are using mut 
//   fn main() {
//     let mut spaces = "   ";
//     spaces = spaces.len();
// }


fn main() {

}