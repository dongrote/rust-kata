use azure_core::Error;
use tokio;
use dotenv::dotenv;
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("missing command line argument");
        return;
    }
    let blob_name = &args[1];
    let account = env::var("STORAGE_ACCOUNT").expect("Missing STORAGE_ACCOUNT env var");
    let access_key = env::var("STORAGE_ACCESS_KEY").expect("Missing STORAGE_ACCESS_KEY env var");
    let container = env::var("STORAGE_CONTAINER").expect("Missing STORAGE_CONTAINER env var");

    let sas_token = StorageCredentials::sas_token(&access_key).expect("sas_token error");
    let blob_client = ClientBuilder::new(account, sas_token).blob_client(&container, blob_name);

    match ensure_blob_exists(&blob_client, blob_name).await {
        Ok(_) => print_blob_contents(&blob_client).await,
        Err(err) => eprintln!("{}", err),
    }
}

async fn ensure_blob_exists(client: &BlobClient, name: &str) -> Result<(), Error> {
    match client.exists().await {
        Ok(exists) => {
            if !exists {
                match client.put_block_blob(format!("this is some data for {}", name)).content_type("text/plain").await {
                    Ok(_) => {},
                    Err(err) => eprintln!("error create block blob '{}': {}", name, err),
                }
            } else {
                println!("blob '{}' exists", name);
            }

            Ok(())
        },
        Err(err) => Err(err),
    }
}

async fn print_blob_contents(client: &BlobClient) {
    match client.get_content().await {
        Ok(response) => print_u8_as_utf8(response),
        Err(err) => eprintln!("client.get_content(): {}", err),
    }
}

fn print_u8_as_utf8(data: Vec<u8>) {
    match String::from_utf8(data) {
        Ok(string) => println!("{}", string),
        Err(_) => (),
    }
}
