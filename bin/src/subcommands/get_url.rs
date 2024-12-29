use clap::{Arg, ArgAction, ArgMatches, Command};
use odesli_rs::OdesliError;

use super::OdesliSubcommand;

pub struct GetUrlSubcommand;

#[async_trait::async_trait]
impl OdesliSubcommand for GetUrlSubcommand {
    fn name(&self) -> &'static str {
        "get-url"
    }

    fn get_subcommand(&self) -> Command {
        Command::new(self.name()).about("find matches using a URL").arg(
            Arg::new("url")
                .help("The URL to search")
                .action(ArgAction::Set)
                .required(true)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .num_args(1),
        )
    }

    async fn handle_subcommand(
        &self,
        _final_cmd: &mut Command,
        cmd_matches: &ArgMatches,
        api_key: Option<String>,
        dump_json: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::utils::build_odesli_client(api_key);
        let url = cmd_matches.get_one::<String>("url").expect("Argument 'url' is required");

        match client.get_by_url(url).await {
            Ok(result) => {
                if dump_json {
                    let result_json = serde_json::to_string_pretty(&result)
                        .expect("result is already json parsed");
                    println!("{}", result_json);
                } else {
                    println!("Input URL: {url}");
                    crate::utils::pretty_print_api_result(&result);
                }
            }
            Err(error) => {
                if dump_json {
                    eprintln!("Failed to get results: {}", error);
                    match error {
                        OdesliError::ParseError { body, .. }
                        | OdesliError::Non200StatusCode { body, .. } => {
                            println!("{}", body);
                        }
                        _ => {}
                    }
                } else {
                    println!("Failed to get results: {}", error);
                }
            }
        };

        Ok(())
    }
}
