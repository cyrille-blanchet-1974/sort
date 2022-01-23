use std::env;
use std::fs::File;

#[derive(Debug)]
pub struct Paramcli
//parameters from command line and/or confFile
{
    pub fic: String, //file to work with if specified
    pub reverse: bool,
}

impl Default for Paramcli {
    fn default() -> Self {
        Paramcli::new()
    }
}

impl Paramcli {
    pub fn new() -> Paramcli {
        let mut fic = String::new();
        let mut reverse = false;

        let args: Vec<String> = env::args().skip(1).collect();
        let name = env::args()
            .take(1)
            .next()
            .unwrap_or_else(|| String::from("sort"));
        if args.len() > 2 {
            println!("Error: toot much parameters");
            help(&name);
        }
        for arg in args {
            if arg == "/?"
                || arg == "-?"
                || arg.to_lowercase() == "/help"
                || arg.to_lowercase() == "-help"
            {
                help(&name);
            }
            if arg == "-r" {
                reverse = true;
                continue;
            }
            fic = arg;
        }
        //checks
        if !fic.is_empty() {
            //check if file exists
            if File::open(&fic).is_err() {
                println!("Error file {} doesn't exists or unereadable", &fic);
                help(&name);
            };
        }
        Paramcli { fic, reverse }
    }
}

fn help(name: &str) {
    println!("{} 1.0 (2022)", name);
    println!("syntax : {} [file] [-r]   ", name);
    println!("parameters between [] are optionnals");
    println!("------------------------------------");
    println!("fic: file to work with (if non use stdin");
    println!("-r : reverse");
    std::process::exit(0);
}
