use std::time::Duration;
use reqwest::{ClientBuilder, Result};
use crate::api::api_enums::*;
use crate::api::string_parser::*;
use crate::api::api_endpoints::*;
use crate::api::api_enums::Guild::GuildName;

pub async fn get_api(resource: &str) -> Result<String> {
    let request_url = format!("https://api.guildwars2.com/v2/{}", resource);
    println!("Requested Url:\n{request_url}\n");

    let timeout = Duration::new(10, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.get(&request_url).send().await?;
    let content = response.text().await?;
    Ok(content)
}
pub async fn api_call<Endpoint: EndpointProvider>(
    input: &Endpoint, access_token: &str, ids: Option<&str>, ) -> Result<String> {

    let endpoint = input.get_endpoint();

    // Build resource based on endpoint patterns
    let resource = match endpoint {
        // Guild base endpoint: append "/{id}" when provided
        "guild" => match ids {
            Some(id) => format!("{}/{}", endpoint, id),
            None => format!("{}?access_token={}", endpoint, access_token),
        },
        // Common item-like endpoints that use ids query parameter
        "items" | "itemstats" | "recipes" | "recipes/search" | "skins" | "finishers" | "pvp/amulets" | "materials" => match ids {
            Some(id_list) => format!("{}?ids={}", endpoint, id_list),
            None => format!("{}?access_token={}", endpoint, access_token),
        },
        // Endpoints with ":id" placeholder: replaced with provided id
        _ if endpoint.contains(":id") => match ids {
            Some(id) => endpoint.replace(":id", id),
            None => format!("{}?access_token={}", endpoint, access_token),
        },
        // Default behavior
        _ => match ids {
            Some(id_list) => format!("{}?ids={}", endpoint, id_list),
            None => format!("{}?access_token={}", endpoint, access_token),
        },
    };

    get_api(&resource).await
}

pub async fn get_list_of_character(access_token: &str)-> Result<Vec<String>>{
    let result = api_call(&AccountCall::Characters, &access_token, None).await?;
    let characters_list = string_parser(&result);

    Ok(characters_list)
}

pub async fn get_character_info(character_name: &String, access_token: &str) -> Result<String> {
    // Character names may contain spaces/special chars; encode them
    let encoded_name = urlencoding::encode(character_name);
    let endpoint = format!("characters/{}?access_token={}", encoded_name, access_token);
    let content = get_api(&endpoint).await?;
    Ok(content)
}

pub async fn get_item_stats()-> String{
    get_api("itemstats").await.unwrap()
}

pub async fn get_item_info(item_id: &String)-> Result<String>{
    let resource = format!("itemstats/{}",item_id);
    let content = get_api(&resource).await?;
    Ok(content)
}

pub async fn get_guild_info(access_token: &str, guild_id: Option<&str>, ) -> Result<String> {
    let endpoint = GuildName;
    api_call(&endpoint,access_token,guild_id).await
}