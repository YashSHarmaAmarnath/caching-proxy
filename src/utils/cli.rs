use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "caching-proxy")]
#[command(about = "Starts a caching proxy server or clears the cache")]

pub struct CliArgs{
    #[arg(long)]
    pub port: Option<u16>,
    
    #[arg(long)]
    pub origin: Option<String>,

    #[arg(long, default_value_t=false)]
    pub clear_cache: bool  
}

pub fn parse_args()->CliArgs{
    CliArgs::parse()
}