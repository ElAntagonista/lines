use clap::{App, Arg};
use regex::Regex;
use std::io::{self, Read, Write};
enum RunType {
    Range(Vec<usize>),
    Lines(Vec<usize>),
}

struct LinesApp {
    run_type: RunType,
}

impl LinesApp {
    fn new() -> Self {
        let matches = App::new("lines")
            .version("1.0")
            .author("Kris Mb")
            .about("A simple util tool aimed at helping you work with std output more easily")
            .arg(
                Arg::new("index")
                    .about("Spaces separated numbers representing the lines order of appearence.\n\tEXAMPLE: `lines -i 1`, `lines -i 1 2`")
                    .required_unless_present("range")
                    .takes_value(true)
                    .long("index")
                    .short('i')
                    .max_values(10),
            )
            .arg(
                Arg::new("range")
                    .about("Range of lines to display. Works as `skip num`-`take num` so both 1-3 and 3-1 are valid values.\n\tEXAMPLE: `lines -r 1-3`")
                    .conflicts_with("index")
                    .required_unless_present("index")
                    .long("range")
                    .short('r')
                    .validator(is_range)
                    .max_values(10), //.validator(is_range),
            )
            .get_matches();

        if matches.is_present("index") {
            let lines_nums: Vec<usize> = matches.values_of_t_or_exit("index");
            LinesApp {
                run_type: RunType::Lines(lines_nums),
            }
        } else {
            let range: Vec<usize> = matches
                .value_of("range")
                .unwrap()
                .split('-')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            LinesApp {
                run_type: RunType::Range(range),
            }
        }
    }

    fn run(self) -> io::Result<()> {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        let mut lines = buffer.lines();
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        match self.run_type {
            RunType::Range(v) => range_out(v[0], v[1], &mut lines, &mut handle),
            RunType::Lines(v) => lines_out(&v, &mut lines, &mut handle),
        }
        Ok(())
    }
}

fn range_out(
    start: usize,
    end: usize,
    lines: &mut std::str::Lines,
    handle: &mut std::io::StdoutLock,
) {
    let mut range_lines = lines.skip(start).take(end);
    loop {
        let line = range_lines.next();
        if line == None {
            break;
        } else {
            handle
                .write_all(format!("{}\n", line.unwrap()).as_bytes())
                .unwrap();
        }
    }
}

fn lines_out(line_nums: &[usize], lines: &mut std::str::Lines, handle: &mut std::io::StdoutLock) {
    let mut counter = 0;
    loop {
        let line = lines.next();
        if line == None {
            break;
        }
        if line_nums.contains(&counter) {
            handle
                .write_all(format!("{}\n", line.unwrap()).as_bytes())
                .unwrap();
        }
        counter += 1;
    }
}

fn is_range(val: &str) -> Result<(), String> {
    let re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    if re.is_match(val) {
        Ok(())
    } else {
        Err(String::from("Range is not valid. Should be number-number"))
    }
}

fn main() -> io::Result<()> {
    let app = LinesApp::new();
    app.run().unwrap();
    Ok(())
}
