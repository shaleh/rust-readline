extern crate readline;

use std::env;
use std::fs;
use std::io;
use std::process::Command;

type Func = fn(&[&str]) -> Result<bool, String>;

struct COMMAND {
   name: &'static str,  /* User printable name of the function. */
   func: Func,          /* Function to call to do the job. */
   expected_args: u32, /* number of expected args */
   doc: &'static str,  /* Documentation for this function.  */
}

fn io_error_to_string(err: io::Error) -> String {
    format!("{}", err)
}

fn com_cd(args: &[&str]) -> Result<bool, String> {
    let path = args[0];

    try!(env::set_current_dir(path).map_err(io_error_to_string));
    Ok(true)
}

fn com_delete(args: &[&str]) -> Result<bool, String> {
    let path = args[0];
    let metadata = try!(fs::metadata(path).map_err(io_error_to_string));

    if metadata.is_dir() {
        try!(fs::remove_dir(path).map_err(io_error_to_string));
    }
    else {
        try!(fs::remove_file(path).map_err(io_error_to_string));
    }

    Ok(true)
}

fn com_help(args: &[&str]) -> Result<bool, String> {
    let item = try!(find_command(args[0]).ok_or("no matching command".to_string()));
    println!("{}", item.doc);
    Ok(true)
}

fn com_list(args: &[&str]) -> Result<bool, String> {
    let path = args[0];

    let iter = try!(fs::read_dir(path).map_err(io_error_to_string));

    for entry in iter {
        let entry = try!(entry.map_err(io_error_to_string));
        println!("{}", entry.path().display());
    }

    Ok(true)
}

fn com_pwd(_: &[&str]) -> Result<bool, String> {
    let cwd = try!(env::current_dir().map_err(io_error_to_string));
    println!("{}", cwd.display());
    Ok(true)
}

fn com_quit(_: &[&str]) -> Result<bool, String> {
    Ok(false)
}

fn com_rename(args: &[&str]) -> Result<bool, String> {
    let src = args[0];
    let dest = args[1];

    println!("Rename {} to {}", src, dest);

    try!(fs::rename(src, dest).map_err(io_error_to_string));

    Ok(true)
}

fn com_stat(args: &[&str]) -> Result<bool, String> {
    let path = args[0];

    let result = try!(fs::metadata(path).map_err(io_error_to_string));

    println!("Is File: {}", result.is_file());
    println!("Size: {}", result.len());

    Ok(true)
}

fn com_view(args: &[&str]) -> Result<bool, String> {
    let path = args[0];

    let status = Command::new("more")
                         .arg(path)
                         .status()
                         .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    if ! status.success() {
        println!("Attempt to view {} failed", path);
    }

    Ok(true)
}

fn com_history(_: &[&str]) -> Result<bool, String> {
    let values: Vec<String> = readline::history();
    println!("{:?}", values);

    Ok(true)
}

static COMMANDS_TABLE: [COMMAND; 12] = [
   COMMAND { name: "cd", func: com_cd,
             expected_args: 1, doc: "Change to directory DIR" },
   COMMAND { name: "delete", func: com_delete,
             expected_args: 1, doc: "Delete FILE" },
   COMMAND { name: "help", func: com_help,
             expected_args: 1, doc: "Display this text" },
   COMMAND { name: "?", func: com_help,
             expected_args: 0, doc: "Synonym for `help'" },
   COMMAND { name: "list",   func: com_list,
             expected_args: 1, doc: "List files in DIR" },
   COMMAND { name: "ls",     func: com_list,
             expected_args: 1, doc: "Synonym for `list'" },
   COMMAND { name: "pwd",    func: com_pwd,
             expected_args: 0, doc: "Print the current working directory" },
   COMMAND { name: "quit",   func: com_quit,
             expected_args: 0, doc: "Quit using Fileman" },
   COMMAND { name: "rename", func: com_rename,
             expected_args: 2, doc: "Rename FILE to NEWNAME" },
   COMMAND { name: "stat",   func: com_stat,
             expected_args: 1, doc: "Print out statistics on FILE" },
   COMMAND { name: "view",   func: com_view,
             expected_args: 1, doc: "View the contents f FILE" },
   COMMAND { name: "history", func: com_history,
             expected_args: 0, doc: "List editline history" },
];

fn main () {

    readline::stifle_history(10);
    if ! readline::history_is_stifled() {
        panic!("Failed to stifle history");
    }

    loop {
        let input = match readline::readline ("FileMan: ") {
            Some(line) => line.trim().to_string(),
            None => {
                break;
            },
        };

        if input.is_empty() {
            continue;
        }

        let command = match readline::history_expand(input.as_ref()) {
            // no expansion, just use the input
            Ok(None) => input.to_string(),
            // expansion found, use it
            Ok(Some(expansion)) => expansion,
            Err(_) => {
                continue;
            },
        };

        readline::add_history(command.as_ref());

        match execute_line(&command) {
            Ok(keep_going) => {
                if ! keep_going {
                    break;
                }
            },
            Err(e) => {
                println!("Failed to execute: {}", e);
            },
        }
    }
}

fn execute_line (line: &String) -> Result<bool,String> {
    let pieces: Vec<_> = line.split(" ").collect();
    let word = pieces[0];

    match find_command (word) {
        Some(command) => {
            let args = &pieces[1..];
            if args.len() as u32 != command.expected_args {
                println!("Error: expected {} given {}", command.expected_args, args.len());
                return Ok(true);
            }
            ((*command).func)(args).or_else(|e| {
                println!("Error: {}", e);
                Ok(true)  // ignore the error and return Ok
            })
        },
        None => {
            return Err(format!("Failed to find: {}", word));
        },
    }
}

fn find_command (name: &str) -> Option<&COMMAND> {
   for i in 0..COMMANDS_TABLE.len() {
        if COMMANDS_TABLE[i].name == name {
            return Some(&COMMANDS_TABLE[i]);
        }
    }

    None
}
