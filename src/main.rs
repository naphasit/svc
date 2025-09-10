use clap::{arg, Arg, ArgAction, ArgGroup, Command};

fn main() {
    let matches = Command::new("My CLI")
        .version("1.0")
        .author("Naphasit")
        .about("Simple version control")
        .subcommand(
            Command::new("commit")
                .about("Adds a new item")
                .arg(arg!(-n --new <NAME> "Create a new commit"))
                .arg(arg!(-g --goto <NAME> "Go to existed commit"))
                .group(ArgGroup::new("commit").args(["new", "goto"]).required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("commit", sub_matches)) => {
            let new = sub_matches.get_one::<String>("new");
            let goto = sub_matches.get_one::<String>("goto");
            if new != None {
                println!("New!")
            }
            else {
                println!("Go to!")
            }
        }
        _ => {}
    }
}
