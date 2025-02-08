use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

const MENU: &'static str = "
Choose one of the following options:
1) Create the file
2) Delete the file
3) Append a line to the file
4) Truncate the file
5) Print file content
6) Quit\
";

enum Choice {
    Create,
    Delete,
    Append,
    Truncate,
    Print,
    Quit,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choice = match s.trim().to_lowercase().as_str()
        {
            "1" | "c" | "create" => Choice::Create,
            "2" | "d" | "delete" => Choice::Delete,
            "3" | "a" | "append" => Choice::Append,
            "4" | "t" | "truncate" => Choice::Truncate,
            "5" | "p" | "print" => Choice::Print,
            "6" | "q" | "quit" => Choice::Quit,
            _ => return Err(()),
        };

        Ok(choice)
    }
}

fn prompt_user(
    prompt: &str,
) -> Result<String, std::io::Error> {
    println!("{prompt}");
    print!("> ");

    std::io::stdout().flush()?;

    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;
    Ok(name)
}

fn create_file<P: AsRef<Path>>(path: P) {
    if let Ok(true) = std::fs::exists(&path) {
        println!("File already exists.");
        return;
    }

    let file = OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(&path);

    match file {
        Ok(_) => println!("File has been created."),
        Err(e) => println!("Could not create file: {e}"),
    }
}

fn delete_file<P: AsRef<Path>>(path: P) {
    let res = std::fs::remove_file(&path);

    match res {
        Ok(_) => println!("File has been deleted."),
        Err(_) => println!("Could not delete file."),
    }
}

fn append_to_file<P: AsRef<Path>>(path: P, prompt: &str) {
    let Ok(mut file) =
        OpenOptions::new().append(true).open(&path)
    else {
        println!("Could not open file.");
        return;
    };

    let Ok(line) = prompt_user(prompt) else {
        println!("Prompt failed.");
        return;
    };

    if let Err(_) = write!(&mut file, "{line}") {
        println!("Could not append to file.");
    }
}

fn truncate_file<P: AsRef<Path>>(path: P) {
    if OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .is_err()
    {
        println!("Could not truncate the file.");
        return;
    };

    println!("File has been truncated.");
}

fn print_file<P: AsRef<Path>>(path: P) {
    let Ok(mut file) =
        OpenOptions::new().read(true).open(&path)
    else {
        println!("Could not open file.");
        return;
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Ok(_) => println!("{s}"),
        Err(_) => println!("Could not read from file."),
    }
}

fn main() -> Result<(), std::io::Error> {
    loop {
        let input = prompt_user(MENU)?;
        let Ok(choice) = Choice::from_str(&input) else {
            println!("Unrecognized option, try again.");
            continue;
        };

        let path = "./file.txt";

        match choice {
            Choice::Create => {
                create_file(path);
            }
            Choice::Delete => delete_file(path),
            Choice::Append => {
                append_to_file(path, "Write a line that will be appended to the file:");
            }
            Choice::Truncate => {
                truncate_file(path);
            }
            Choice::Print => {
                print_file(path);
            }
            Choice::Quit => break,
        }
    }

    Ok(())
}
