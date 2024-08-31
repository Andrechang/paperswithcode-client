mod client;
mod http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace with your actual token if needed
    let token: Option<String> = Some("".to_string());
    let client: client::PapersWithCodeClient = client::PapersWithCodeClient::new(token, None);

    match client.paper_get().await {
        Ok(paper) => println!("Paper: {:?}", paper),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}