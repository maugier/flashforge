use anyhow::Result;
use ffctl::{FlashForge, Temperature};
use colored::Colorize;


fn main() -> Result<()> {
    
    let address = std::env::var("FLASHFORGE_ADDRESS").unwrap_or_else(|_| "192.168.1.25:8899".to_owned());

    let mut machine = FlashForge::new(address)?;

    let temp = machine.temperature()?;

    if let Some(nozzle) = temp.nozzle {
        print_temperature("nozzle", &nozzle);
    }

    if let Some(bed) = temp.bed {
        print_temperature("bed", &bed);
    }


    Ok(())

}

fn print_temperature(name: &str, t: &Temperature) {
    let current = format!("{}", t.current);
    let current = if t.current == 0 { current.blue() } else { current.red() };
    println!("{}: {}/{}", name, current, t.target)
}