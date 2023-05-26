use std::fs;
use std::env;

fn init(name: String) {
    match fs::create_dir(name.clone()) {
        Ok(()) => {
            let gitpath = name.as_str().to_owned() + "/.git";
            match fs::create_dir(gitpath.clone()) {
                Ok(()) => {
                    let hookpath = gitpath.as_str().to_owned() + "/hooks";
                    match fs::create_dir(hookpath) {
                        Ok(()) => println!("Created Hooks"),
                        Err(err) => println!("Failed to create directory: {}", err),
                    }
                },
                Err(err) => println!("Failed to create directory: {}", err),
            }
        },
        Err(err) => println!("Failed to create directory: {}", err),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let argument = &args[1].to_string();
        let name = args.get(2).map(String::clone).unwrap_or_default();

        match argument.as_str() {
            "init" => {
                init(name);
            },
            _ => {
                println!("Unknown");
            }
        }
    }
}
