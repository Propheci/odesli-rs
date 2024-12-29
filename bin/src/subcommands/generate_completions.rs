use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use clap_complete::{generate, Shell};

use super::OdesliSubcommand;

pub struct GenerateCompletionsSubcommand;

#[async_trait::async_trait]
impl OdesliSubcommand for GenerateCompletionsSubcommand {
    fn name(&self) -> &'static str {
        "generate-completions"
    }

    fn get_subcommand(&self) -> Command {
        Command::new(self.name()).about("generate completions for various shells").arg(
            Arg::new("shell")
                .help("The shell to generate completions for")
                .action(ArgAction::Set)
                .required(true)
                .value_parser(value_parser!(Shell))
                .num_args(1),
        )
    }

    async fn handle_subcommand(
        &self,
        final_cmd: &mut Command,
        cmd_matches: &ArgMatches,
        _api_key: Option<String>,
        _dump_json: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let shell =
            cmd_matches.get_one::<Shell>("shell").copied().expect("Argument 'shell' is required");

        generate(shell, final_cmd, final_cmd.get_name().to_string(), &mut std::io::stdout());

        Ok(())
    }
}
