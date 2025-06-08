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

fn main(){
    let s=String::from("Hello, world!");  
    let slice=first_word(&s);

    println!("The first word is: {}, {}",s, slice);
}


fn first_word(s: &String)-> &str{
        let bytes= s.as_bytes();
        for(i, &items) in bytes.iter().enumerate(){
                if items == b' ' {
                      return &s[0..i];
                }
        }

        &s[..]
}