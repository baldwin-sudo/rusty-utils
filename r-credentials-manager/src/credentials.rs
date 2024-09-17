use std::io;

pub struct Credential {
    pub id: String,
    pub username: String,
    pub usage_desc: String,
    pub password: String,
}
impl Credential {
    pub fn create_credential() -> Result<Self, io::Error> {
        let usage_desc = loop {
            println!("1-Enter credential usage (or type 'quit' to exit):");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_string();
            if input.to_lowercase() == "quit" {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "User quit the input process",
                ));
            }
            if !input.is_empty() {
                break input;
            }
            println!("!! usage description cannot be empty !!");
        };

        let username = loop {
            println!("2-Enter credential username (or type 'quit' to exit):");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_string();
            if input.to_lowercase() == "quit" {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "User quit the input process",
                ));
            }
            if !input.is_empty() {
                break input;
            }
            println!("!! username cannot be empty !!");
        };

        let password = loop {
            println!("3-Enter the password to store (or type 'quit' to exit):");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_string();
            if input.to_lowercase() == "quit" {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "User quit the input process",
                ));
            } else {
                break input;
            }
        };

        Ok(Credential {
            id: "_".to_string(),
            usage_desc,
            username,
            password,
        })
    }
}
