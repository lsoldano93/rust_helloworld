// use std::io::*;

const GREETING : &str = "-- Welcome to BS!";

// struct User
// {
//    name: String
// }

fn main() 
{
   println!("{}", GREETING);

   // Create input stream, prompt for name
   // let mut input = String::new();
   // println!(">> Enter your name: ");
   // stdin().read_line(&mut input).unwrap();

   // // name is no longer valid due to move into User
   // let user = User { name : input };
   // println!("-- Hello, {}", user.name);

   // // Process input to calculate, then calculate it
   // input = String::new();
   // println!("Say something below:");
   // stdin().read_line(&mut input).unwrap();
   // println!("-- You really said, \"{}\"", input);

   // Messing with vars
   goofin();
}

pub fn vars()
{
   let name = "Luke";
   let mut age = 28;
   age = age + 1;

   println!("My name is {} and I am {}", name, age);

   const ID : u32 = 001;
   println!("id: {}", ID);

   let (my_name, my_age) = ("Luke", 28);
   println!("{} is {}", my_name, my_age);

   let x = 1;

   let y = 2.5;

   let z: i64 = 3252623626;

   println!("The max value for i32 is: {}", std::i32::MAX);

   let is_active = true;
   let is_greater = 10 > 5;
   let a1 = 'a';
   let face = '\u{1F600}';

   println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}

pub fn goofin()
{
   let mut hello = String::from("Hello ");
   hello.push('w');
   hello.push_str("orld");


   println!("String: {}, is length {}, string cap is: {}", hello, hello.len(), hello.capacity());

   let blah = hello.replace("Hello", "worldz");
   let contains = blah.contains("worldz");
   println!("{:?}", (blah, contains));

   assert_eq!(1, 1);

   let person : (&str, &str, u8) = ("Luke", "Chicago", 28);
   println!("My name is {}, I'm from {} and am {} years old", person.0, person.1, person.2);

   let numbers : [i32; 5] = [1, 2, 3, 4, 5];
   println!("Array contents: {:?}", numbers);
}