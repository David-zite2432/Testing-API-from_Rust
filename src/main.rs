use std::io::stdin;
use serde::{ Serialize, Deserialize };

fn main() {
    // This project will simulate recolate collect data form users API
    let mut user_id = String::new();
    println!("Into the user id");
    stdin().read_line(&mut user_id).expect("Can´t read the line");

    if let Ok(value) = return_user(&user_id) {
        println!("{}", value);
    }
    println!("Comapnie Data");

}

fn return_user(user_id: &str) -> Result<String, ureq::Error> {
    let body = ureq::get(&format!(
        "https://jsonplaceholder.typicode.com/users/{}",
        user_id
    )).call()?.into_string()?;

    let user: UserData = serde_json::from_str(&body).unwrap();
    let general_info = format!(
        "Name: {}, User name: {}, email: {} website: {}",
        user.name, user.username, user.email, user.website
    );

    Ok(general_info)
}

#[derive(Serialize, Deserialize)]
struct UserData {
    name: String,
    username: String,
    email: String,
    website: String,
}
