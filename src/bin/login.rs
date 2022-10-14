/*Login and Registration System.
This is one of the simplest projects to start with to learn about file systems in rust.
The project involves a user registration process by asking username and password.
Upon successful registration, a user file is created with the credentials.
If the user does not exist, upon login, an error will be shown.
You will also learn how to use Visual Studio to create a simple project.*/

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};

#[derive(Debug, PartialEq)]
struct User {
    username: String,
    password: String,
}

impl User {
    ///Creates a new user
    fn new(username: &str, password: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
        }
    }
    /// **Returns** a Result. If the parsing is successfull then a `User struct` is returned
    /// in case there is a parsing error then a error string is returned.
    ///
    /// # Arguments
    ///
    /// * `line` - It should be a valid formated string.
    ///
    /// ## Example of line format
    /// `"User(This is a username);Password(This is a password)"`
    fn try_parse(line: &str) -> Result<Self, String> {
        let fields = line.split(";").collect::<Vec<&str>>();
        let username: Option<String> = match fields[0].strip_prefix("User(") {
            Some(user) => match user.strip_suffix(")") {
                Some(user) => Some(user.to_string()),
                None => None,
            },
            None => None,
        };
        let password: Option<String> = match fields[1].strip_prefix("Password(") {
            Some(pass) => match pass.strip_suffix(")") {
                Some(password) => Some(password.to_string()),
                None => None,
            },
            None => None,
        };

        match (username, password) {
            (Some(username), Some(password)) => Ok(Self { username, password }),
            (_, _) => Err("Parsing error for: {line}".to_owned()),
        }
    }

    fn save_to_file(&self, file_path: &str) -> Result<(), String> {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path);
        match f {
            Ok(mut file) => {
                match file.write(
                    &format!("User({});Password({})\n", self.username, self.password)
                        .bytes()
                        .collect::<Vec<u8>>(),
                ) {
                    Ok(_) => return Ok(()),
                    Err(_) => return Err("Error writing to file".to_owned()),
                }
            }
            Err(e) => {
                return Err(format!(
                    "Error in opening the file. Please check that file path is correct.\n {e:?}"
                )
                .to_owned())
            }
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => Some(buffer.trim().to_owned()),
        Err(_) => None,
    }
}

#[derive(Debug)]
enum LoginMenu {
    SignUp,
    Login,
    Quit,
}

impl LoginMenu {
    fn show_menu() {
        println!("Choose between this options: ");

        println!("1) SignUp");

        println!("2) Login");

        println!("3) Quit");
    }
    fn get_option() -> LoginMenu {
        loop {
            print!("\n>");
            std::io::stdout().flush();
            match get_input() {
                Some(ch) => match ch.as_str() {
                    "1" => return LoginMenu::SignUp,
                    "2" => return LoginMenu::Login,
                    "3" => return LoginMenu::Quit,
                    _ => {
                        println!("Wrong choice. Please select from the options above");
                    }
                },
                None => {
                    println!("Error in reading from terminal please, check.");
                }
            }
        }
    }
}

fn read_username() -> String {
    print!("Insert username: ");
    io::stdout().flush();
    get_input().expect("Error reading username.")
}

fn read_password() -> String {
    print!("Insert password: ");
    io::stdout().flush();
    get_input().expect("Error reading password")
}

fn check_username(username: &str, file: &File) -> Option<User> {
    match io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .find(|line| line.contains(format!("User({username})").as_str()))
    {
        Some(line) => match User::try_parse(line.trim()) {
            Ok(user) => return Some(user),
            Err(e) => {
                println!("{e:?}");
                return None;
            }
        },
        None => None,
    }
}

fn init() -> bool {
    let file_path = "/home/elohim/Documents/rust/rust_training/users.db";
    //Print menu choices
    LoginMenu::show_menu();
    match LoginMenu::get_option() {
        LoginMenu::Quit => return false,
        LoginMenu::Login => {
            let username = read_username();
            let f = File::open(file_path).expect("Error in openning the database.");
            match check_username(&username, &f) {
                Some(user) => {
                    let password = read_password();
                    if user.password == password {
                        println!("{user:?}");
                    } else {
                        println!("Wrong password.");
                    }
                }
                None => println!("No username found."),
            }
        }
        LoginMenu::SignUp => {
            let mut username = String::new();
            loop {
                username = read_username();
                if !username.contains(";") {
                    break;
                }
                println!("Username must not contain ';' character.");
            }
            let password = read_password();
            let user = User::new(&username, &password);
            if let Err(e) = user.save_to_file(file_path) {
                println!("{e}");
            }
            println!("User credentials saved to database.");
        }
    }
    true
}

fn main() {
    loop {
        if !init() {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_parse() {
        let s = "User(this is a username);Password(This is a password)";
        let user: User = User::new("this is a username", "This is a password");
        assert_eq!(Ok(user), User::try_parse(s));
    }
}
