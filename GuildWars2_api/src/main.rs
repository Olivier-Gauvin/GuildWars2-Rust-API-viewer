use tokio::fs::read_to_string;
use crate::api::*;
use crate::api::api_call::*;
use crate::api::api_enums::*;
use crate::api::string_parser::*;


pub mod structure{
    pub mod character;
    pub mod account;
}
pub mod api{
    pub mod api_call;
    pub mod api_enums;
    pub mod string_parser;
    pub mod api_endpoints;
}

pub async fn get_access_token() -> String {
    read_to_string("D:/Programming/RustRoverProjects/access_token.txt")
        .await.expect("file not read!")
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = get_access_token().await;


    //let list_of_character = get_list_of_character(&access_token).await?;
    let character = parse_character_info("Lockonstrat".to_string(), &access_token).await.unwrap();
    //let list_of_parsed_character = get_list_of_parsed_character_info(
    //   list_of_character, &access_token).await;
    let guild_id = character.guild.as_deref();
    let guild_info = get_guild_info(&access_token, guild_id);

    //println!("List of characters: {:?}\n", list_of_parsed_character);
    //println!("List of characters: {:?}\n", character);
    println!("guild info: {:?}\n", guild_info.await?);


    //let ids_vec = Some(string_parser(&api_call(&Items::Skins, &access_token, None).await?));
    //for ids in ids_vec.unwrap() {
    //    let input2 = Items::Skins;
    //    let test2 = api_call(&input2, &access_token,Some(&ids)).await?;
    //    println!("Test info:\n {}\n", test2);
    //}


    //let bank_items = string_parser(
    //    &api(&AccountCall::Bank, &access_token, None).await.unwrap());
    //     serialize_info::<_, account::Bank>(Some(bank_items)).await;


    //let list_itemstats_ids = string_parser(&get_item_stats().await);
    //println!("List of item stats: {:?}\n", &list_itemstats_ids);
    //for stats in &list_itemstats_ids{
    //    serialize_info::<_, character::ItemStats>
    //        (Some(stats), ItemStats, &access_token).await;
    //}


    Ok(())
}
