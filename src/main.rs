use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, Write},
    path::Path,
    str::FromStr,
};

const MENU: &'static str = "
Choose one of the following options:
1) Open or create the file
2) Delete the file
3) Append a line to the file
4) Truncate the file
5) Print file content
6) Quit\
";

enum Choice {
    Open,
    Delete,
    Append,
    Truncate,
    Print,
    Quit,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choice = match s.trim().to_lowercase().as_str() {
            "1" | "o" | "open" | "c" | "create" => Choice::Open,
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

fn prompt_user(prompt: &str) -> Result<String, std::io::Error> {
    println!("{prompt}");
    print!("> ");

    std::io::stdout().flush()?;

    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;
    Ok(name)
}

fn open_file<P: AsRef<Path>>(path: P) -> Option<File> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(&path);

    if let Err(e) = file {
        println!("Could not open/create file: {}.", e.kind());
        return None;
    }

    println!("File has been opened.");
    file.ok()
}

fn main() -> Result<(), std::io::Error> {
    let mut file: Option<File> = None;
    let path = "./file.txt";

    loop {
        let input = prompt_user(MENU)?;

        let Ok(choice) = Choice::from_str(&input) else {
            println!("Unrecognized option, try again.");
            continue;
        };

        match choice {
            Choice::Open => {
                file = open_file(path);
            }
            Choice::Delete => {
                file = None;
                let res = std::fs::remove_file(&path);

                match res {
                    Ok(_) => println!("File has been deleted."),
                    Err(e) => println!("Could not delete file: {}.", e.kind()),
                }
            }
            Choice::Append => {
                let Some(file) = file.as_mut() else {
                    println!("File has not been created yet.");
                    continue;
                };

                let input = prompt_user(
                    "Write a line that will be appended to the file:",
                )?;

                if let Err(e) = write!(file, "{input}") {
                    println!("Could not append to file: {}.", e.kind());
                }
            }
            Choice::Truncate => {
                let Some(file) = file.as_mut() else {
                    println!("File has not been created yet.");
                    continue;
                };

                if let Err(e) = file.set_len(0) {
                    println!("Could not truncate file: {}.", e.kind());
                }
            }
            Choice::Print => {
                let Some(file) = file.as_mut() else {
                    println!("File has not been created yet.");
                    continue;
                };

                let mut s = String::new();

                file.flush()?;
                file.seek(std::io::SeekFrom::Start(0))?;

                match file.read_to_string(&mut s) {
                    Ok(_) => println!("{s}"),
                    Err(e) => {
                        println!("Could not read from file: {}.", e.kind())
                    }
                }
            }
            Choice::Quit => break,
        }
    }

    Ok(())
}
