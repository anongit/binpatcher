use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{io, process};

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let exec = args.next();
    let search = args.next();
    let replacement = args.next();
    let mut args = args.map(PathBuf::from);

    match (search, replacement, args.next(), args.next()) {
        (Some(search), Some(replace), Some(input), Some(output)) => {
            patch(&input, &output, &search, &replace)?
        }
        _ => {
            eprintln!(
                "Usage: {} <search> <replace> <input> <output>\n",
                exec.unwrap()
            );
            process::exit(1);
        }
    };

    Ok(())
}

fn parse_hexdigit(digit: u8) -> u8 {
    match digit {
        b'0'..=b'9' => digit - b'0',
        b'a'..=b'f' => digit - b'a' + 10,
        b'A'..=b'F' => digit - b'A' + 10,
        _ => {
            eprintln!(
                "invalid digit, expected [0-9a-f], got: {:?}",
                char::from(digit)
            );
            process::exit(3);
        }
    }
}

fn parse_hex(value: &str) -> Vec<u8> {
    let value: Vec<u8> = value.bytes().filter(|&x| x != b'_').collect();

    let mut buf = vec![];
    if value.len() % 2 != 0 {
        eprintln!(
            "value must have an even length, got: {:?}, len: {}",
            String::from_utf8_lossy(&value),
            value.len()
        );
        process::exit(3);
    }
    for v in value.chunks_exact(2) {
        buf.push(parse_hexdigit(v[0]) * 16 + parse_hexdigit(v[1]));
    }
    buf
}

fn patch(input: &Path, output: &Path, search: &str, replace: &str) -> io::Result<()> {
    if output == input {
        eprintln!("<input> and <output> can't be the same file");
        process::exit(1);
    }

    let search_term = parse_hex(search);
    let replacement = parse_hex(replace);
    let len = search_term.len();
    if len != replacement.len() {
        eprintln!(
            "<search> and <replace> must have the same length, got: {} and {}",
            search.len(),
            replace.len()
        );
        process::exit(1);
    }

    let mut f = File::open(input)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let positions: Vec<usize> = buffer
        .windows(len)
        .enumerate()
        .filter_map(|(p, w)| if w == search_term { Some(p) } else { None })
        .collect();

    if positions.is_empty() {
        println!("search term not found");
        process::exit(2);
    }

    for (i, pos) in positions.iter().enumerate() {
        println!("{}] {}", i, pos);
    }
    println!("q] quit");

    let mut input_text = String::new();
    let pos = loop {
        println!("\nChoose:");

        input_text.clear();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        if trimmed == "q" {
            println!("exiting...");
            return Ok(());
        }

        match trimmed.parse::<usize>() {
            Ok(i) if i < positions.len() => break positions[i],
            _ => println!(
                "Expected a number between 0 and {}, got: {:?}",
                positions.len(),
                trimmed
            ),
        };
    };
    println!();

    // write log file
    let log = format!(
        concat!(
            "input:   {:?}\n",
            "output:  {:?}\n",
            "search:  {:?}\n",
            "replace: {:?}\n\n",
            "md5sum: {:x}\n",
            "offset:\n",
            "   dec: {5}\n",
            "   hex: 0x{5:x}\n",
        ),
        input,
        output,
        search,
        replace,
        md5::compute(&buffer),
        pos
    );

    let mut out = File::create(output.with_extension("log"))?;
    out.write_all(log.as_bytes())?;
    println!("{}", &log);

    // do the actual replacement
    let mut out = File::create(&output)?;
    buffer[pos..pos + len].copy_from_slice(&replacement);
    out.write_all(&buffer)?;

    println!("done");
    Ok(())
}
