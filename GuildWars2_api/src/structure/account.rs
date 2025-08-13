use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::structure::character::*;
use serde::de::DeserializeOwned;

pub trait Serializable: DeserializeOwned + Send {}
impl<T> Serializable for T where T: DeserializeOwned + Send {}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Account{
    pub achievements: Option<Vec<Achievement>>,
    pub bank: Option<Vec<Bank>>,
    pub dailycrafting: Option<Vec<String>>,
    pub dungeons: Option<Vec<String>>,
    pub dyes: Option<Vec<u16>>,
    pub finishers: Option<Vec<Finishers>>,
    pub gliders: Option<Vec<Option<u32>>>,
    #[serde(rename = "home/cats")]
    pub home_cats: Option<Vec<Cats>>,
    #[serde(rename = "home/nodes")]
    pub home_nodes: Option<Vec<String>>,
    #[serde(rename = "homesteads/decorations")]
    pub homesteads_decorations: Option<Vec<Decorations>>,
    #[serde(rename = "homesteads/cats")]
    pub homesteads_glyphs: Option<Vec<String>>,
    pub inventory: Option<Vec<ItemSlot>>,
    pub jadebots: Option<Vec<Option<u32>>>,
    pub luck: Vec<Option<Luck>>,
    pub legendaryarmory: Option<Vec<LegendaryArmor>>,
    pub mailcarriers: Option<Vec<Option<u32>>>,
    pub mapchests: Vec<Option<String>>,
    pub masteries: Vec<Option<Mastery>>,
    #[serde(rename = "mastery/points")]
    pub mastery_points: Option<MasteryPoint>,
    pub materials: Vec<Material>,
    pub minis: Option<Vec<Option<u32>>>,
    #[serde(rename = "mounts/skins")]
    pub mounts_skins: Option<Vec<u32>>,
    #[serde(rename = "mounts/types")]
    pub mounts_types: Option<Vec<String>>,
    pub novelties: Vec<Option<u32>>,
    pub outfits: Option<Vec<Option<u32>>>,
    pub progression: Vec<Option<Progression>>,
    #[serde(rename = "pvp/heroes")]
    pub pvp: Vec<Option<u32>>,
    pub raids: Vec<Option<String>>,
    pub recipes: Vec<Option<u32>>,
    pub skiff: Vec<Option<u32>>,
    pub skins: Vec<Option<u32>>,
    pub titles: Vec<Option<u32>>,
    pub wallet: Vec<Option<Currency>>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Achievement{
    pub id: u32,
    pub bits: Option<Vec<u32>>,
    pub current: Option<u8>,
    pub max: Option<u32>,
    pub done: bool,
    pub repeated: Option<u32>,
    pub unlocked: Option<bool>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Bank{
    pub id: u32,
    pub count: u16,
    pub charge: Option<u16>,
    pub skin: Option<u32>,
    pub dyes: Option<Vec<Value>>,
    pub upgrades: Option<Vec<u32>>,
    pub upgrade_slot_indices: Option<Vec<u32>>,
    pub infusions: Option<Vec<u32>>,
    pub binding: Option<String>,
    pub bound_to: Option<String>,
    pub stats: Option<Stats>,
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Finishers{
    pub id: u32,
    pub permanent: bool,
    pub quantity: Option<u16>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Cats{
    id: u16,
    hint: Option<String>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Decorations{
    id: u16,
    count: u16,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LegendaryArmor{
    id: u16,
    count: u16,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Mastery{
    id: u16,
    level: u16,
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ItemSlot{
    id: u16,
    count: u8,
    charges: Option<u16>,
    skin: Option<u32>,
    upgrades: Option<Vec<u32>>,
    infusions: Option<Vec<u32>>,
    binding: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Luck{
    id: String,
    value: u16,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MasteryPoint{
    totals: Vec<Masteries>,
    unlocked: Vec<u16>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Masteries{
    region: String,
    spent: u16,
    earned: u16,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Material{
    id: u32,
    category: u32,
    bindings: Option<Vec<String>>,
    count: u16,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Progression{
    id: String,
    value: u16,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Currency{
    id: u16,
    value: u32,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GuildInfo{
    level : u16,
    motd : String,
    influence : u32,
    aetherium : u16,
    resonance : u16,
    favor : u16,
    id: String,
    name: String,
    tag: String,
    emblem: Vec<Emblem>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Emblem{
    background_color: Vec<BackgroundColor>,
    foreground_color: Vec<ForegroundColor>,
    flags: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundColor{
    id: u16,
    colors: Vec<u16>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ForegroundColor{
    id: u16,
    colors: Vec<u16>,
}