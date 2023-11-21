use std::fs::File;
use std::io::Read;
// ccwc -c test.txt -outputs the number of bytes in a file
// ccwc -l test.txt -outputs the number of lines in a file
// ccwc -w test.txt  -outputs the number of words in a file
//ccwc -m test.txt -outputs the number of characters in a file
// ccwc test.txt -output all
extern crate ansi_term;
extern crate clap;

use ansi_term::Color::{Blue, Green};
use clap::{Arg,App};

sturct FileStats{
name: String,
lines: usize,
word: usize,
characters: usize,
}

struct WcStats{
    stats: Vec<FileStats>,
    number_of_files: usize,
    total: FileStats,
    line_flag: bool,
    word_flog: bool,
    char_flag: bool,

}
impl WcStats{
    fn new() -> WcStats{
        WcStats{
            stats: Vec::new(),
            number_of_files: 0,
            total: FileStats{name: String::from(""), lines: 0, words: 0, characters: 0},
            line_flag: false,
            word_flag: false,
            char_flag: false,
        }
    }
}

fn getStats(&mut self, file: &str) -> Result<(), String> {
    let filename = file.to_string();
    match File::open(file){
        Ok(mut fd) => {
            let mut contents = String::new();
            match fd.read_to_string(&mut contents){
                Ok(_) => {
                    let lines: Vec<&str> = contents.lines.collect();
                    let words: Vec<&str> = contents.split_ascii_whitespace().collect();
                    
                    self.total.lines += lines.len();
                    self.total.words += words.len();
                    self.total.characters += contents.len();
                    self.stats.push(FileStats {
                        name: filename, lines: lines.len(), words: words.len(), characters: contents.len()
                    });
                    Ok(())
                }
                Err(e) =>{
                    Err(format! ("wc: {}: {}", filename, e))
                }
            }
        }
        Err(_) => {
            Err(format! ("wc: {} no such file or directory", filename))
       }
    }
}
fn print_to_console(self){
    for stat in self.stats{
        if self.line_flag {
            print!("{}\t", stat.lines)
        }
        if self.word_flag {
            print!("{}\t", stat.words)
        }
        if self.char_flag {
            print!("{}\t", stat.characters);
        }
        println!("{}", Green.dimmed().paint(stat.name));
    }
    if self.number_of_files > 1 {
        if self.line_flag {
            print!("{}\t", self.total.lines);
        }
        if self.word_flag {
            print!("{}\t", self.total.words);
        }
        if self.char_flag {
            print!("{}\t", self.total.characters);
        }
        println!("{}", Blue.bold().paint("total"));
    }
}
fn main() -> Result<(), Error>{
   let matches = App::new("wc-clone.rs")
   .version("0.0.1")
   .author("Farjmand <farjmand.zara6@gmail.com")
   .about("The good old wc rewritten in rust")
   .arg(Arg::with_name(lines)
            .short("l")
            .long("lines")
            .help("prints the newline counts")
            )
    .arg(Arg::with_name(words)
            .short("w")
            .long("words")
            .help("prints the words counts")
            )
    .arg(Arg::with_name(characters)
            .short("m")
            .long("characters")
            .help("prints the characters counts")
            )
     .arg(Arg::with_name("FILE")
            .help("file path(s) to run wc on")
            .multiple(true)
            .empty_values(false))

    Ok(())
}
