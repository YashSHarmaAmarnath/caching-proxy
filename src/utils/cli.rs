use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "caching-proxy")]
#[command(about = "Starts a caching proxy server or clears the cache")]

pub struct CliArgs {
    #[arg(
        long,
        required_unless_present = "clear_cache",
        conflicts_with = "clear_cache"
    )]
    pub port: Option<u16>,

    #[arg(
        long,
        required_unless_present = "clear_cache",
        conflicts_with = "clear_cache"
    )]
    pub origin: Option<String>,

    #[arg(long)]
    pub clear_cache: bool,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}
