use std::fmt;
use std::io;

pub enum Priority {
    High,
    Medium,
    Low,
}
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let priority_str = match self {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        };
        write!(f, "{}", priority_str)
    }
}

pub struct Task {
    pub name: String,
    pub body: String,
    pub priority: Priority,
}
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Task Name: {}\nTask Body: {}\nPriority: {}",
            self.name, self.body, self.priority
        )
    }
}

impl Task {
    pub fn create_task() -> Result<Self, io::Error> {
        let name = loop {
            println!("1-Enter Task name (or type 'quit' to exit):");
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
            println!("!! Task name cannot be empty !!");
        };

        let body = loop {
            println!("2-Enter Task body (or type 'quit' to exit):");
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
            println!("!! Task body cannot be empty !!");
        };

        let priority = loop {
            println!(
                "3-Enter Task priority level (High=1/Medium=2/Low=3) (or type 'quit' to exit):"
            );
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_string();
            if input.to_lowercase() == "quit" {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "User quit the input process",
                ));
            }
            match input.parse::<u8>() {
                Ok(1) => break Priority::High,
                Ok(2) => break Priority::Medium,
                Ok(3) => break Priority::Low,
                _ => println!("!! Please enter a valid priority level (1, 2, or 3) !!"),
            }
        };

        Ok(Task {
            name,
            body,
            priority,
        })
    }
}
