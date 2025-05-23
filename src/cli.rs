use std::io::Write;
use std::io::stdout;
use std::process::exit;
use std::time::Instant;
use clap::Parser;
use clap::arg;
use crate::extensionsemantics::CategorizedBasedApproximateSolver;
use crate::parser;

pub enum Format {
    Apx,
    Cnf
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Problem {
    DS, DC, SE
}
#[derive(Debug, Clone, Copy)]
pub enum Semantics {
    CO,ST,SST,STG,ID,PR
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Heuristic {
    #[default]
    V2,
    HARPER,
    HCAT,
    INOUT,
    NoSelfAtt,
    Card,
    Max,
    Counting,
    Perso,
}
#[derive(Debug, Clone)]
pub struct Task {
    pub problem : Problem,
    pub semantics : Semantics,
    pub argument : usize,
    pub algo : Heuristic,
    pub verbose : bool,
    pub old : bool,
    pub threshold : Option<f64>,
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
    #[arg(long="fo")]
    /// Format of the file containing the AF.
    input_format : Option<String>,
    #[arg(short = 'p', long="task")]
    /// A computational problem supported by the solver (e.g. DC-CO, DS-PR).
    task : Option<String>,
    #[arg(long)]
    /// Prints the supported computational problems and exits
    problems : bool,
    /// Avalaible options : harper, inout, hcat, noselfatt, card, maxb, counting
    #[arg(long)]
    heuristic : Option<String>,
    /// Print details of the execution time of each part of the solution
    /// "to parse the file ; to solve the grounded extention ; to solve with an heuristic ; the result "
    #[arg(short, long, verbatim_doc_comment)]
    verbose : bool,
    /// Choose which algo is used for the grounded part, if set then use the new one
    #[arg(short, long)]
    old : bool,
    ///Choose the value of the threshold for the graduated semantic
    #[arg(short, long)]
    thresold : Option<f64>,
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
    let mut algo = Heuristic::V2;
    if let Some(x) = cli.heuristic {
        match x.as_str() {
            "harper" => algo = Heuristic::HARPER,
            "hcat" => algo = Heuristic::HCAT,
            "inout" => algo = Heuristic::INOUT,
            "noselfatt" => algo = Heuristic::NoSelfAtt,
            "card" => algo = Heuristic::Card,
            "maxb" => algo = Heuristic::Max,
            "counting" => algo = Heuristic::Counting,
            "perso" => algo = Heuristic::Perso,
            _ => {
                eprintln!("The heuristic {x} is not know, look at the help section for more detail");
                exit(1);
            }
        }
    }
    let mut task = Task { problem, semantics, argument : argument_name, algo,
        verbose : cli.verbose,
        old : cli.old,
        threshold : cli.thresold
    };
    if task.algo == Heuristic::V2 && task.threshold.is_none() {
        CategorizedBasedApproximateSolver::choice_threshold_v2_heuristic(&mut task);
    }
    let file = cli.input_af.clone().unwrap();
    let file_path = file.as_str();
    let start = Instant::now();
    let af = if let Some(fo) = cli.input_format {
        if fo == "apx" {
            parser::get_input(file_path, Format::Apx)
        }
        else {
            parser::get_input(file_path, Format::Cnf)
        }
    } else {
        parser::get_input(file_path, Format::Cnf)
    };
    if task.verbose {
        print!("{};",start.elapsed().as_millis() as f32 / 1000.0);
    }
    let _ = stdout().flush();
    if CategorizedBasedApproximateSolver::solve(af, task) {
        println!("YES");
    }
    else {
        println!("NO");
    }

}

fn print_supported_problems() {
    println!("[DC-CO,DC-ST,DC-SST,DC-STG,DC-ID,DS-PR,DS-ST,DS-SST,DS-STG]");
}