use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn init(name: String) -> std::io::Result<()> {
    match fs::create_dir(name.clone()) {
        Ok(()) => {
            let gitpath = name.as_str().to_owned() + "/.git";
            match fs::create_dir(gitpath.clone()) {
                Ok(()) => {
                    let hookpath = gitpath.as_str().to_owned() + "/hooks";
                    let infopath = gitpath.as_str().to_owned() + "/info";
                    let objectspath = gitpath.as_str().to_owned() + "/objects";
                    let refspath = gitpath.as_str().to_owned() + "/refs";

                    match fs::create_dir(hookpath) {
                        Ok(()) => println!("Created Hooks"),
                        Err(err) => println!("Failed to create directory: {}", err),
                    }

                    match fs::create_dir(infopath) {
                        Ok(()) => println!("Created info"),
                        Err(err) => println!("Failed to create directory: {}", err),
                    }

                    match fs::create_dir(objectspath.clone()) {
                        Ok(()) => {
                            let packpath = objectspath.to_owned() + "/pack";
                            let infopath2 = objectspath.to_owned() + "/info";

                            match fs::create_dir(packpath) {
                                Ok(()) => println!("Created pack"),
                                Err(err) => println!("Failed to create directory: {}", err),
                            }

                            match fs::create_dir(infopath2) {
                                Ok(()) => println!("Created info:"),
                                Err(err) => println!("Failed to create directory: {}", err),
                            }
                        },
                        Err(err) => println!("Failed to create directory: {}", err),
                    }

                    match fs::create_dir(refspath.clone()) {
                        Ok(()) => {
                            let tagspath = refspath.to_owned() + "/tags";
                            let headspath = refspath.to_owned() + "/heads";

                            match fs::create_dir(tagspath) {
                                Ok(()) => println!("Created tags"),
                                Err(err) => println!("Failed to create directory: {}", err),
                            }

                            match fs::create_dir(headspath) {
                                Ok(()) => println!("Created heads"),
                                Err(err) => println!("Failed to create directory: {}", err),
                            }
                        },
                        Err(err) => println!("Failed to create directory: {}", err),
                    }

                    let configpath = gitpath.as_str().to_owned() + "/config";
                    let descpath = gitpath.as_str().to_owned() + "/description";
                    let mut configfile = File::create(configpath)?;
                    let mut descfile = File::create(descpath)?;

                    writeln!(configfile, "[core]")?;
                    writeln!(configfile, "	repositoryformatversion = 0")?;
                    writeln!(configfile, "	filemode = false")?;
                    writeln!(configfile, "	bare = false")?;
                    writeln!(configfile, "	logallrefupdates = true")?;
                    writeln!(configfile, "  symlinks = false")?;
                    writeln!(configfile, "	ignorecase = true")?;

                    configfile.flush()?;

                    writeln!(descfile, "Unnamed repository; edit this file 'description' to name the repository. ")?;

                    descfile.flush()?;
                },
                Err(err) => println!("Failed to create directory: {}", err),
            }

        },
        Err(err) => println!("Failed to create directory: {}", err),
    }
    Ok(())
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
