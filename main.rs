use std::io::*;

const GREETING : &str = "-- Welcome to BS!";

struct User
{
   name: String
}

fn main() 
{
   println!("{}", GREETING);

   // Create input stream, prompt for name
   let mut input = String::new();
   println!(">> Enter your name: ");
   stdin().read_line(&mut input).unwrap();

   // name is no longer valid due to move into User
   let user = User { name : input };
   println!("-- Hello, {}", user.name);

   // Process input to calculate, then calculate it
   input = String::new();
   println!("Say something below:");
   stdin().read_line(&mut input).unwrap();
   println!("-- You really said, \"{}\"", input);
}