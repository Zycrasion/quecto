use std::{collections::HashMap, env::args, fs, path::Path, process::Command, str::FromStr};

use quecto::{
    compiler::Compiler,
    parser::Parser,
    tokeniser::{self, Tokeniser},
};

fn main()
{
    let mut arguments: HashMap<String, Option<String>> = HashMap::new();

    {
        let args_vec = args().collect::<Vec<String>>();
        let mut args_iter = args_vec.into_iter();
        args_iter.next();

        while let Some(arg) = args_iter.next()
        {
            if arg.starts_with("--")
            {
                arguments.insert(arg.clone(), args_iter.next().clone());
            }
            else
            {
                arguments.insert(arg.clone(), None);
            }
        }
    }

    if arguments.contains_key("help")
    {
        help();
        return;
    }

    if let Some(path_string) = arguments.get("--file")
    {
        if let Some(path_string) = path_string
        {
            let path = Path::new(path_string);
            if !path.exists()
            {
                eprintln!("FILE DOESN'T EXIST {path_string}");
                panic!();
            }
            let contents = fs::read_to_string(path).unwrap();
            let out = compile_single_file(
                contents,
                arguments.contains_key("-tokens"),
                arguments.contains_key("-nodes"),
                arguments.contains_key("-asm"),
            )
            .unwrap();

            if let Some(output_file) = arguments.get("--output")
            {
                let output_file = output_file.clone().unwrap();
                let output_file = output_file.trim();
                let path_final = output_file.clone();
                let path_ld = format!("{output_file}.o");
                let path_nasm = format!("{output_file}.nasm");
                fs::write(&path_nasm, out).unwrap();
                Command::new("nasm")
                    .arg(&path_nasm)
                    .arg("-felf64")
                    .arg(format!("-o{}", &path_ld))
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
                Command::new("ld")
                    .arg(&path_ld)
                    .arg(format!("-o{}", &path_final))
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
                Command::new("rm")
                    .arg(path_ld)
                    .arg(path_nasm)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
        }
    }
    else
    {
        println!("INCORRECT USAGE: EXPECTED INPUT FILE");
        help();
    }
}

fn help()
{
    println!("HELP:");
    println!("-help           - Open this");
    println!("--file <file>   - Input File");
    println!("--output <file> - Output to File");
    println!("-tokens         - Print Tokens");
    println!("-nodes          - Print Nodes");
    println!("-asm          - Print Assembly");
    return;
}

fn compile_single_file(
    contents: String,
    print_tokens: bool,
    print_nodes: bool,
    print_asm: bool,
) -> Result<String, ()>
{
    if print_tokens
    {
        let tokeniser = Tokeniser(contents.clone());
        let tokens = tokeniser.tokenise();
        if print_tokens
        {
            println!("{:#?}", tokens);
        }
    }

    let parser = Parser::from_str(contents.as_str()).unwrap();

    if print_nodes
    {
        let nodes = parser.clone().parse();
        println!("{:#?}", nodes);
    }

    let compiler = Compiler(parser);

    if print_asm
    {
        let asm = compiler.clone().compile();
        println!("{asm}");
    }

    Ok(compiler.compile())
}
