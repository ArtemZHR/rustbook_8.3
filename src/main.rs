use std::collections::HashMap;
use std::io;
use std::io::BufRead;

enum Command {
    Add { department: String, name: String },
    List(String),
    All,
    Quit,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();

        match words.as_slice() {
            ["All"] => Some(Command::All),
            ["Quit"] => Some(Command::Quit),
            ["List", department] => Some(Command::List(department.to_string())),
            ["Add", name, "to", department] => Some(Command::Add {
                name: name.to_string(),
                department: department.to_string(),
            }),
            _ => None
        }
    }
}

fn main() {
    let mut emloyees: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = io::stdin();

    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");

    for line in stdin.lock().lines() {
        let input = line.expect("[Error]: unable to read user input");

        match Command::from_input(&input) {
            None => println!("[Error]: input error"),

            Some(Command::Add { name, department }) => {
                emloyees.entry(department).or_default().push(name)
            }

            Some(Command::List(department)) => match emloyees.get(&department) {
                Some(names) => {
                    for name in names {
                        println!("{}: {}", department, name);
                    }
                }
                None => println!("I don't recognize that department!"),
            },

            Some(Command::All) => {
                for (department, names) in &emloyees {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{}: {}", department, name);
                    }
                }
            }

            Some(Command::Quit) => break,
        }
    }
    println!("Have a nice day!");
}
