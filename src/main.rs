use std::fs::File;
use std::io::prelude::*;
use std::str::Chars;
use console::Term;

#[macro_use]
extern crate clap;

enum Command {
    Right,
    Left,
    Add,
    Substract,
    Out,
    In,
    While(Vec<Command>)
}

fn parse(program: &mut Chars) -> Vec<Command> {
    let mut commands = vec![];

    loop {
        match program.next() {
            Some('>') => commands.push(Command::Right),
            Some('<') => commands.push(Command::Left),
            Some('+') => commands.push(Command::Add),
            Some('-') => commands.push(Command::Substract),
            Some('.') => commands.push(Command::Out),
            Some(',') => commands.push(Command::In),
            Some('[') => commands.push(Command::While(parse(program))),
            None | Some(']') => break,
            _ => continue
        }
    }

    commands
}

fn run(commands: &[Command], memory: &mut [u32], pointer: &mut usize) {
    for c in commands {
        match c {
            Command::Right => *pointer += 1,
            Command::Left => *pointer -= 1,
            Command::Add => memory[*pointer] += 1,
            Command::Substract => memory[*pointer] -= 1,
            Command::Out => print!("{}", memory[*pointer] as u8 as char),
            Command::In => memory[*pointer] = Term::stdout().read_char().unwrap_or_default() as u32,
            Command::While(w) => while memory[*pointer] != 0 { run(w, memory, pointer) }
        }
    }
}

fn main() -> std::io::Result<()> {
    let matches = clap_app!(bfrs =>
        (version: "1.0")
        (author: "David N. <dabsunter@gmail.com>")
        (about: "Run your favorite brainfuck programs !")
        (@arg MEM: -m --memory +takes_value "Sets the size of brainfuck array")
        (@arg CMD: -c --command "Directly run the program passed as string")
        (@arg INPUT: +required "Sets the input program to run (file by default)")
    ).get_matches();

    let mut program = String::from(matches.value_of("INPUT").unwrap());
    if matches.occurrences_of("CMD") == 0 {
        let mut bf_file = File::open(program)?;
        program = String::new();
        bf_file.read_to_string(&mut program)?;
    }

    let commands = parse(program.chars().by_ref());

    let mut memory = vec![0; match matches.value_of("MEM") {
        Some(m) => m.parse().unwrap(),
        None => 30000
    }];
    let mut pointer: usize = 0;

    run(&commands, &mut memory, &mut pointer);

    Ok(())
}
