use clap::{Args, Subcommand};
use sqlx::{Connection, Executor, PgConnection};

#[derive(Debug, Args)]
pub struct DatabaseArgs {
    #[command(subcommand)]
    pub command: DatabaseCommand,
}

#[derive(Debug, Subcommand)]
pub enum DatabaseCommand {
    /// Initializes the database, creating a new user and database
    Init(InitDbArgs),
}

#[derive(Debug, Args)]
pub struct InitDbArgs {
    /// Name of the database to create
    #[arg(long, default_value = "address_book")]
    pub db_name: String,

    /// Name of the user to create
    #[arg(long, default_value = "address_admin")]
    pub db_user: String,
}

pub async fn handle_db_command(db_args: DatabaseArgs) -> anyhow::Result<()> {
    match db_args.command {
        DatabaseCommand::Init(args) => {
            println!(
                "Initializing database '{}' with user '{}'",
                args.db_name, args.db_user
            );

            // Use a fixed password as requested
            let password = "admin12345";

            // Connect to the default 'postgres' database to run admin commands
            // This requires a superuser connection string in the .env file
            // e.g., DATABASE_URL="postgres://postgres:password@localhost/postgres"
            let db_url = std::env::var("DATABASE_URL")
                .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set in .env to connect as a superuser."))?;

            let mut conn = PgConnection::connect(&db_url).await?;

            // Create the new user
            conn.execute(
                format!(
                    "CREATE USER IF NOT EXIST {} WITH PASSWORD '{}'",
                    args.db_user, password
                )
                .as_str(),
            )
            .await?;
            println!("User '{}' created.", args.db_user);

            // Create the new database
            conn.execute(format!("CREATE DATABASE {}", args.db_name).as_str())
                .await?;
            println!("Database '{}' created.", args.db_name);

            // Grant privileges
            conn.execute(
                format!(
                    "GRANT ALL PRIVILEGES ON DATABASE {} TO {}",
                    args.db_name, args.db_user
                )
                .as_str(),
            )
            .await?;
            println!(
                "Privileges granted on database '{}' to user '{}'.",
                args.db_name, args.db_user
            );

            println!("\nSetup complete!");
            println!("Please update your .env file with the new application DATABASE_URL:");
            println!(
                "\nDATABASE_URL=\"postgres://{}:{}@localhost/{}\"\n",
                args.db_user, password, args.db_name
            );

            Ok(())
        }
    }
}

