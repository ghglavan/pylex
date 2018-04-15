mod dfa;

#[macro_use]
extern crate clap;


use std::fs::File;
use std::io::prelude::*;

fn main() {

    let matches = clap_app!(PyLexer =>
        (version: "sqrt(7)")
        (author: "GGlavan <https://github.com/glavangeorge>")
        (about: "Lexer for python")
        (@arg INPUT: +required "File to parse")
        (@arg OUTPUT: +required "File to write")
    ).get_matches();


    let in_filename = matches.value_of("INPUT").unwrap();
    let out_filename = matches.value_of("OUTPUT").unwrap();

    let mut f = File::open(in_filename).expect("input file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");


    let mut df = dfa::Tokenizer::new(String::from(contents));

    let mut err_message = String::new();

    while let Some((t, val, poz)) = df.get_token() {
        println!("Got token {} - {:?}", val, t);
        if val == String::from("err") {
            err_message = format!("Got error {} at {}", t, poz);
        }
    }

    let mut o_f = File::create(out_filename).expect("output file not found");

    for ((t, val), positions) in df.get_table() {
        write!(o_f, "{} - {:?} {:?} \n", t, val, positions).unwrap();
    }
    
    if err_message.len() > 0 {
        write!(o_f, "{}", err_message).unwrap();
    }

}
