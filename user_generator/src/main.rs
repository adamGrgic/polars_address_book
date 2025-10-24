use clap::{Parser,Subcommand};
use serde::{Serialize};
use fake::{Fake, faker::address::en::*, faker::name::en::*, faker::phone_number::en::*,faker::number::en::*};

#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}



#[derive(Debug,Serialize)]
struct Address {
    address1: String,
    address2: String,
    city: String,
    state: String,
    postal_code: String,
    country: String,
    phone_number: String,
}

#[derive(Debug)]
struct User {
    address: Address,
    first_name: String,
    last_name: String,
    age: String,
}

impl User {
    fn new() -> Self {
        Self {
            address: Address{
                address1: StreetName().fake(),
                address2: SecondaryAddress().fake(),
                city: CityName().fake(),
                state: StateAbbr().fake(),
                postal_code: PostCode().fake(),
                country: "United States".to_string(),
                phone_number: PhoneNumber().fake(),
            },
            first_name:FirstName().fake(),
            last_name: LastName().fake(),
            age:Digit().fake(),
        }
    }
}
fn fake_address() -> Address {
    Address {
        address1: StreetName().fake(),
        address2: SecondaryAddress().fake(),
        city: CityName().fake(),
        state: StateAbbr().fake(),
        postal_code: PostCode().fake(),
        country: "United States".to_string(),
        phone_number: PhoneNumber().fake(),
    }
}



#[derive(Subcommand, Debug)]
enum Commands {
    /// start the admin daemon server
    Create {
        // add address collection to a json file
        #[arg(short,long)]
        count: i64,
    },
}



fn main() {
    println!("Running user generator module ..");
    let cli = Cli::parse();

    let n = 10;
    let addresses: Vec<Address> = (0..n).map(|_| fake_address()).collect();

    println!("{:?}", addresses);
    let mut addresses2 = Vec::with_capacity(n);
    match cli.command {
        Commands::Create {count: _} => {
            // println!("Add was chosen for {} times", count);
            // for i in 0 .. count {
            //     println!("creating user {}", i);
            // }
            //
            for _ in 0..n {
            let addr = fake_address();
            // e.g., skip invalid
            if !addr.postal_code.is_empty() {
                addresses2.push(addr);
            }

            println!("{:?}",addresses2);
        }
        }
    }
}
