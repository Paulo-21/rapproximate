mod parser;
mod graph;
use clap::Parser;
use std::process::exit;

fn print_supported_problems() {
    println!("[DC-CO,DC-ST,DC-SST,DC-STG,DC-ID,DS-PR,DS-ST,DS-SST,DS-STG]");
}

#[derive(Parser, Debug)]
#[command(author="Paul Cibier", version, about="This tool can solve all the problems in the approximate track of ICCMA 2023",
 long_about = None)]
 struct Cli {
    #[arg(short, long)]
    /// Quary argument for credulous and skeptical acceptance
    argument : String,
    #[arg(short='f', long="input_AF")]
    /// Path of the file containing the AF.
    input_af : String,
    #[arg(short = 'p', long="task")]
    /// A computational problem supported by the solver (e.g. DC-CO, DS-PR).
    task : String,
    #[arg( long, action = clap::ArgAction::Count)]
    /// Prints the supported computational problems and exits
    problems : u8
}


fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
    
    if cli.problems > 0 { // Print support problem if --problems
        print_supported_problems();
        exit(0);
    }



    //println!("{}", arg.next().unwrap());
    println!("Hello, world!");
}
