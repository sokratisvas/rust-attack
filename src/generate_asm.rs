use crate::commands;
use std::fs::File;
use std::io::prelude::*;

pub fn partition_instr(instruction: String) -> Vec<String> {
    instruction.split(' ').map(str::to_string).collect()
}

pub fn boolean_index(command: String) -> usize {
    match command.as_str() {
        "eq" => 0,
        "gt" => 1,
        "lt" => 2,
        _ => {
            println!("error: invalid command");
            3
        }
    }
}

pub fn asm(contents: Vec<String>, filename: String, mult_files: bool) -> Vec<String> {
    let mut asm_code: Vec<String> = Vec::new();
    let mut boolean_cnt: [u32; 3] = [0; 3];
    let mut call_cnt: u32 = 1;

    if mult_files == true {
        asm_code.push(commands::bootstrap());
    }

    for line in contents {
        let instruction: Vec<String> = partition_instr(line);
        match instruction.len() {
            1 => {
                if instruction[0] == "eq" || instruction[0] == "gt" || instruction[0] == "lt" {
                    asm_code.push(commands::arithmetic_command(
                        instruction[0].clone(),
                        boolean_cnt[boolean_index(instruction[0].clone())],
                    ));
                    boolean_cnt[boolean_index(instruction[0].clone())] += 1;
                } else if instruction[0] == "return" {
                    asm_code.push(commands::funret());
                } else {
                    asm_code.push(commands::arithmetic_command(instruction[0].clone(), 0));
                }
            }

            2 => {
                asm_code.push(commands::branching_command(
                    instruction[0].clone(),
                    instruction[1].clone(),
                ));
            }

            3 => {
                if instruction[0] == "push" {
                    asm_code.push(commands::push_command(
                        instruction[1].clone(),
                        instruction[2].clone(),
                        filename.clone(),
                    ));
                } else if instruction[0] == "pop" {
                    asm_code.push(commands::pop_command(
                        instruction[1].clone(),
                        instruction[2].clone(),
                        filename.clone(),
                    ));
                } else if instruction[0] == "call" {
                    asm_code.push(commands::funcall(
                        instruction[1].clone(),
                        instruction[2].clone(),
                        call_cnt,
                    ));
                    call_cnt += 1;
                } else {
                    asm_code.push(commands::fundecl(
                        instruction[1].clone(),
                        instruction[2].clone(),
                    ));
                }
            }

            _ => println!(" "),
        }
    }
    asm_code.push(format!("(END)\n@END\n0;JMP\n"));
    asm_code
}

pub fn write_output(asm: Vec<String>, mut filename: String) -> std::io::Result<()> {
    let outpath: String;
    if filename.ends_with(".vm") {
        outpath = filename.replace(".vm", ".asm");
    } else {
        filename.push_str(".asm");
        outpath = filename;
    };
    let mut output = File::create(outpath)?;
    for line in asm {
        write!(output, "{}", line)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }
    Ok(())
}
