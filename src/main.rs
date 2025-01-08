use structopt::StructOpt; //allows for defining and parsing command-line arguments
use regex::Regex; // imports Regex crate for regular expressions
use std::fs::{OpenOptions, File}; // imports OpenOptions and File from the standard library module
use std::io::{self, Write, Read}; //  imports I/O traits enabling writing to and reading from streams respectively
use std::io::stdin;
use chrono::Local; // provides functionality for working with dates and times in the local timezone

#[derive(StructOpt)]
struct Cli {
    // The website URL to restrict
    website_url: Option<String>, // Changed to Option to allow for user input
    // Option to list restricted URLs
    #[structopt(short, long)]
        list: bool, // New field for listing URLs
}

fn main() {
    let cli_arguments = Cli::from_args();

    // If the list option is provided, display the restricted websitess
        if cli_arguments.list {
            list_restricted_websites().expect("Failed to read restricted Websites");
            return; // End statement
        }
    
        let mut website_to_restrict = match cli_arguments.website_url {
            Some(url) => url,
            None => {
                let mut input = String::new();
                println!("Please enter the website URL you want to restrict:");
                stdin().read_line(&mut input).expect("Failed to read website");
                input.trim().to_string() // Trim whitespace and return
            }
    };

    // Loop until a valid URL website is entered
    while !is_valid_url(&website_to_restrict) {
        eprintln!("Error: Enter a valid URL website that begins with 'http://' or 'https://'");
        let mut input = String::new();
        println!("Please enter a valid website URL:");
        stdin().read_line(&mut input).expect("Failed to read website");
        website_to_restrict = input.trim().to_string(); // Update to the new input
    }

    // Confirm before proceeding
    if confirm_restriction(&website_to_restrict) {
        println!("Website {} has been restricted.", website_to_restrict);
        restricted_url(&website_to_restrict).expect("Failed to log restriction");
    } else {
        println!("You Cancelled the Operation. The website was not restricted.");
    }
}

fn is_valid_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^(http|https)://[^\s/$.?#].[^\s]*$").unwrap();
    url_regex.is_match(url)
}

fn confirm_restriction(url: &str) -> bool {
    let mut input = String::new();
    println!("Do you want to proceed with restricting the following website?\n{}", url);
    println!("Type 'yes' to confirm or 'no' to cancel:");

    stdin().read_line(&mut input).expect("Failed to read line");
    let response = input.trim().to_lowercase(); // Trim whitespaces and convert to lowercase

    response == "yes" // Return true if the user confirms operation
}

fn restricted_url(url: &str) -> io::Result<()> {
    // Get the current timestamp
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Open the file in append mode to add new restricted URLs
    let mut file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Append to the file
        .open("restricted_websites.txt")?; // Open the file

    // Write the URL and timestamp to the file
    writeln!(file, "[{}] {}", timestamp, url)?;
    Ok(())
}

fn list_restricted_websites() -> io::Result<()> {
    let file = File::open("restricted_websites.txt");
    
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            if contents.is_empty() {
                println!("No website has been restricted.");
            } else {
                println!("{}", contents);
            }
        }
        Err(_) => {
            println!("No website has been restricted.");
        }
    }
    
    Ok(())
}
