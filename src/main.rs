use std::vec::Vec;
use std::io::{BufReader, BufRead};
use std::fs::File;
extern crate sort_lib;
use sort_lib::quick_sort::quick_sort;
// use quick_r_sort::quick_r_sort::quick_sort;





fn main() {

    let file = File::open("numbers3.txt").unwrap();
    let file_reader = BufReader::new(file);
    let mut vector: Vec<isize> = vec![];

    for line in file_reader.lines() {
        match line {
            Err(why)   => panic!("{:?}", why),
            Ok(string) => match string.trim().parse::<isize>() {
                Err(why)        => panic!("Not a number! {:?}", why),
                Ok(number)=> vector.push(number)
            }
        }
    }
    

    quick_sort(&mut vector.clone());


    // println!("{:?}", vector);
    println!("Done sorting");

    
}

