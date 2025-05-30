use clap::{ArgMatches, Command};
use lazy_static::lazy_static;

pub mod generate_completions;
pub mod get_id;
pub mod get_url;
pub mod platforms;

#[async_trait::async_trait]
pub trait OdesliSubcommand: Sync {
    fn name(&self) -> &'static str;
    fn get_subcommand(&self) -> Command;
    async fn handle_subcommand(
        &self,
        final_cmd: &mut Command,
        cmd_matches: &ArgMatches,
        api_key: Option<String>,
        dump_json: bool,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

lazy_static! {
    pub static ref SUBCOMMANDS: [Box<dyn OdesliSubcommand>; 4] = [
        Box::new(get_url::GetUrlSubcommand),
        Box::new(get_id::GetIdSubcommand),
        Box::new(platforms::PlatformsSubcommand),
        Box::new(generate_completions::GenerateCompletionsSubcommand),
    ];
}
