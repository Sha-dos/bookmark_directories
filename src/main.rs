use std::process::exit;
use std::{env, fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::io::Write;

#[derive(Debug)]
struct SavedDir {
    name: String,
    dir: String,
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    match &args[1].as_str() {
        &"init" => {
            if args.len() != 2 { eprintln!("Error: Expected 1 argument!"); }

            let shell_path = format!("{}{}", env::home_dir().unwrap().to_str().unwrap(), "/cd.sh");

            if !Path::new(&shell_path).exists() {
                File::create(&shell_path);
            }

            let mut file = OpenOptions::new()
            .write(true)
            .open(&shell_path)
            .unwrap();

            let data = String::from("#!/bin/bash
            output=$(bookmark_directories $1 $2)
            if [[ $output == cd* ]]
            then
                cd ~/
                eval $output
            else
                echo $output
            fi"
            );

            file.write(data.as_bytes());

            let zshrc_path = format!("{}{}", env::home_dir().unwrap().to_str().unwrap(), "/.zshrc");

            let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(zshrc_path)
            .unwrap();

            let mut contents = String::new();

            file.read_to_string(&mut contents);

            if !contents.contains(r#"alias mrk=". ~/cd.sh""#) {
                file.write(br#"alias mrk=". ~/cd.sh""#);
            }
        }

        &"l" => {
            if args.len() != 2 { eprintln!("Error: Expected 1 argument!"); }

            let path = format!("{}{}", env::home_dir().unwrap().to_str().unwrap(), "/.bookmarked_dirs");

            let mut saved: Vec<SavedDir> = Vec::new();

            let lines = read_lines(path)
                .expect("Error: Failed to open file. Have you set up your shell? {bookmark_directories init}");

            for line in lines {
                saved.push(parse(&mut line.unwrap()));
            }

            for saved_dir in saved {
                println!("Name: {} Directory: {}", saved_dir.name, saved_dir.dir);
            }
        }

        &"r" => {
            if args.len() != 3 { eprintln!("Error: Expected 2 argument!s"); }

            let path = format!("{}{}", env::home_dir().unwrap().to_str().unwrap(), "/.bookmarked_dirs");

            let mut contents = String::new();

            let mut file = OpenOptions::new()
                .write(true)
                .read(true)
                .append(false)
                .open(&path)
                .expect("Error: Failed to open file!");

            file.read_to_string(&mut contents);

            let lines = read_lines(&path);

            let mut directory = String::new();

            for line in lines.unwrap() {
                if line.as_ref().unwrap().contains(&args[2]) {
                    let seperate: Vec<&str> = line.as_ref().unwrap().split(",").collect();
                    directory = seperate[1].to_string();
                }
            }

            let name = format!("{}{}", &args[2], ",");

            contents = contents.replace(&name, "");
            contents = contents.replace(&directory, "");

            std::fs::remove_file(&path);

            let mut file = OpenOptions::new()
                .write(true)
                .read(true)
                .append(false)
                .create(true)
                .open(&path)
                .expect("Error: Failed to create file!");

            file.write_all(&contents.trim().as_bytes());
        }

        _ => {
            let saved_path = format!("{}{}", env::home_dir().unwrap().to_str().unwrap(), "/.bookmarked_dirs");
            // println!("{}", &saved_path);

            if !Path::new(&saved_path).exists() {
                File::create(&saved_path);
            }

            let lines = read_lines(PathBuf::from(&saved_path)).unwrap();

            if fs::read_to_string(&saved_path).unwrap().contains(&args[1]) {
                let mut saved_dirs: Vec<SavedDir> = Vec::new();

                for line in lines {
                    saved_dirs.push(parse(&mut line.unwrap()));
                }

                for saved_dir in saved_dirs {
                    if saved_dir.name == args[1] {
                        goto_dir(saved_dir.dir);
                    }
                }
            } else {
                let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&saved_path)
                .unwrap();

                let mut data = format!("{},{}", &args[1], env::current_dir().unwrap().into_os_string().into_string().unwrap());

                writeln!(file, "{}", &data);
            }
        }
    }
}

fn parse(data: &mut String) -> SavedDir {
    let split: Vec<&str> = data.split(",").collect();

    let name = &split[0].to_string();
    let dir = &split[1].to_string();

    SavedDir {
        name: name.to_string(),
        dir: dir.to_string(),
    }
}

fn goto_dir(path: String) {
    println!("cd {}", &path);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
