use std::string::ToString;
use reqwest::Result;
use std::time::Duration;
use reqwest::ClientBuilder;
use std::fs;
pub mod structure{
    pub mod character;
    pub mod account;
}
use crate::structure::character::*;
use crate::structure::account::*;

pub mod api_match_call{
    pub mod api_call_functions;
    pub mod api_call_enums;
}
use crate::api_match_call::*;
use crate::api_match_call::api_call_enums::AccountCall;
use crate::api_match_call::api_call_functions::{api_call, EndpointProvider};

pub async fn get_api(resource: &str) -> Result<String>{
    let request_url =format!( "https://api.guildwars2.com/v2/{}",resource);
    println!("Requested Url:\n{request_url}\n");

    let timeout = Duration::new(10, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.get(&request_url).send().await?;
    let content = response.text().await?;
    Ok(content)
}

pub async fn get_access_token() -> String {
    fs::read_to_string("C:/Users/Olivi/RustroverProjects/access_token.txt")
        .expect("file not read!")
}

pub async fn get_list_of_characters(access_token: &str) -> Result<Vec<String>> {
    let content = api_call(&AccountCall::Characters, &access_token, None).await?;
    let character_list = string_parser(&content);

    Ok(character_list)
}

pub fn string_parser(list: &String) -> Vec<String> {
    //this function retrieve the element of a list in a json string,
    // and return a vector containing the values.
    let mut list_vec = vec![];
    for value in list.split(",") {
        let value = value.replace("[", "");
        let value = value.replace("]", "");
        let value = value.replace("\"","");
        let value = value.trim();
        list_vec.push(value.to_string());
    }
    list_vec
}

pub async fn get_character_info(character_name: &String, access_token: &str) -> Result<String> {
    let resource = format!("characters/{}/?access_token={}",character_name, access_token);
    let content = get_api(&resource).await?;
    Ok(content)
}

pub async fn parse_character_info(character_name: String, access_token: &str)-> Option<Character>{
    //returns the serialized structure of the character.
    let character_info = get_character_info(&character_name, &access_token).await.unwrap();
    //println!("Raw JSON: {}\n", &character_info);
    match serde_json::from_str::<Character>(&character_info) {
        Ok(parsed_character) => { let parsed_character =  parsed_character;
            println!("{:?} has been parsed\n", character_name);
            Some(parsed_character)
        },
        Err(err) => {
            eprintln!("Failed to parse character info: {}\n", err);
            None
        }
    }
}

//this function is essentially to test if all the characters' info are correctly parsed.
//might not be needed for the final app.
pub async fn get_list_of_parsed_character_info(
    list_of_character: Vec<String>, access_token: &str)-> Vec<Character>{
    let mut i = 0;
    let mut character_info_list: Vec<Character> = vec![];
    for character in list_of_character {
        println!("{}", character);
        character_info_list.push(parse_character_info(character, &access_token).await.unwrap());
        println!("this is from the Character structure: {:?}\
        \n-------------------------------------------------\n\n",
                 character_info_list[i].name);
        i += 1;
    }character_info_list

}

pub async fn get_item_stats()-> String{
    get_api("itemstats").await.unwrap()
}

pub async fn get_item_info(item_id: &String)-> Result<String>{
    let resource = format!("itemstats/{}",item_id);
    let content = get_api(&resource).await?;
    Ok(content)
}

pub async fn parse_item_info(item: &String)-> Result<()>{
    let item_info = get_item_info(&item).await.unwrap();
    //println!("Raw JSON: {}\n", &item_info);

    match serde_json::from_str::<ItemStats>(&item_info) {
        //this will need to be change so that it returns the Structure(parsed_item) of the parsed items.
        Ok(parse_item) => println!("Item Stats: {:?}\n", &parse_item),
        Err(err) => {
            eprintln!("Failed to parse character info: {}\n", err);
        }
    }
    Ok(())
}

pub async fn serialize_info<T: EndpointProvider, E: Serializable>(
    item_ids: Option<&str>, input: T,access_token: &str) -> Option<E> {

    let info = api_call(&input,access_token, item_ids);

    match serde_json::from_str::<E>(&info.await.unwrap()) {
        Ok(parsed_structure) => Some(parsed_structure),
        Err(err) => {
            eprintln!("Failed to deserialize: {}", err);
            None
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    //let access_token = get_access_token().await;


    //let list_of_character = get_list_of_characters(&access_token).await?;
    //let list_of_parsed_character = get_list_of_parsed_character_info(
    //   list_of_character, &access_token).await;
    //println!("List of characters: {:?}\n", &list_of_parsed_character);


    //let ids_vec = Some(string_parser(&api_call(&Skins, &access_token, None).await?));

    //for ids in ids_vec.unwrap() {
    //    let input2 = Skins;
    //    let test2 = api_call(&input2, &access_token,Some(&ids)).await?;
    //    println!("Test info:\n {}\n", test2);
    //}


    let list_item_ids = string_parser(&get_item_stats().await);
    for item in &list_item_ids{
        println!("Item: {}\n", &item);
        parse_item_info(item).await?;
    }


    Ok(())
}
