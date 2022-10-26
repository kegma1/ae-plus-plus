use crate::ops::Pos;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
enum Mode {
    Normal,
    String,
}

pub fn lex(path: &String) -> Result<Vec<(String, Pos)>, &'static str> {
    let mut prg: Vec<(String, Pos)> = vec![];

    if let Ok(lines) = read_lines(path) {
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let mut col = 1;
                let mut word = String::from("");
                let mut mode = Mode::Normal;
                for char in ip.chars() {
                    match (char, &mode) {
                        ('#', Mode::Normal) => break,
                        ('"', Mode::Normal) => {
                            word.push(char);
                            mode = Mode::String
                        }
                        ('"', Mode::String) => {
                            word.push(char);
                            mode = Mode::Normal
                        }
                        (' ', Mode::Normal) => {
                            if !word.is_empty() {
                                prg.push((word.clone(), (i + 1, col, path.clone())));
                                col += word.len() + 1;
                                word.clear();
                            }
                        }
                        (_, _) => word.push(char),
                    }
                }
                prg.push((word.clone(), (i + 1, col, path.clone())));
            }
        }
    }

    Ok(prg)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
