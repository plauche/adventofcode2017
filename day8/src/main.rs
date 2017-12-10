use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn main() {
    let path = Path::new("puzzle.txt");
    let display = path.display();
    let mut registers = HashMap::<String, i32>::new();
    let mut actualbig = 0;
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("can't read {}: {}", display, why.description()),
        Ok(_) => {}
    }

    for line in s.lines() {
        let mut parts = line.split_whitespace();
        let reg = parts.next().unwrap();
        let dir = parts.next().unwrap();
        let val: i32 = parts.next().unwrap().parse().expect("gimme num");
        parts.next();
        let creg = parts.next().unwrap();
        let op = parts.next().unwrap();
        let opval: i32 = parts.next().unwrap().parse().expect("gimme num");
        let mut comp = false;

        let rval = match registers.entry(String::from(creg)) {
            Vacant(v) => {
                v.insert(0);
                0
            }
            Occupied(o) => {
                let rval = o.get();

                *rval
            }
        };

        let comp = match op {
            ">" => rval > opval,
            ">=" => rval >= opval,
            "<" => rval < opval,
            "<=" => rval <= opval,
            "==" => rval == opval,
            "!=" => rval != opval,
            _ => false,
        };

        if comp {

            let mut realval = match registers.get(reg) {
                Some(rval) => *rval,
                None => 0,
            };

            if dir == "inc" {
                realval += val;
            } else if dir == "dec" {
                realval -= val;
            }

            registers.insert(String::from(reg), realval);
            if realval > actualbig {
                actualbig = realval;
            }
        }

    }

    let mut bigval = 0;

    for (key, value) in &registers {
        println!("{}: {}", key, value);
        if *value > bigval {
            bigval = *value;
        }
    }

    println!("Biggest val is {}", bigval);
    println!("Biggest ever val is {}", actualbig);
}
