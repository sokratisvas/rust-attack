pub fn segment_mapping(segment: String) -> String {
    match segment.as_str() {
        "local" => "LCL",
        "argument" => "ARG",
        "this" => "THIS",
        "that" => "THAT",
        _ => "error: invalid segment",
    }
    .to_string()
}

pub fn arithmetic_command(command: String, cnt: u32) -> String {
    match command.as_str() {
        "add" => "@SP\n\
                  M=M-1\n\
                  A=M\n\
                  D=M\n\
                  A=A-1\n\
                  M=M+D\n"
            .to_string(),

        "sub" => "@SP\n\
                  M=M-1\n\
                  A=M\n\
                  D=M\n\
                  A=A-1\n\
                  M=M-D\n"
            .to_string(),

        "neg" => "@SP\n\
                 A=M-1\n\
                 M=-M\n"
            .to_string(),

        "eq" => format!(
            "@SP\n\
                    M=M-1\n\
                    A=M\n\
                    D=M\n\
                    A=A-1\n\
                    D=M-D\n\
                    @ISEQUAL{0}\n\
                    D;JEQ\n\
                    @NOTEQUAL{0}\n\
                    D;JNE\n\
                    (ISEQUAL{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=-1\n\
                    @EQEND{0}\n\
                    0;JMP\n\
                    (NOTEQUAL{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=0\n\
                    @EQEND{0}\n\
                    0;JMP\n\
                    (EQEND{0})\n",
            cnt
        ),

        "not" => "@SP\n\
                 A=M-1\n\
                 M=!M\n"
            .to_string(),

        "and" => "@SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                A=A-1\n\
                M=M&D\n"
            .to_string(),

        "or" => "@SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                A=A-1\n\
                M=M|D\n"
            .to_string(),

        "gt" => format!(
            "@SP\n\
                    M=M-1\n\
                    A=M\n\
                    D=M\n\
                    A=A-1\n\
                    D=M-D\n\
                    @ISGREATER_GT{0}\n\
                    D;JGT\n\
                    @ISLESS_GT{0}\n\
                    D;JLE\n\
                    (ISGREATER_GT{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=-1\n\
                    @GTEND{0}\n\
                    0;JMP\n\
                    (ISLESS_GT{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=0\n\
                    @GTEND{0}\n\
                    0;JMP\n\
                    (GTEND{0})\n",
            cnt
        ),

        "lt" => format!(
            "@SP\n\
                    M=M-1\n\
                    A=M\n\
                    D=M\n\
                    A=A-1\n\
                    D=M-D\n\
                    @ISGREATER_LT{0}\n\
                    D;JGE\n\
                    @ISLESS_LT{0}\n\
                    D;JLT\n\
                    (ISGREATER_LT{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=0\n\
                    @LTEND{0}\n\
                    0;JMP\n\
                    (ISLESS_LT{0})\n\
                    @SP\n\
                    A=M-1\n\
                    M=-1\n\
                    @LTEND{0}\n\
                    0;JMP\n\
                    (LTEND{0})\n",
            cnt
        ),

        _ => format!("error: invalid arithmetic command"),
    }
}

pub fn push_command(segment: String, address: String) -> String {
    match segment.as_str() {
        "local" | "argument" | "this" | "that" => format!(
            "@{0}\n\
                D=A\n\
                @{1}\n\
                D=D+M\n\
                @addr\n\
                M=D\n\
                A=M\n\
                D=M\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n",
            address,
            segment_mapping(segment.clone())
        ),

        "constant" => format!(
            "@{0}\n\
                D=A\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n",
            address
        ),

        "temp" => format!(
            "@{0}\n\
                D=A\n\
                @5\n\
                D=D+A\n\
                A=D\n\
                D=M\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n",
            address
        ),

        "static" => format!(
            "@STC.{0}\n\
                D=M\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n",
            address
        ),

        "pointer" => {
            let memory_idx = if address == "0" { "THIS" } else { "THAT" };

            format!(
                "@{0}\n\
                D=M\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n",
                memory_idx
            )
        }
        _ => format!("todo push segment!"),
    }
}

pub fn pop_command(segment: String, address: String) -> String {
    match segment.as_str() {
        "local" | "argument" | "this" | "that" => format!(
            "@{0}\n\
                D=A\n\
                @{1}\n\
                D=D+M\n\
                @addr\n\
                M=D\n\
                @SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                @addr\n\
                A=M\n\
                M=D\n",
            address,
            segment_mapping(segment.clone())
        ),

        "temp" => format!(
            "@{0}\n\
                D=A\n\
                @5\n\
                D=D+A\n\
                @addr\n\
                M=D\n\
                @SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                @addr\n\
                A=M\n\
                M=D\n",
            address
        ),

        "static" => format!(
            "@SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                @STC.{0}\n\
                M=D\n",
            address
        ),

        "pointer" => {
            let memory_idx = if address == "0" { "THIS" } else { "THAT" };

            format!(
                "@SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                @{0}\n\
                M=D\n",
                memory_idx
            )
        }
        _ => format!("todo push segment!"),
    }
}

pub fn branching_command(arg1: String, arg2: String) -> String {
    match arg1.as_str() {
        "label" => format!("({})\n", arg2),
        "goto" => format!("@{}\n0;JMP\n", arg2),
        "if-goto" => format!("@SP\nM=M-1\nA=M\nD=M\n@{}\nD;JNE\n", arg2),
        _ => format!("error: invalid branch")
    }
}

pub fn funcall(fun: String, args: String, cnt: u32) -> String {
    format!(
        "// push return address\n\
        @{0}_RETURN_{2}\n\
        D=A\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n\
        // push LCL\n\
        @LCL\n\
        D=M\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n\
        // push ARG\n\
        @ARG\n\
        D=M\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n\
        // push THIS\n\
        @THIS\n\
        D=M\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n\
        // push THAT\n\
        @THAT\n\
        D=M\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n\
        // ARG = SP - 5 - nArgs\n\
        @SP\n\
        D=M\n\
        @5\n\
        D=D-A\n\
        @{1}\n\
        D=D-A\n\
        @ARG\n\
        M=D\n\
        // LCL = SP\n\
        @SP\n\
        D=M\n\
        @LCL\n\
        M=D\n\
        // goto function\n\
        @{0}\n\
        0;JMP\n\
        // label\n\
        ({0}_RETURN_{2})\n",
        fun,
        args,
        cnt
    )
}

pub fn fundecl(fun: String, args: String) -> String {
    let local_init: &str = "@SP\nA=M\nM=0\n@SP\nM=M+1\n";
    let mut block: String = String::new();
    for i in 0..args.parse::<i32>().unwrap() {
        block.push_str(local_init);
    }
    format!("// function declaration\n({0})\n{1}", fun, block)
}

pub fn funret() -> String {
    format!(
        "// endframe = LCL\n\
         @LCL\n\
         D=M\n\
         @endframe\n\
         M=D\n\
         // retaddr = *(endframe - 5)\n\
         @endframe\n\
         D=M\n\
         @5\n\
         D=D-A\n\
         A=D\n\
         D=M\n\
         @retaddr\n\
         M=D\n\
         // *ARG = pop()\n\
         @SP\n\
         M=M-1\n\
         A=M\n\
         D=M\n\
         @ARG\n\
         A=M\n\
         M=D\n\
         // SP = ARG + 1\n\
         @ARG\n\
         D=M\n\
         D=D+1\n\
         @SP\n\
         M=D\n\
         // THAT = *(endframe - 1)\n\
         @endframe\n\
         D=M\n\
         @1\n\
         D=D-A\n\
         A=D\n\
         D=M\n\
         @THAT\n\
         M=D\n\
         // THIS = *(endframe - 2)\n\
         @endframe\n\
         D=M\n\
         @2\n\
         D=D-A\n\
         A=D\n\
         D=M\n\
         @THIS\n\
         M=D\n\
         // ARG = *(endframe - 3)\n\
         @endframe\n\
         D=M\n\
         @3\n\
         D=D-A\n\
         A=D\n\
         D=M\n\
         @ARG\n\
         M=D\n\
         // LCL = *(endframe - 4)\n\
         @endframe\n\
         D=M\n\
         @4\n\
         D=D-A\n\
         A=D\n\
         D=M\n\
         @LCL\n\
         M=D\n\
         // goto retaddr\n\
         @retaddr\n\
         A=M\n\
         0;JMP\n") 
}

pub fn bootstrap() -> String {
    format!(
        "@256\n\
         D=A\n\
         @SP\n\
         M=D\n
         {}",
         funcall("Sys.init".to_string(), "0".to_string(), 0))
}
