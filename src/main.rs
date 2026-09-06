use crate::utils::cli::CliArgs;

mod utils;

fn main() {
    let args: CliArgs = utils::cli::parse_args();
    println!("{:?}",args);

    if args.clear_cache{
        println!("Clear cache");
        return;
    }

    let port = match args.port {
        Some(p)=>p,
        None=>{
            eprintln!("Error: --port is required to start server");
            std::process::exit(1);
        }
    };

    let origin = match args.origin {
        Some(o)=>o,
        None=>{
            eprintln!("Error: --origin is required to start server");
            std::process::exit(1);
        }
    };

    if port <= 0{
        eprintln!("Error: --port value invalid");
        std::process::exit(1);
    }

    if !origin.starts_with("http://") && !origin.starts_with("https://") {
        eprintln!("Error: --origin must start with http:// or https://");
        std::process::exit(1);
    }

    println!("start caching proxy at port: {port} for origin:{origin}");
}
