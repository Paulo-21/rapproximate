use std::fs::{self, File};
use std::io::{BufRead, self};
use std::path::Path;
use std::process::exit;
use crate::graph::ArgumentationFramework;
use crate::cli::Format;

pub fn get_input(file_path : &str, format : Format) -> ArgumentationFramework {
    match format {
        Format::APX => readingAPX(file_path),
        Format::CNF => readingCNF(file_path),
        //Format::CNF => readingCNF_perf(file_path),
    }
}

pub fn readingCNF( file_path : &str) -> ArgumentationFramework {
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
let mut content_iter = contents.trim().split('\n');
let first_line = content_iter.next().unwrap();
let iter: Vec<&str> = first_line.split_ascii_whitespace().collect();
let nb_arg = iter[2].parse::<usize>().unwrap();
let mut af = ArgumentationFramework::new(nb_arg);
for line in content_iter {
    if !line.starts_with('#') {//&& (!line.trim().eq("")) {
        let (attacker,target) = parse_cnfattack_line(line);
        //println!("{} {}", attacker, target);
        af.add_attack(attacker, target);
    }
}
    af
}
pub fn readingCNF_perf( file_path : &str) -> ArgumentationFramework {
    
    if let Ok(mut lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        let first_line = lines.next().unwrap().unwrap();
        let iter: Vec<&str> = first_line.split_ascii_whitespace().collect();
        let nb_arg = iter[2].parse::<usize>().unwrap();
        let mut af = ArgumentationFramework::new(nb_arg);
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() || ip.starts_with('#') {
                    break;
                }
                let mut split = ip.split_ascii_whitespace();
                let attacker = split.next().unwrap().parse::<i32>().unwrap();
                let target = split.next().unwrap().parse::<i32>().unwrap();
                af.add_attack(attacker, target);
            }
        }
        return af;
    }
    exit(0);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let buffsize = 1<<20;
    let file = File::open(filename)?;
    //println!("{}",buffsize);
    Ok(io::BufReader::with_capacity(buffsize, file).lines())
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
#[inline(always)]
fn parse_cnfattack_line (line : &str) -> (i32,i32) {
    let mut a = line.split_ascii_whitespace();
    let att = a.next().unwrap().parse::<i32>().unwrap();
    let targ = a.next().unwrap().parse::<i32>().unwrap();
    (att,targ)
}

pub fn readingAPX( file_path : &str) -> ArgumentationFramework {
    
    let nb_arg = find_number_argument(file_path);
    let af = ArgumentationFramework::new(nb_arg as usize);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let a = contents.trim().split('\n');

    for line in a {
        if !line.starts_with('#') && (!line.trim().eq("")) {
            //af.add
        }
    }
    
    af
}