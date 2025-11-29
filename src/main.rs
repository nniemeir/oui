/**
 * main.rs
 * 
 * OUI (Organizationally Unique Identifier) lookup utility.
 * 
 * OVERVIEW:
 * Matches MAC addresses to network interface card (NIC) manufacturers using 
 * the IEEE OUI database. The first 6 hexadecimal digits of a MAC address
 * identify the manufacturer. 
 */

use std::env;
use std::process;

/** 
 * MAC address validation constants.
 * usize is Rust's unsigned integer type.
 */
const MIN_MAC_LENGTH: usize = 12; /* Minimum length without separators */
const MAX_MAC_LENGTH: usize = 17; /* Maximum length with separators */
const OUI_LENGTH: usize = 6; /* OUI is first 6 hex digits */

/**
 * get_csv_path - Construct the path to the IEEE OUI database CSV file
 * 
 * RESULT TYPE:
 * Rust uses Result<Type, Error> for operations that can fail
 * - Ok(value) indicates success and contains the value
 * - Err(error) indicates failure and contains the error
 * 
 * ? OPERATOR:
 * This is shorthand for error propagation. When used after a Result:
 * - If Ok: unwrap the value and continue
 * - If Err: return the error to the calling function immediately
 * 
 * Return: Result containing the CSV path string, or an error if HOME is not set
 */
fn get_csv_path() -> Result<String, std::env::VarError> {
    let home_path = env::var("HOME")?; // Returns the error to the calling function if HOME is not set
    let csv_path = format!("{}/.local/share/oui/IEEE_OUI.csv", home_path);
    Ok(csv_path) // Ok wraps the successful result
}

/**
 * parse_mac - Clean and validate a MAC address, extracting the OUI portion
 * @mac: MAC address string in any common format
 * 
 * RUST STRINGS:
 * &str is a string slice (a reference to a string), while String is an owned string.
 * String can be modified, &str cannot. We accept &str because we only need to read it.
 * 
 * ITERATOR CHAINS:
 * Rust's iterator methods can be chained to transform data efficiently:
 * - chars() - convert string to iterator of characters
 * - filter() - keep only characters that aren't separators
 * - collect() - gather filtered characters back into a String
 * 
 * INTO() METHOD:
 * into() converts one type into another. Here, we're converting a string literal into
 * Box<dyn std::error::Error>, which is a trait object that can hold any error type.
 * 
 * Return: Result containing the 6-character OUI string, or an error for invalid input
 */
fn parse_mac(mac: &str) -> Result<String, Box<dyn std::error::Error>> {
    /* Validate length (must be 12-17 characters) */
    if mac.len() < MIN_MAC_LENGTH || mac.len() > MAX_MAC_LENGTH {
        return Err("Invalid MAC Address.".into());
    }

    /*
     * Remove all common separators using an iterator chain
     * chars() creates an iterator over each character
     * filter() keeps only characters not in the separator list
     * collect() gathers the filtered characters into a String
     */
    let cleaned: String = mac
        .chars()
        .filter(|c| !['-', ':', '.', ' '].contains(c))
        .collect();

    /* Convert result to uppercase for consistent matching with database */
    let uppered = cleaned.to_ascii_uppercase();

    /* Extract first 6 characters */
    let search_term = &uppered[..OUI_LENGTH];
    Ok(search_term.to_string())
}

/*
 * lookup_oui - Search the IEEE OUI database for a matching manufacturer
 * @csv_path: Path to the IEEE OUI CSV file
 * @mac: The 6-character OUI to search for
 * 
 * CSV FORMAT
 * The database is a semicolon-delimited CSV file:
 * Column 0: OUI
 * Column 1: Manufacturer name
 * 
 * Return: Result indicating success or failure
 */
fn lookup_oui(csv_path: &str, mac: &str) -> Result<(), Box<dyn std::error::Error>> {
    /*
     * Create a CSV reader with customer delimiter
     * b';' is a byte literal (semicolon as u8)
     */
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(csv_path)?;

    /* 
     * Iterate through each record in the CSV
     * records() returns an iterator over Result<StringRecord, Error>
     */
    for result in rdr.records() {
        let record = result?; // Propagate any read errors
        if record.get(0) == Some(mac) {
            /* Print manufacturer name (second column).
             * unwrap_or provides a default if column doesn't exist
             */
            println!("{}", record.get(1).unwrap_or("Unknown vendor."));
            return Ok(());
        }
    }

    /* No match found after searching entire database */
    println!("No match.");
    Ok(())
}

/*
 * run - Main application logic with error handling
 * 
 * RUST OWNERSHIP:
 * Rust's ownership system ensures memory safety without garbage collection:
 * - Each value has a single owner
 * - When the owner goes out of scope, the value is dropped
 * - Value can be borrowed (referenced) without transferring ownership
 * 
 * COLLECT() METHOD:
 * env::args() returns an iterator over command-line arguments. collect() 
 * gathers them into a Vec<String> (a growable array of owned strings).
 * 
 * Return: Result indicating success or failure of the entire operation
 */
fn run() -> Result<(), Box<dyn std::error::Error>> {
    /* Collect command-line arguments into a vector
     * args[0] is the program name, args[1] is the first actual argument
     */
    let args: Vec<String> = env::args().collect();

    /* Validate argument count (program takes exactly one argument) */
    if args.len() != 2 {
        return Err("OUI takes a single argument.".into());
    }

    /* Parse and validate MAC address, extracting the OUI */
    let mac = parse_mac(&args[1])?;

    /* Get the path to the OUI database */
    let csv_path: String = get_csv_path()?;

    /* Find the manufacturer name from the OUI */
    lookup_oui(&csv_path, &mac)?;

    Ok(())
}

/*
 * main - Program entry point
 * 
 * IF LET SYNTAX: 
 * "if let" is a more consise alternative to match when you only care about
 * one case. Here, we only care if an error is returned.
 * 
 * ERROR OUTPUT:
 * eprintln! prints to stderr instead of stdout, equivalent to running
 *  
 */
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
