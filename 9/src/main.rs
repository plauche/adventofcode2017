use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::Chars;

fn solve_group(level: i32, c: &mut Chars) -> (i32, i32) {
    let mut score = level;
    let mut count = 0;
    loop {
        match c.next() {
            Some(next) => {
                if next == '{' {
                    let (_score, _count) = solve_group(level + 1, c);
                    score += _score;
                    count += _count;
                } else if next == '}' {
                    break;
                } else if next == '<' {
                    count += solve_garbage(c);
                }
            }
            _ => {
                break;
            }
        }
    }
    (score, count)
}

fn solve_garbage(c: &mut Chars) -> i32 {
    let mut count = 0;
    loop {
        match c.next() {
            Some(next) => {
                if next == '>' {
                    break;
                } else if next == '!' {
                    c.next();
                } else {
                    count += 1;
                }
            }
            _ => {
                break;
            }
        }
    }
    count
}

fn solve(level: i32, c: &mut Chars) -> (i32, i32) {
    let mut score = 0;
    let mut count = 0;
    loop {
        match c.next() {
            Some(next) => {
                if next == '{' {
                    let (_score, _count) = solve_group(level + 1, c);
                    score += _score;
                    count += _count;
                } else if next == '<' {
                    count += solve_garbage(c);
                }
            }
            None => {
                break;
            }
        };
    }
    (score, count)
}


fn main() {
    let path = Path::new("puzzle.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open"),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read"),
        Ok(_) => {}
    }

    let (score, count) = solve(0, &mut (s.chars()));
    println!("Score {}", score);
    println!("Count {}", count);
}
