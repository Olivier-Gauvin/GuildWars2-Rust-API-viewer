use std::string::ToString;
use reqwest::Result;
use std::time::Duration;
use reqwest::ClientBuilder;
use std::fs;
pub mod structure{
    pub mod character;
}
use crate::structure::character::*;

pub async fn get_api(resource: &str) -> Result<String>{
    let request_url =format!( "https://api.guildwars2.com/v2/{}",resource);
    println!("{}\n", request_url);

    let timeout = Duration::new(10, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.get(&request_url).send().await?;
    let content = response.text().await?;
    Ok(content)
}

pub async fn get_access_token() -> String {
    fs::read_to_string("C:/Users/Olivi/RustroverProjects/access_token.txt").expect("file not read!")
}

pub async fn get_characters(access_token: &str) -> Result<Vec<String>> {
    let resource = format!("characters/?access_token={}", access_token);
    let content = get_api(&resource).await?;
    let character_list = string_parser(&content);

    Ok(character_list)
}

pub fn string_parser(characters: &String) -> Vec<String> {
    let mut characters_vec = vec![];
    for character in characters.split(",") {
        let character = character.replace("[", "");
        let character = character.replace("]", "");
        let character = character.replace("\"","");
        let character = character.trim();
        characters_vec.push(character.to_string());
    }
    characters_vec
}

pub async fn get_character_info(character_name: &String, access_token: &str) -> Result<String> {
    let resource = format!("characters/{}/?access_token={}",character_name, access_token);
    let content = get_api(&resource).await?;
    Ok(content)
}

pub async fn parse_character_info(character: String, access_token: &str)-> Result<()>{
    let character_info = get_character_info(&character, &access_token).await.unwrap();
    //println!("Raw JSON: {}\n", &character_info);

    // Attempt to parse it into the updated Character struct
    match serde_json::from_str::<Character>(&character_info) {
        Ok(parsed_character) => println!("{:?}", parsed_character),
        Err(err) => {
            eprintln!("Failed to parse character info: {}\n", err);
        }
    }
    Ok(())
}

pub async fn get_item_stats()-> String{
    get_api("itemstats").await.unwrap()
}

pub async fn get_item_info(item_id: &String)-> Result<String>{
    let resource = format!("itemstats/{}",item_id);
    let content = get_api(&resource).await?;
    Ok(content)
}

pub async fn parse_item_info(item: String)-> Result<()>{
    let item_info = get_item_info(&item).await.unwrap();
    println!("Raw JSON: {}\n", &item_info);

    match serde_json::from_str::<Items>(&item_info) {
        Ok(parse_item) => println!("{:?}", parse_item),
        Err(err) => {
            eprintln!("Failed to parse character info: {}\n", err);
        }
    }
    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    //let item_stats = string_parser(&get_item_stats().await);
    let access_token = get_access_token().await;
    let list_of_character = get_characters(&access_token).await.unwrap();

    //for item in item_stats{
    //    println!("{}", item);
    //    parse_item_info(item).await?;
    //}

    for character in list_of_character{
        println!("{}", character);
        parse_character_info(character, &access_token).await?;
    }





    Ok(())
}
