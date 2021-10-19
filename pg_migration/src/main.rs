use chrono::Utc;
use clap::{App, AppSettings, Arg, SubCommand};
use std::{fs, io};
use tokio_postgres::{Client, Error, NoTls};

const MIGRATIONS_DIR: &str = "./migrations";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app_matches = App::new("migration")
        .version("1.0")
        .author("Davi Souza <ddas.souza@gmail.com>")
        .about("PostgreSQL migration tool")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("create")
                .about("Create migration file")
                .version("1.0")
                .arg(
                    Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .required(true)
                        .takes_value(true)
                        .help("Filename in snake case (e.g., create_table)"),
                ),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Run pending migrations")
                .version("1.0"),
        )
        .get_matches();

    if app_matches.subcommand.is_none() {
        return Ok(());
    }

    let mut client = connect().await?;

    create_table(&client).await?;

    if let Some(matches) = app_matches.subcommand_matches("create") {
        create_migration_file(matches.value_of("name").unwrap());
        println!("Migration file created");
    } else if let Some(_) = app_matches.subcommand_matches("run") {
        let executed_files = get_executed_filenames(&client).await?;
        let mut local_files = get_local_files();
        local_files.retain(|file| !executed_files.contains(&file));
        for file in local_files {
            execute_file(&mut client, file).await?;
        }
    }
    Ok(())
}

async fn connect() -> Result<Client, Error> {
    let (client, connection) =
        tokio_postgres::connect("postgresql://app-db:app-db@app-db:5432/app-db", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            println!("connection error: {}", e);
        }
    });
    Ok(client)
}

async fn create_table(client: &Client) -> Result<(), Error> {
    client
        .query(
            "create table if not exists db_migration_table (\
            id bigserial primary key, \
            name text not null, \
            created_at timestamp without time zone not null \
        )
        ",
            &[],
        )
        .await?;
    Ok(())
}

async fn get_executed_filenames(client: &Client) -> Result<Vec<String>, Error> {
    Ok(client
        .query(
            "select name \
        from db_migration_table \
        order by created_at asc",
            &[],
        )
        .await?
        .iter()
        .map(|row| row.get::<usize, String>(0))
        .collect())
}

async fn execute_file(client: &mut Client, filename: String) -> Result<(), Error> {
    let tx = client.transaction().await?;
    tx.execute(get_local_file_content(filename.clone()).as_str(), &[])
        .await?;
    tx.execute(
        "insert into db_migration_table ( \
            name, \
            created_at \
        ) values ($1, now())",
        &[&filename],
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

fn create_migration_file(filename: &str) {
    if !is_dir_present() {
        create_migrations_dir();
    }
    let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let path = format!(
        "{}/{}_{}.sql",
        MIGRATIONS_DIR,
        now,
        filename.replace(" ", "_")
    );
    fs::write(path, b"-- Write your migration script here\n")
        .expect("Unable to create migration file");
}

fn is_dir_present() -> bool {
    match fs::read_dir(MIGRATIONS_DIR) {
        Ok(_) => true,
        _ => false,
    }
}

fn create_migrations_dir() {
    fs::create_dir(MIGRATIONS_DIR).expect("Unable to create directory \"./migrations\"");
}

fn get_local_files() -> Vec<String> {
    let mut entries = fs::read_dir("./migrations")
        .expect("The directory \"migrations\" was not found.")
        .map(|res| res.map(|e| e.file_name().into_string().unwrap()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    entries.sort();
    entries
}

fn get_local_file_content(filename: String) -> String {
    fs::read_to_string(format!("{}/{}", MIGRATIONS_DIR, filename)).unwrap()
}
