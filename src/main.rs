use std::env;
use std::process;

const MIN_MAC_LENGTH: usize = 12;
const MAX_MAC_LENGTH: usize = 17;
const OUI_LENGTH: usize = 6;

fn get_csv_path() -> Result<String, std::env::VarError> {
    let home_path = env::var("HOME")?; // The ? operator returns the error to the calling function if one occurs
    let csv_path = format!("{}/.local/share/oui/IEEE_OUI.csv", home_path);
    Ok(csv_path) // Ok is a variant of the result enum that is returned on success
}

fn parse_mac(mac: &str) -> Result<String, Box<dyn std::error::Error>> {
    if mac.len() < MIN_MAC_LENGTH || mac.len() > MAX_MAC_LENGTH {
        return Err("Invalid MAC Address.".into());
    }
    let cleaned: String = mac
        .chars()
        .filter(|c| !['-', ':', '.', ' '].contains(c))
        .collect();
    let uppered = cleaned.to_ascii_uppercase();

    let search_term = &uppered[..OUI_LENGTH];
    Ok(search_term.to_string())
}

fn lookup_oui(csv_path: &str, mac: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(csv_path)?;

    for result in rdr.records() {
        let record = result?;
        if record.get(0) == Some(mac) {
            println!("{}", record.get(1).unwrap_or("Unknown vendor."));
            return Ok(());
        }
    }

    println!("No match.");
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("OUI takes a single argument.".into());
    }
    let mac = parse_mac(&args[1])?;
    let csv_path: String = get_csv_path()?;
    lookup_oui(&csv_path, &mac)?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
