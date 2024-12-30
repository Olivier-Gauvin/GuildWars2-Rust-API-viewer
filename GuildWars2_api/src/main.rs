use reqwest::Result;
use std::time::Duration;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let access_token = "44D89F7F-C1E7-3E47-A530-052E898809D6DF7D63AF-FFE0-4760-A6F8-F8F6F32AC0E4";
    let request_url = format!("https://api.guildwars2.com/v2/characters/?access_token={}", access_token);
    println!("{}", request_url);

    let timeout = Duration::new(10, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.get(&request_url).send().await?;

    if response.status().is_success() {
        println!("Response Status= {}", &response.status());
    } else {
        println!("Response Status= {}", &response.status());
    }

    let content = &response.text().await?;
    println!("Response content= {content}");


    Ok(())
}
