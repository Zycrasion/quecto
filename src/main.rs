use std::{collections::HashMap, env::args, fs, path::Path, str::FromStr};

use quecto::{
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
            }
            let contents = fs::read_to_string(path).unwrap();
            compile_single_file(
                contents,
                arguments.contains_key("-tokens"),
                arguments.contains_key("-nodes"),
            )
            .unwrap()
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
    return;
}

fn compile_single_file(contents: String, print_tokens: bool, print_nodes: bool) -> Result<(), ()>
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
    let nodes = parser.parse();

    if print_nodes
    {
        println!("{:#?}", nodes);
    }

    Ok(())
}
