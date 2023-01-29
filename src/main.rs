use std::fs;
use std::io;
//use std::error::Error;
use std::process;
use regex::Regex;

fn main() {
    println!("Enter File Path");
    let mut file_path: String = String::new();
    
    io::stdin().read_line(&mut file_path).expect("An Error Has Occured.");

    let file_path: &str = file_path.trim();
    println!("{file_path}");

    let data: String = get_file_data(file_path).unwrap_or_else(|err: &str| {
        println!("Problem parsing argumants: {err}");
        process::exit(1);
    });

    let config: Config = Config::new(data).unwrap_or_else(|err| {
        println!("Problem parsing argumants: {err}");
        process::exit(1);
    });

    let re = Regex::new(r"(?P<info>[A-Z]),\s(?P<date>\d{2}-\d{2}-\d{4})").unwrap();

    for d in config.data {  
        let caps = re.captures(&d).unwrap();
        println!("{:?}", caps.get(1).unwrap().as_str());
    }

}

fn get_file_data(file_path: &str) -> Result<String, &'static str> {
    if file_path.len() < 1 {
        return Err("No Data");
    }
    let contents: String = fs::read_to_string(file_path).expect("ERRORS");
    
    Ok(contents)
}

struct Config {
    data: Vec<String>
}

impl Config {
    fn new(args: String) -> Result<Config, &'static str> {
        if args.len() == 0 {
            return Err("No Data From fs args");
        }
        let data: String = args;

        let seperate = data.split("\r\n");

        let mut vec: Vec<String> = vec![];
        for st in seperate {
            let s: String = String::from(st);
            vec.push(s);
        }


        Ok(Config {data: vec})
    }
}