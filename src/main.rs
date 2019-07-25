use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::str::Chars;
use std::time::SystemTime;

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

fn transpile_to_c(program: &mut Chars, out: &mut File) -> std::io::Result<()> {
    loop {
        match program.next() {
            Some('>') => out.write(b"++p;")?,
            Some('<') => out.write(b"--p;")?,
            Some('+') => out.write(b"++*p;")?,
            Some('-') => out.write(b"--*p;")?,
            Some('.') => out.write(b"putchar(*p);")?,
            Some(',') => out.write(b"*p=getchar();")?,
            Some('[') => out.write(b"while(*p){")?,
            Some(']') => out.write(b"}")?,
            None => break,
            _ => continue
        };
    }
    Ok(())
}

fn run(commands: &[Command], memory: &mut [u32], pointer: &mut usize) {
    for c in commands {
        match c {
            Command::Right => *pointer += 1,
            Command::Left => *pointer -= 1,
            Command::Add => memory[*pointer] += 1,
            Command::Substract => memory[*pointer] -= 1,
            Command::Out => print!("{}", memory[*pointer] as u8 as char),
            Command::In
            => memory[*pointer] = std::io::stdin()
                .bytes()
                .next()
                .expect("Invalid input")
                .unwrap() as u32,
            Command::While(w)
            => while memory[*pointer] != 0 {
                run(w, memory, pointer)
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let matches = clap_app!(bfrs =>
        (version: "1.0")
        (author: "David N. <dabsunter@gmail.com>")
        (about: "Run your favorite brainfuck programs !")
        (@arg MEM: -m --memory +takes_value "Sets the size of brainfuck array")
        (@arg TR2C: -tr2c --transpileToC +takes_value "Transpile to C and sets out file")
        (@arg DEBUG: -d --debug "Dump some values")
        (@arg CMD: -c --command "Directly run the program passed as string")
        (@arg INPUT: +required "Sets the input program to run (file by default)")
    ).get_matches();

    let mut program = String::from(matches.value_of("INPUT").unwrap());
    if matches.occurrences_of("CMD") == 0 {
        let mut bf_file = File::open(program)?;
        program = String::new();
        bf_file.read_to_string(&mut program)?;
    }

    let sys_time = SystemTime::now();

    if matches.occurrences_of("TR2C") == 0 {
        let commands = parse(program.chars().by_ref());

        let mut memory = vec![0; match matches.value_of("MEM") {
            Some(m) => m.parse().unwrap(),
            None => 30000
        }];
        let mut pointer: usize = 0;

        run(&commands, &mut memory, &mut pointer);
    } else {
        let mut out = File::create(matches.value_of("TR2C").unwrap())?;
        out.write(format!("#include <stdio.h>\n int main(){{char a[{}]={{0}};char *p=a;",
                          matches.value_of("MEM").unwrap_or("30000")).as_bytes())?;
        transpile_to_c(program.chars().by_ref(), &mut out)?;
        out.write(b"}")?;
    }

    if matches.occurrences_of("DEBUG") != 0 {
        println!();
        let difference = sys_time.elapsed().unwrap();
        println!("Elapsed time: {:?}", difference);
    }

    Ok(())
}
