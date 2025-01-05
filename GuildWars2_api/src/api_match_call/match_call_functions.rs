use reqwest::Result;
use crate::get_api;
use crate::structure::character::*;
use crate::structure::account::*;

pub async fn match_account_api_call(input: &AccountCall, access_token: &str) -> Result<String> {
    async fn get_account_data(path: &str, access_token: &str) -> Result<String> {
        let resource = format!("{path}?access_token={access_token}");
        get_api(&resource).await
    }

    // Map AccountCall to the appropriate endpoint path
    let endpoint = match input {
        AccountCall::Account => "account",
        AccountCall::Achievements => "account/achievements",
        AccountCall::Bank => "account/bank",
        AccountCall::DailyCrafting => "account/dailycrafting",
        AccountCall::Dungeons => "account/dungeons",
        AccountCall::Dyes => "account/dyes",
        AccountCall::Finishers => "account/finishers",
        AccountCall::Gliders => "account/gliders",
        AccountCall::Cats => "account/home/cats",
        AccountCall::Nodes => "account/nodes",
        AccountCall::Decorations => "account/homestead/decorations",
        AccountCall::Glyphs => "account/homestead/glyphs",
        AccountCall::Inventory => "account/inventory",
        AccountCall::Jadebots => "account/jadebots",
        AccountCall::Luck => "account/luck",
        AccountCall::LegendaryArmory => "account/legendaryarmory",
        AccountCall::MailCarriers => "account/mailcarriers",
        AccountCall::MapChests => "account/mapchests",
        AccountCall::Masteries => "account/masteries",
        AccountCall::MasteryPoints => "account/masterypoints",
        AccountCall::Materials => "account/materials",
        AccountCall::Minis => "account/minis",
        AccountCall::MountsSkins => "account/mounts/skins",
        AccountCall::MountsTypes => "account/mounts/types",
        AccountCall::Novelties => "account/novelties",
        AccountCall::Outfits => "account/outfits",
        AccountCall::Progression => "account/progression",
        AccountCall::PvPHeroes => "account/pvp/heroes",
        AccountCall::Raids => "account/raids",
        AccountCall::Recipes => "account/recipes",
        AccountCall::Skiff => "account/skiff",
        AccountCall::Skins => "account/skins",
        AccountCall::Titles => "account/titles",
        AccountCall::Wallet => "account/wallet",
        AccountCall::WizardsvaultDaily => "account/wizardsvault/daily",
        AccountCall::WizardsvaultListings => "account/wizardsvaultlistings",
        AccountCall::WizardsvaultSpecial => "account/wizardsvaultspecial",
        AccountCall::WizardsVaultWeekly => "account/wizardsVaultweekly",
        AccountCall::WorldBosses => "account/worldbosses",
    };

    // Call the helper function with the endpoint
    get_account_data(endpoint, access_token).await
}