use std::fs;
use crate::graph::ArgumentationFramework;
use crate::cli::Format;

pub fn get_input(file_path : &str, format : Format) -> ArgumentationFramework {
    match format {
        Format::APX => readingAPX(file_path),
        Format::CNF => readingCNF(file_path),
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

pub fn readingAPX( file_path : &str) -> ArgumentationFramework {
    
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