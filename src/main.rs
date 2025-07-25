
use std::env;
use std::process;

const FILE_NAME: &str = "assets/IEEE_OUI.csv";
const MIN_MAC_LENGTH: usize = 12;
const MAX_MAC_LENGTH: usize = 17;
const OUI_LENGTH: usize = 6;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("OUI only takes a single argument.\n");
        process::exit(1);
    }
    
    let mac = &args[1];

    if mac.chars().count() < MIN_MAC_LENGTH || mac.chars().count() > MAX_MAC_LENGTH {
        println!("Invalid MAC Address.\n");
        process::exit(1);
    }

    let no_spaces = mac.replace(":", "");
    let uppered = no_spaces.to_ascii_uppercase();
    let search_term = &uppered[..OUI_LENGTH];

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(FILE_NAME)?;
    for result in rdr.records() {
        let record = result?;
        if record.get(0) == Some(search_term) {
            println!("{}", record.get(1).unwrap_or("Unknown"));
            process::exit(0);
        }
    }

    println!("No match.");
    Ok(())
}
