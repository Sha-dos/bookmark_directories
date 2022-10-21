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
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 { eprintln!("Error: Expected 1 argument"); }
    else {
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

            // println!("Dirs: {:?}", saved_dirs[0]);

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

            let data = format!("{},{}", &args[1], env::current_dir().unwrap().to_str().unwrap());

            file.write_all(data.as_bytes());
        }
    }
}

fn parse(data: &mut String) -> SavedDir {
    let split: Vec<&str> = data.split(",").collect();

    let name = split[0].to_string();
    let dir = split[1].to_string();

    SavedDir {
        name,
        dir,
    }
}

fn goto_dir(path: String) {
    println!("{}", &path);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
