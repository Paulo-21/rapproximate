use std::fs;
use memchr::memchr;
use memmap2::Mmap;
use crate::graph::ArgumentationFramework;
use crate::cli::Format;

pub fn get_input(file_path : &str, format : Format) -> ArgumentationFramework {
    match format {
        Format::APX => reading_apx(file_path),
        //Format::CNF => reading_cnf(file_path),
        Format::CNF => reading_cnf_perf(file_path),
    }
}

pub fn reading_cnf( file_path : &str) -> ArgumentationFramework {
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
fn bytes_to_int(bytes: &[u8]) -> Option<i32> {
    let mut result = 0;
    for &byte in bytes {
        // Vérifie si le caractère est un chiffre ASCII
        if byte >= b'0' && byte <= b'9' {
            result = result * 10 + (byte - b'0') as i32;
        } else {
            // Si un caractère n'est pas un chiffre, retourne None (échec)
            return None;
        }
    }
    Some(result)
}
pub fn reading_cnf_perf( file_path : &str) -> ArgumentationFramework{
    let mmap: Mmap;
    let mut data;
    {
        let file = std::fs::File::open(file_path).unwrap();
        mmap = unsafe { Mmap::map(&file).unwrap() };
        data = &*mmap;
    }
    
    let Some(separator) = memchr(b' ', data) else {panic!("oups")};
    data = &data[separator+1..];
    let Some(separator) = memchr(b' ', data) else {panic!("oups")};
    data = &data[separator+1..];
    let end = memchr(b'\n', &data).unwrap();
    let nb_arg = bytes_to_int(&data[.. end]).unwrap() as usize;
    let mut af = ArgumentationFramework::new(nb_arg);
    data = &data[end + 1..];
    loop {
        let Some(separator) = memchr(b' ', data) else {
            break;
        };
        let Some(end) = memchr(b'\n', &data[separator..]) else {break;};
        let att = bytes_to_int(&data[..separator]).unwrap();
        let target = bytes_to_int(&data[separator + 1..separator + end]).unwrap();
        af.add_attack(att, target);
        data = &data[separator + end + 1..];
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
#[inline(always)]
fn parse_cnfattack_line (line : &str) -> (i32,i32) {
    let mut a = line.split_ascii_whitespace();
    let att = a.next().unwrap().parse::<i32>().unwrap();
    let targ = a.next().unwrap().parse::<i32>().unwrap();
    (att,targ)
}

pub fn reading_apx( file_path : &str) -> ArgumentationFramework {
    
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