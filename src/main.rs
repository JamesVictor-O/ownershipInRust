// fn main(){
//        let mut s=String::from("Hello, world!");
//        let len= calculate_length(&mut s);
//        println!("The length of '{}' is {}.", s, len);
// }

// fn calculate_length(s:  &mut String) -> usize {
//     s.push_str("?"); 
//     println!("The string is now: {}", s);
//     s.len()
// }

// fn main() {
//     let s = String::from("Hello, world!");
//     take_ownership(s);
// }

// fn take_ownership(some_string: String) {
//     println!("Borrowing: {}", some_string);
// }

// --------------------------------------borrowing
//  fn main() {
//     let s = String::from("Hello, world!");
//     let len = calculate_length(&s); 
//     println!("The length of '{}' is {}.", s, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }



//  fn main(){
//     let s1=String::from("Hello, world!");
//     let len=calculate_length(&s1);

//     println!("The length of is {}, {}.", s1,len);
//  }


//  fn calculate_length(s1: &String) -> usize {
//     s1.len()
//  }


//  fn main(){
//    let mut s = String::from("hello");
    
//    {
//       let r1=&s;
//       println!("{r1}")
//    }
//     let r2=&s;
//     println!("{}", r2)
//  }

//  fn change(some_string: &mut String){
//     some_string.push_str(", world");
//  }

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> String {
//     let s = String::from("hello");

//     s
// }


// ---------------------------------------slice

// fn main(){
//     let s=String::from("Hello, world!");  
//     let slice=first_word(&s);

//     println!("The first word is: {}, {}",s, slice);
// }


// fn first_word(s: &String)-> &str{
//         let bytes= s.as_bytes();
//         for(i, &items) in bytes.iter().enumerate(){
//                 if items == b' ' {
//                       return &s[0..i];
//                 }
//         }

//         &s[..]
// }

//----------------------------------------------- struct

// struct User{
//        username:String,
//        email:String,
//        sign_in_count: u64,
//        active: bool,
// }

// fn main (){
//        let user1= build_user(String::from("victorjames408@gmail.com"), String::from("jamesvictor408"));

//        let user2: User = User {
//               email: String::from("victorjames408@gmail.com"),
//               username: String::from("jamesvictor408"),
//               ..user1
//        };

//        println!("User email: {}, username: {}, sign_in_count: {}, active: {}", 
//                 user1.email, user1.username, user1.sign_in_count, user1.active);
// }
 

// fn build_user(email:String, username:String)-> User{
//        User {
//               email,
//               username,
//               sign_in_count: 1,
//               active: true,
//         }
// }

// -------------------------------------------------tuple struct

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// struct Rectangle (
//     u32, // width
//     u32, // height
// );

// fn main(){
//    let params=Rectangle(30, 50);
   
//    println!("The area of the rectangle is {} square pixels.", area(params.0, params.1));

// }
// #[derive(Debug)]
// struct Rectangle{
//        width: u32,
//        height: u32,  
// }
// fn main(){
//        let rect= Rectangle {
//               width: 30,
//               height: 50,
//        };
//        println!("the reactangle, {:#?}", rect);
//        println!("The area of the rectangle is {} square pixels.", area(&rect));
// }

// fn area(rectangle: &Rectangle)-> u32{
//        rectangle.width * rectangle.height
// }

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main (){
//     let first_user=User{
//        username: String::from("jamesvictor408"),
//        email: String::from("victorjames408@gmail.com"),
//        sign_in_count:1,
//        active:true
//     };

//     let name=first_user.username;
//     let email=first_user.email;

//     println!("first user, {}", name);
//     println!("first user, {}", first_user.email);
// }


//-------------------------------- method
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
    
//     fn is_square(&self) -> bool {
//               self.width == self.height
//     }
    
//     fn set_width(&mut self, new_width: u32) {
//         self.width = new_width;
//     }
    
// }

// fn main(){
//        let mut rect=Rectangle{
//               width: 30,
//               height: 50,
//        };

//        println!("The area of the rectangle is {} square pixels.", rect.area());
//        println!("Is the rectangle a square? {}", rect.is_square());
//        rect.set_width(40);
// }


// fn main(){
//     let s1 = String::from("hello");
//       let (s2, len)=calculate_length(s1);
//        println!("this is s1 {}, this is s2 {}",len, s2)
// }

// fn calculate_length(s: String)-> (String, usize){
//     let length=s.len();
//     (s, length)
// }


// fn main(){
//     let mut s1= String::from("Hi, James");
//     change(&mut s1);
// }

// fn calculate_length(s: &String)-> usize{
//     s.len()
// }


// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

//  struct Rectangle {
//     width: u32,
//     height: u32,
//  }

// struct User<'a> {
//     username: &'a str,
//     email: &'a str,
// }

// fn main(){
//    let name="james";
//    let email="victorjames408@gmail.com";
//    let user= User{
//          username: name,
//          email: email,
//    };

//    println!("User email: {}, username: {}", user.email, user.username);
// }
//  impl Rectangle {
//      fn area(&self) -> u32 {
//         self.width * self.height
//      }

//      fn can_hold(&self, other: &Rectangle){
//          self.width > other.width && self.height > other.height;
//      }
//  }




//  fn main(){
//     let rect1= Rectangle{ width:30, height:40};
//     let rect2 = Rectangle { width: 10, height: 40 };
//     println!("{}, {:?}", rect1.area(),rect1.can_hold(&rect2))
//  }


#[derive(Debug)]
 struct Rectangle {
    width: u32,
    height: u32,
 }

 impl Rectangle{
    fn area(&self) -> u32{
    self.width * self.height
}
    fn width(&self)-> bool{
      self.width > 0
    }
 }
fn main(){
   let area1= Rectangle{
      width:50,
      height:20
   };
     
   println!("rect1 is {}", area1.area());

   if area1.width(){
      println!("this is the width, {}", area1.width())
   }
}

