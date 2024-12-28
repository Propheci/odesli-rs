use clap::{Arg, ArgAction, ArgMatches, Command};
use odesli_rs::{EntityType, OdesliError, Platform};

use super::OdesliSubcommand;

pub struct GetIdSubcommand;

lazy_static::lazy_static! {}

#[async_trait::async_trait]
impl OdesliSubcommand for GetIdSubcommand {
    fn name(&self) -> &'static str {
        "get-id"
    }

    fn get_subcommand(&self) -> Command {
        Command::new(self.name())
            .about("find matches using entity IDs")
            .arg(
                Arg::new("id")
                    .help("The entity ID")
                    .action(ArgAction::Set)
                    .required(true)
                    .value_parser(clap::builder::NonEmptyStringValueParser::new())
                    .num_args(1),
            )
            .arg(
                Arg::new("platform")
                    .help("The Platform from which the entity ID is taken")
                    .action(ArgAction::Set)
                    .required(true)
                    .value_parser(clap::builder::EnumValueParser::<Platform>::new())
                    .num_args(1),
            )
            .arg(
                Arg::new("entity-type")
                    .help("The entity type to search, can be album or song")
                    .action(ArgAction::Set)
                    .required(true)
                    .value_parser(clap::builder::EnumValueParser::<EntityType>::new())
                    .num_args(1),
            )
    }

    async fn handle_subcommand(
        &self,
        cmd_matches: &ArgMatches,
        api_key: Option<String>,
        dump_json: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::utils::build_odesli_client(api_key);

        let entity_id = cmd_matches.get_one::<String>("id").expect("Argument 'id' is required");
        let query_platform =
            cmd_matches.get_one::<Platform>("platform").expect("Argument 'platform' is required");
        let entity_type = cmd_matches
            .get_one::<EntityType>("entity-type")
            .expect("Argument 'entity-type' is required");

        match client.get_by_id(entity_id, query_platform, entity_type).await {
            Ok(result) => {
                if dump_json {
                    let result_json = serde_json::to_string_pretty(&result)
                        .expect("result is already json parsed");
                    println!("{}", result_json);
                } else {
                    println!("Input Details:");
                    println!("  ID: {}", entity_id);
                    println!("  Platform: {:?}", query_platform);
                    println!("  Type: {:?}", entity_type);
                    println!();
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
