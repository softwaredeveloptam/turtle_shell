use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::fs;

fn main() {

    println!("Welcome to Turtle Shell");

    loop {
    
        current_path().expect("Current path cannot be read.");
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
    
        while let Some(command) = commands.next()  {

            // everything after the first whitespace character is interpreted as args to the command
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    // default to '/' as new directory if one was not provided
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    // previous_command = None;
                },
                "mkdir" => {
                    make_new_dir().unwrap();
                },
                "ls" => {
                    list_all_dir();
                },
                "exit" => return,
                "help" => {
                    println!("\nHere are a list of commands to help you get started.");
                    println!("----------------------------------------------------\n");
                    println!("cd <file path> - allows you to move between folders and change directories according to the file path.");
                    println!("cd .. - allows you to move up one directory");
                    println!("mkdir <new folder name> - allows you to create a new directory with the name inputted.");
                    println!("ls - lists all files and folders within a directory.");
                    println!("exit - exits out of turtle shell\n");
                },
                _ => {
                    println!("Invalid command. Type <help> for a list of commands.");
                    continue
                }
            }
        }
    }
}

fn current_path() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    Ok(())
}

fn make_new_dir() -> std::io::Result<()> {

    fs::create_dir_all("C:/Users/User/Dropbox/coding/Rust/turtle_shell/some/dir")?;
    Ok(())
}

fn list_all_dir() {
    let paths = fs::read_dir("./").unwrap();

    println!("\nHere are all the files and folders in this directory.");
    println!("-----------------------------------------------------");

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
    println!("");
}