use std::fs;
use clap::Parser;
use clap::{arg};
use crate::graph::{self, ArgumentationFramework};
use std::{process::exit};
use std::time::{Duration, Instant};

pub enum Format {
    APX,
    CNF
}

pub fn get_input(file_path : &str, format : Format) -> ArgumentationFramework {
    match format {
        Format::APX => readingAPX(file_path),
        Format::CNF => readingCNF(file_path),
    }
}

fn readingAPX( file_path : &str) -> ArgumentationFramework {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let a = contents.trim().split('\n');
    let b : Vec<&str> = a.collect();
    let mut info = b[0].split_ascii_whitespace();
    let p = info.next().unwrap();
    let af = info.next().unwrap();
    let n = info.next().unwrap().parse::<usize>().unwrap();
    println!("{}", n);
    let af = ArgumentationFramework::new(n);
    af
}
fn readingCNF( file_path : &str) -> ArgumentationFramework {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let a = contents.trim().split('\n');
    let first_line = a.next().unwrap();
    let iter: Vec<&str> = first_line.split_ascii_whitespace().collect();
    let nb_arg = iter[2].parse::<i32>().unwrap();
    


    for line in a {

    }
    let af = ArgumentationFramework::new(n);
    af
}

fn print_supported_problems() {
    println!("[DC-CO,DC-ST,DC-SST,DC-STG,DC-ID,DS-PR,DS-ST,DS-SST,DS-STG]");
}
pub fn launcher() {
    let cli = Cli::parse();
    
    if cli.problems { // Print support problem if --problems
        print_supported_problems();
        exit(0);
    }
    if let Some(x) = cli.task {
        if x.contains('-') {
            let mut r = x.split('-');
            let problem = r.next().unwrap();
            let semantics = r.next().unwrap();
            println!("{} {}", problem, semantics);
            if problem != "DC" && problem != "DS" {
                eprintln!("This software only supports problems DC and DS.");
                exit(1);
            }
        }
    }
    
    let start = Instant::now();
    let af = get_input("test.txt", Format::APX);
    println!("{};",start.elapsed().as_millis() as f32 / 1000.0);

}




#[derive(Parser, Debug)]
#[command(author="Paul Cibier", version, about="This tool can solve all the problems in the approximate track of ICCMA 2023",
 long_about = None)]
 struct Cli {
    #[arg(short, long)]
    /// Quary argument for credulous and skeptical acceptance
    argument : Option<String>,
    #[arg(short='f', long="input_AF")]
    /// Path of the file containing the AF.
    input_af : Option<String>,
    #[arg(short = 'p', long="task")]
    /// A computational problem supported by the solver (e.g. DC-CO, DS-PR).
    task : Option<String>,
    #[arg( long)]
    /// Prints the supported computational problems and exits
    problems : bool
}