use std::{collections::HashMap, env::args};

fn main()
{
    let mut arguments = HashMap::new();

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
        println!("HELP:");
        println!("-help           - Open this");
        println!("--file <file>   - Input File");
        println!("--output <file> - Output to File");
        println!("-debug          - Output every file");
        return;
    }
}
