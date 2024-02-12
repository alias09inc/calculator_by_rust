use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

#[derive(Parser)]
#[command(
    name = "aliAs's RPN program",
    version = "0.1.1",
    author = "aliAs09inc",
    about = "This is my first program"
)]
struct Opts {
    // Sets the level of verbosity
    #[arg(long="verbose", short='v')]
    verbose: bool,

    // Formulas written in RPN
    #[arg(name = "FILE")]
    forumula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.forumula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        // println!("No file is specified");
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
    }
}

struct RpnCalculator(bool);


impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }
        }
    }
}