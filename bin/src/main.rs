pub mod subcommands;
pub mod utils;

use clap::{Arg, ArgAction, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::new("odesli")
        .about("interact with Odesli API using CLI")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("api-key")
                .short('k')
                .long("api-key")
                .help("The Odesli API key to use, if any")
                .action(ArgAction::Set)
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("Dump the output in JSON as received from API")
                .num_args(0)
                .action(ArgAction::SetTrue)
                .required(false),
        );

    for subcommand in crate::subcommands::SUBCOMMANDS.iter() {
        command = command.subcommand(subcommand.get_subcommand())
    }

    let global_matches = command.clone().get_matches();

    let api_key = if global_matches.contains_id("api-key") {
        Some(global_matches.get_one::<String>("api-key").expect("contains_id").to_string())
    } else {
        None
    };
    let dump_json = global_matches.get_flag("json");

    match global_matches.subcommand() {
        Some((cmd, cmd_matches)) => {
            for subcommand in subcommands::SUBCOMMANDS.iter() {
                if cmd.eq(subcommand.name()) {
                    subcommand
                        .handle_subcommand(&mut command, cmd_matches, api_key, dump_json)
                        .await?;
                    return Ok(());
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
