mod libs;
use libs;

mod modules;
use modules::base::models::User;

fn main() {
    let user = User::new(1, String::from("juice"), String::from("juice@test.com"), String::from("password"));
    println!("{:#?}", user);
}
