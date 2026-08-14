use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "glasses" => {
            if args.len() < 3 {
                eprintln!(
                    "Error: Missing input file.\nUsage: c_blunt glasses <file.cb> [-o output.cs]"
                );
                std::process::exit(1);
            }
            let input_path = &args[2];
            let output_path = if args.len() >= 5 && args[3] == "-o" {
                args[4].clone()
            } else {
                format!("{}.cs", input_path.trim_end_matches(".cb"))
            };

            to_cs(input_path, &output_path);
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            let input_path = &args[1];
            let output_path = if args.len() >= 4 && args[2] == "-o" {
                args[3].clone()
            } else {
                format!("{}.c", input_path.trim_end_matches(".cb"))
            };

            let src = fs::read_to_string(input_path).unwrap_or_else(|err| {
                eprintln!("Error reading file '{input_path}': {err}");
                std::process::exit(1);
            });

            let c_code = to_c(&src);
            fs::write(&output_path, c_code).expect("Failed to write C file");
            println!("Compiled {} -> {}", input_path, output_path);
        }
    }
}

fn print_help() {
    println!("C Blunt (.cb) Compiler");
    println!("Usage:");
    println!("  c_blunt <file.cb> [-o file.c]        Transpile to C source code");
    println!(
        "  c_blunt glasses <file.cb> [-o file.cs] Transpile to C# (putting on glasses lets you C Sharp)"
    );
}

fn to_c(source: &str) -> String {
    let mut out = String::from(
        "#include <stdio.h>\n#include <stdlib.h>\nint main(){static unsigned char tape[30000]={0};unsigned char *ptr=tape;",
    );

    for ch in source.chars() {
        match ch {
            '▌' => out.push_str("++ptr;"),
            '▐' => out.push_str("--ptr;"),
            '▀' => out.push_str("++(*ptr);"),
            '▄' => out.push_str("--(*ptr);"),
            '█' => out.push_str("putchar(*ptr);"),
            '░' => out.push_str("*ptr=getchar();"),
            '▒' => out.push_str("while (*ptr){"),
            '▓' => out.push_str("}"),
            _ => {}
        }
    }

    out.push_str("return 0;}");
    out
}

fn to_cs(input_path: &str, output_path: &str) {
    let src = fs::read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("Error reading file '{input_path}': {err}");
        std::process::exit(1);
    });

    let mut out = String::from(
        "using System;class Program{static void Main(){byte[] tape=new byte[30000];int ptr=0;",
    );

    for ch in src.chars() {
        match ch {
            '▌' => out.push_str("ptr = (ptr + 1) % 30000;"),
            '▐' => out.push_str("ptr = (ptr == 0) ? 29999 : ptr - 1;"),
            '▀' => out.push_str("tape[ptr]++;"),
            '▄' => out.push_str("tape[ptr]--;"),
            '█' => out.push_str("Console.Write((char)tape[ptr]);"),
            '░' => out.push_str("tape[ptr]=(byte)Console.Read();"),
            '▒' => out.push_str("while(tape[ptr]!=0){"),
            '▓' => out.push_str("}"),
            _ => {}
        }
    }

    out.push_str("}}");

    fs::write(output_path, out).expect("Failed to write C# file");
    println!(
        "Put on glasses! Transpiled {} -> {}",
        input_path, output_path
    );
}
