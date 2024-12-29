use clap::{ArgMatches, Command};
use odesli_rs::Platform;
use strum::IntoEnumIterator;

use super::OdesliSubcommand;

pub struct PlatformsSubcommand;

#[async_trait::async_trait]
impl OdesliSubcommand for PlatformsSubcommand {
    fn name(&self) -> &'static str {
        "platforms"
    }

    fn get_subcommand(&self) -> Command {
        Command::new(self.name())
    }

    async fn handle_subcommand(
        &self,
        _final_cmd: &mut Command,
        _cmd_matches: &ArgMatches,
        _api_key: Option<String>,
        _dump_json: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Here are all the platforms supported by Odesli:\n");

        for (num, platform) in Platform::iter().enumerate() {
            println!("{}. {:?}: {}", num + 1, platform, platform.as_str())
        }

        Ok(())
    }
}
