use std::io::Write;
use std::io::stdout;
use std::process::exit;
use std::time::Instant;
use clap::Parser;
use clap::arg;
use crate::extensionsemantics::CategorizedBasedApproximateSolver;
use crate::parser;

pub enum Format {
    APX,
    CNF
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Problem {
    DS, DC, SE
}
#[derive(Debug, Clone, Copy)]
pub enum Semantics {
    CO,ST,SST,STG,ID,PR
}
#[derive(Debug, Clone)]
pub struct Task {
    pub problem : Problem,
    pub semantics : Semantics,
    pub argument : usize,
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


pub fn launcher() {
    let cli = Cli::parse();

    if cli.problems { // Print support problem if --problems
        print_supported_problems();
        exit(0);
    }
    let arg_name = cli.argument.clone();
    let argument_name = match arg_name {
        Some(arg) => { arg.parse::<usize>().unwrap()-1 },
        None => {
            eprintln!("Expected an argument with -a");
            exit(1);
        }
    };
    let pr_sm = cli.task.clone();
    let (problem, semantics) = match pr_sm {
        Some(t) => {
            if !t.contains('-') {
                eprintln!("Error parsing command-line arguments\n");
                exit(1);
            }
            let mut r = t.split('-');
            let problem = String::from(r.next().unwrap());
            let problem = match problem.as_str() {
                "DC" => Problem::DC,
                "DS" => Problem::DS,
                "SE" => Problem::SE,
                _ => { eprintln!("This problem is not handled by the program at this time"); exit(1);}
            };
            let semantics = String::from(r.next().unwrap());
            let semantics = match semantics.as_str() {
                "ST" => Semantics::ST,
                "SST" => Semantics::SST,
                "STG" => Semantics::STG,
                "ID" => Semantics::ID,
                "PR" => Semantics::PR,
                "CO" => Semantics::CO,
                _ => { eprintln!("This problem is not handled by the program at this time"); exit(1);}
            };
            (problem, semantics)
        },
        None => {
            eprintln!("expected a problem and a semantic");
            exit(1) 
        }
    };
    let file = cli.input_af.clone().unwrap();
    let file_path = file.as_str();
    let start = Instant::now();
    let af = parser::get_input(file_path, Format::CNF);
    print!("{};",start.elapsed().as_millis() as f32 / 1000.0);
    stdout().flush();
    let task = Task { problem, semantics, argument : argument_name  };
    
    print!("{}", CategorizedBasedApproximateSolver::solve(af, task));


}

fn print_supported_problems() {
    println!("[DC-CO,DC-ST,DC-SST,DC-STG,DC-ID,DS-PR,DS-ST,DS-SST,DS-STG]");
}