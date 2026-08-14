use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: c_blunt <file.cb> [-o output_binary]");
        std::process::exit(1);
    }
    let in_path = &args[1];
    let out_path = if args.len() >= 4 && args[2] == "-o" {
        args[3].clone()
    } else {
        in_path.trim_end_matches(".cb").to_string()
    };

    let src = fs::read_to_string(in_path).unwrap_or_else(|err| {
        eprintln!("Error reading file '{in_path}': {err}");
        std::process::exit(1);
    });
    let c_code = to_c(&src);
    let temp_c = format!("{out_path}_temp.c");
    fs::write(&temp_c, c_code).expect("Failed to write temp C file");

    println!("Compiling {} -> {}...", in_path, out_path);
    let status = Command::new("cc")
        .arg(&temp_c)
        .arg("-o")
        .arg(&out_path)
        .status();

    let _ = fs::remove_file(temp_c);

    match status {
        Ok(s) if s.success() => println!("Successfully compiled: ./{}", out_path),
        _ => eprintln!("Error: C compiler failed ('cc'). Ensure gcc/clang is installed."),
    }
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
            '▒' => {
                out.push_str("while (*ptr){{");
            }
            '▓' => {
                out.push_str("}}");
            }
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
            '▒' => {
                out.push_str("while(tape[ptr]!=0){{");
            }
            '▓' => {
                out.push_str("}}");
            }
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
