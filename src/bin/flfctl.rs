use anyhow::Result;
use ffctl::{FlashForge, Temperature, Scanner, Temperatures};
use colored::Colorize;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(about = "Control networked FlashForge 3d printers")]
struct CLI {
    /// Address of the printer to connect
    #[arg(short,long)]          address: Option<String>,
    #[command(subcommand)]      command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Scan the local network with a multicast UDP ping
    Scan { #[arg(short, long, default_value_t = 200)] timeout: u64 },

    /// Get info about the printer (model, name, ...)
    Info,

    /// Check printer status
    Status,

    /// Check nozzle and hotbed temperature
    Temp,

    /// Turn the LED on or off
    Led { #[arg(value_enum)] on: OnOff },

    /// Rename the printer
    Rename { name: String }
}

#[derive(Debug,Clone,Copy,ValueEnum)]
enum OnOff {
    On, Off
}

fn main() -> Result<()> {
    
    let cli = CLI::parse();

    let address = cli.address
                    .or(std::env::var("FLFCTL_ADDRESS").ok())
                    .expect("No address given. Set FLFCTL_ADDRESS or use -a.");

    match cli.command {
        Commands::Info => {
            let mut machine = FlashForge::new(address)?;
            println!("{}", machine.info()?.trim_end());
        },
        Commands::Led { on } => {
            let args = match on {
                OnOff::On  => "r255 g255 b255 F0", 
                OnOff::Off => "r0 g0 b0 F0",
            };
            FlashForge::new(address)?
                .command("M146", args)?;
        },
        Commands::Temp => {
            let mut machine = FlashForge::new(address)?;
            print_temperatures(&machine.temperature()?);
        },
        Commands::Status => {
            let mut machine = FlashForge::new(address)?;
            let status = machine.status()?;
            println!("{:?}", status);
        },
        Commands::Scan { timeout } => {
            for result in Scanner::scan(timeout)? {
                let result = result?;
                println!("{}\t{}", result.address, result.machine_name)
            }
        },
        Commands::Rename { name } => { FlashForge::new(address)?.rename(&name)? }

    }

    Ok(())

}

fn print_temperatures(t: &Temperatures) {
    if let Some(nozzle) = &t.nozzle { print_temperature("nozzle", nozzle) }
    if let Some(bed) = &t.bed { print_temperature("bed", bed) }
}

fn print_temperature(name: &str, t: &Temperature) {
    let target = format!("{}", t.target);
    let target = if t.target == 0 { target.blue() } else { target.red() };
    println!("{:>6}: {:>3}/{} °C", name, t.current, target)
}