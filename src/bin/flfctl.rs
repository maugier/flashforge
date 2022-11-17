use anyhow::Result;
use ffctl::{FlashForge, Temperature, Scanner};
use colored::Colorize;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct CLI {
    #[arg(short,long)]          address: Option<String>,
    #[command(subcommand)]      command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Scan { #[arg(short, long, default_value_t = 200)] timeout: u64 },
    Info,

    Status,
    Temp,

    Led{ on: bool },
}


fn main() -> Result<()> {
    
    let cli = CLI::parse();

    let address = cli.address
                    .or(std::env::var("FLFCTL_ADDRESS").ok())
                    .expect("No address given. Set FLFCTL_ADDRESS or use -a.");

    let mut machine = FlashForge::new(address)?;

    match cli.command {
        Commands::Info => {
            println!("{}", machine.info()?.trim_end());
        },
        Commands::Led { on } => todo!(),
        Commands::Temp => {
            let temp = machine.temperature()?;

            if let Some(nozzle) = temp.nozzle {
                print_temperature("nozzle", &nozzle);
            }
        
            if let Some(bed) = temp.bed {
                print_temperature("bed", &bed);
            }
        },
        Commands::Status => todo!(),
        Commands::Scan { timeout }=> {
            for result in Scanner::scan(timeout)? {
                let result = result?;
                println!("{}\t{}", result.address, result.machine_name)
            }
        },
    }

    Ok(())

}

fn print_temperature(name: &str, t: &Temperature) {
    let target = format!("{}", t.target);
    let target = if t.target == 0 { target.blue() } else { target.red() };
    println!("{:>6}: {:>3}/{} °C", name, t.current, target)
}