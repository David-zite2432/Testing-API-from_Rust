use std::io::stdin;

fn main() {
    // This project will simulate recolate collect data form users API
    let mut _user_id = String::new();
    println!("Into the user id");
    stdin().read_line(&mut _user_id).expect("Can´t read the line");
}
