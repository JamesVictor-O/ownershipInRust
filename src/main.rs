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

// borrowing
//  fn main() {
//     let s = String::from("Hello, world!");
//     let len = calculate_length(&s); 
//     println!("The length of '{}' is {}.", s, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// slice

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

// struct

struct User{
       username:String,
       email:String,
       sign_in_count: u64,
       active: bool,
}

fn main (){
       let user1= build_user(String::from("victorjames408@gmail.com"), String::from("jamesvictor408"));

       let user2: User = User {
              email: String::from("victorjames408@gmail.com"),
              username: String::from("jamesvictor408"),
              ..user1
       };

       println!("User email: {}, username: {}, sign_in_count: {}, active: {}", 
                user1.email, user1.username, user1.sign_in_count, user1.active);
}
 

fn build_user(email:String, username:String)-> User{
       User {
              email,
              username,
              sign_in_count: 1,
              active: true,
        }
}