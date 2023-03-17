use std::io::stdin;
use serde_json::Value;

fn main() {
    // This project will simulate recolate collect data form users API
    let mut user_id = String::new();
    println!("Into the user id");
    stdin().read_line(&mut user_id).expect("Can´t read the line");

    if let Ok(value) = return_user(&user_id) {
        println!("{}", value);
    }

}

fn return_user(user_id: &str) -> Result<String, ureq::Error> {
    let body = ureq::get(&format!(
        "https://jsonplaceholder.typicode.com/users/{}",
        user_id
    )).call()?.into_string()?;

    let user: Value = serde_json::from_str(&body).unwrap();
    Ok(user.to_string())
}
