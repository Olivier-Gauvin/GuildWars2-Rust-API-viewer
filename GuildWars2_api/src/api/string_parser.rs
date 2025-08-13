use crate::get_character_info;
use crate::api::api_call::{api_call, get_item_info};
use crate::api::api_endpoints::EndpointProvider;
use crate::structure::account::Serializable;
use crate::structure::character::*;

pub fn string_parser(list: &String) -> Vec<String> {
    //this function retrieves the element of a list in a JSON string
    // and returns a vector containing the values.
    let mut list_vec = vec![];
    for value in list.split(",") {
        let mut value = value.to_string();
        value.retain(|c| !r#""[]"#.contains(c));
        let value = value.trim();
        list_vec.push(value.to_string());
    }
    list_vec
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

//this function is essentially to test if all the characters' info is correctly parsed.
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

pub async fn parse_item_info(item: &String)-> reqwest::Result<()> {
    let item_info = get_item_info(&item).await?;
    println!("Raw JSON: {}\n", &item_info);

    match serde_json::from_str::<ItemStats>(&item_info) {
        //this will need to be change so that it returns the Structure(parsed_item) of the parsed items.
        Ok(parse_item) => println!("Item Stats: {:?}\n", &parse_item),
        Err(err) => {
            eprintln!("Failed to parse character info: {}\n", err);
        }
    }
    Ok(())
}

pub async fn serialize_info<T: EndpointProvider, E: Serializable + std::fmt::Debug>(
    ids: Option<&str>, input: T,access_token: &str) -> Option<E> {

    let info = api_call(&input,access_token, ids).await.unwrap();
    println!("Raw JSON: {}\n", &info);
    let info = info.strip_prefix("[").unwrap().strip_suffix("]").unwrap().to_string();

    match serde_json::from_str::<E>(&info) {
        Ok(parsed_structure) => {
            println!("Item Stats: {:?}\n", &parsed_structure);
            Some(parsed_structure)
        },
        Err(err) => {
            eprintln!("Failed to deserialize: {}", err);
            None
        }
    }
}
