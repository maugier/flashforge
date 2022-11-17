use anyhow::Result;
use ffctl::{FlashForge, Temperature, Scanner, Temperatures, Status};
use colored::{Colorize, ColoredString};
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

    /// List files in internal storage
    Ls,

    /// Turn the LED on or off
    Led { #[arg(value_enum)] on: OnOff },

    /// Rename the printer
    Rename { name: String }
}

#[derive(Debug,Clone,Copy,ValueEnum,PartialEq,Eq)]
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
            let rgb = match on {
                OnOff::On  => (255,255,255), 
                OnOff::Off => (0,0,0),
            };
            FlashForge::new(address)?.led(rgb)?;
        },
        Commands::Status => {
            let mut machine = FlashForge::new(address)?;
            let status = machine.status()?;
            let temp = machine.temperature()?;

            print_status(&status);
            print_temperatures(&temp);
        },
        Commands::Scan { timeout } => {
            for result in Scanner::scan(timeout)? {
                let result = result?;
                println!("{}\t{}", result.address, result.machine_name)
            }
        },
        Commands::Rename { name } => { FlashForge::new(address)?.rename(&name)? },
        Commands::Ls => {
            todo!()
        },
        

    }

    Ok(())

}

fn colorize(s: &str) -> ColoredString {
    match s {
        "READY" => s.green(),
        "MOVING" => s.bold().yellow(),
        "BUILDING_FROM_SD" => s.bold().yellow(),
        _ => s.into(),
    }
}

fn onoff(x: bool) -> ColoredString {
    if x { "ON".bold().red() } else { "off".blue() }
}

fn print_status(s: &Status) {
    println!("Status: {}", colorize(&s.status));
    println!("  Head: {}", colorize(&s.movemode));
    println!("   LED: {}", onoff(s.led));
    println!(" Stops: X {} / Y {} / Z {}", onoff(s.endstop.x), onoff(s.endstop.y), onoff(s.endstop.z));
    println!("  File: {}", &s.file)
}

fn print_temperatures(t: &Temperatures) {
    if let Some(nozzle) = &t.nozzle { print_temperature("Nozzle", nozzle) }
    if let Some(bed) = &t.bed { print_temperature("Bed", bed) }
}

fn print_temperature(name: &str, t: &Temperature) {
    let target = format!("{}", t.target);
    let target = if t.target == 0 { target.blue() } else { target.red() };
    println!("{:>6}: {:>3}/{} °C", name, t.current, target)
}