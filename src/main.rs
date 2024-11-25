use rand::seq::SliceRandom; // for shuffling
use std::error::Error;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::path::Path;
use std::process;
use rusoto_core::{Region, RusotoError};
use rusoto_ses::{Content, Destination, Message, SendEmailRequest, Ses, SesClient};

// Assuming a structure for Participant
#[derive(Clone, Deserialize)]
struct Participant {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    if let Err(e) = handle().await {
        eprintln!("An error occurred: {}", e);
    }
}

async fn handle() -> Result<(), Box<dyn Error>> {

    load_aws_credentials();

    // Assuming a function `get_all_participants` to fetch participants from the database
    let mut people = get_all_participants()?;
    
    // Shuffling participants
    let mut rng = rand::thread_rng();
    people.shuffle(&mut rng);

    // Pairing participants and sending emails
    for i in 0..people.len() {
        let person = &people[i];
        let other_person = &people[(i + 1) % people.len()];


        if let Err(e) = send_secret_santa_email(&person, &other_person).await {
            eprintln!("Failed sending a mail to {}: {}", person.name, e);
        }
    }

    Ok(())
}

fn get_all_participants() -> Result<Vec<Participant>, Box<dyn Error>> {
    // Open the file
    let file = File::open("participants.csv")?;

    // Create a CSV reader and iterate over each record
    let mut rdr = csv::Reader::from_reader(file);
    let mut participants = Vec::new();

    for result in rdr.deserialize() {
        let record: Participant = result?;
        participants.push(record);
    }

    Ok(participants)
}

async fn send_secret_santa_email(gift_sender: &Participant, gift_recipient: &Participant) -> Result<(), Box<dyn Error>> {
    let client = SesClient::new(Region::EuCentral1); // Choose appropriate AWS region

    let subject = "Your Secret Santa Match!";
    let body_text = format!("Ciao brutto maiale {}, quest'anno il regalo ti tocca farlo a quello stronzo di {}.", gift_sender.name, gift_recipient.name);

    println!("{} -> {}", gift_sender.name, gift_recipient.name);

    let request = SendEmailRequest {
        destination: Destination {
            to_addresses: Some(vec![gift_sender.email.clone()]),
            ..Default::default()
        },
        message: Message {
            body: rusoto_ses::Body {
                text: Some(Content {
                    charset: Some("UTF-8".to_owned()),
                    data: body_text,
                }),
                ..Default::default()
            },
            subject: Content {
                charset: Some("UTF-8".to_owned()),
                data: subject.to_string(),
            },
        },
        source: "noreply@pietrobongiovanni.com".to_string(), // Replace with your SES verified email address
        ..Default::default()
    };

    match client.send_email(request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e))
    }
}

fn load_aws_credentials() {
        // Get the current directory
        let current_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("Failed to get current directory: {}", e);
                process::exit(1);
            }
        };
    
        // Construct the path to the credentials file in the current directory
        let credentials_file = current_dir.join(".aws_credentials");
    
        // Check if the credentials file exists
        if !Path::new(&credentials_file).exists() {
            eprintln!("Credentials file not found in the current directory.");
            process::exit(1);
        }
    
        // Set the environment variable to the path of the credentials file
        env::set_var("AWS_SHARED_CREDENTIALS_FILE", credentials_file.to_str().unwrap());
}