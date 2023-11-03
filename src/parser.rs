use std::fs;
use clap::Parser;
use clap::arg;
use crate::graph::ArgumentationFramework;
use std::process::exit;
use std::time::Instant;

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

fn readingCNF( file_path : &str) -> ArgumentationFramework {
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let mut content_iter = contents.trim().split('\n');
    let first_line = content_iter.next().unwrap();
    let iter: Vec<&str> = first_line.split_ascii_whitespace().collect();
    let nb_arg = iter[2].parse::<usize>().unwrap();
    let mut af = ArgumentationFramework::new(nb_arg);

    for line in content_iter {
        if !line.starts_with('#') && (!line.trim().eq("")) {
            let (attacker,target) = parseCNFAttackLine(line);
            //println!("{} {}", attacker, target);
            af.add_attack(attacker, target);
        }
    }
    af
}
fn find_number_argument(file_path : &str) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let a = contents.trim().split('\n');
    let mut nb_arg = 0;
    for line in a {
        if line.starts_with("arg") { nb_arg +=1; }
        else { break; }
    }
    nb_arg
}

fn parseCNFAttackLine (line : &str) -> (i32,i32) {
    let mut a = line.split_ascii_whitespace();
    let att = a.next().unwrap().parse::<i32>().unwrap();
    let targ = a.next().unwrap().parse::<i32>().unwrap();
    (att,targ)
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
    let file = cli.input_af.clone().unwrap();
    let file_path = file.as_str();
    println!("{file_path}");
    let start = Instant::now();
    let af = get_input(file_path, Format::CNF);
    println!("{};",start.elapsed().as_millis() as f32 / 1000.0);
    println!("{}", af.af_attackee.len());
    
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
fn readingAPX( file_path : &str) -> ArgumentationFramework {
    
    let nb_arg = find_number_argument(file_path);
    let af = ArgumentationFramework::new(nb_arg as usize);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let a = contents.trim().split('\n');

    for line in a {
        if !line.starts_with("#") && (!line.trim().eq("")) {
            //af.add
        }
    }
    
    af
}