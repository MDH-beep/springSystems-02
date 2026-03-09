use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            let status = Command::new("ls")
                .arg(path)
                .status()
                .expect("Failed to execute ls");

            if !status.success() {
                println!("Error listing directory");
            }
        }

        FileOperation::Display(file) => {
            let status = Command::new("cat")
                .arg(file)
                .status()
                .expect("Failed to execute cat");

            if !status.success() {
                println!("Error displaying file");
            }
        }

        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);

            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("Failed to create file");

            if output.status.success() {
                println!("File '{}' created successfully.", path);
            } else {
                println!("Failed to create file.");
            }
        }

        FileOperation::Remove(path) => {
            let status = Command::new("rm")
                .arg(&path)
                .status()
                .expect("Failed to remove file");

            if status.success() {
                println!("File '{}' removed successfully.", path);
            } else {
                println!("Failed to remove file.");
            }
        }

        FileOperation::Pwd => {
            let status = Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");

            if !status.success() {
                println!("Error printing working directory");
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let choice = get_input("Enter your choice (0-5): ");

        match choice.as_str() {
            "1" => {
                let path = get_input("Enter directory path: ");
                perform_operation(FileOperation::List(path));
            }

            "2" => {
                let file = get_input("Enter file path: ");
                perform_operation(FileOperation::Display(file));
            }

            "3" => {
                let path = get_input("Enter file path: ");
                let content = get_input("Enter content: ");
                perform_operation(FileOperation::Create(path, content));
            }

            "4" => {
                let path = get_input("Enter file path: ");
                perform_operation(FileOperation::Remove(path));
            }

            "5" => {
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Please enter a number between 0 and 5.");
            }
        }
    }
}