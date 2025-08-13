use serde::Serialize;
use serde::Deserialize;
use serde_json::Value;



#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Character{
    pub name: String,
    pub race: String,
    pub gender: String,
    pub flags: Vec<Value>,
    pub profession: String,
    pub level: u8,
    pub guild: Option<String>,
    pub age: u64,
    //pub last_modified: String,
    pub created: String,
    pub deaths: u64,
    pub title: Option<u16>,
    pub crafting: Option<Vec<Discipline>>,
    pub backstory:  Vec<String>,
    pub wvw_abilities: Option<Vec<WvWAbilities>>,
    pub equipment: Option<Vec<Equipment>>,
    pub recipes: Vec<u32>,
    pub training: Vec<Value>,
    pub bags: Option<Vec<Option<Bag>>>,
    //pub build_tabs: Vec<BuildTab>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub id: u32,
    pub count: Option<u16>,
    pub slot: String,
    pub infusions: Option<Vec<u32>>,
    pub upgrades: Option<Vec<u32>>,
    pub skin: Option<u32>,
    pub stats: Option<Stats>,
    pub binding: Option<String>,
    pub location: Option<String>,
    //pub tabs: Vec<u8>,
    pub charges: Option<u16>,
    pub bound_to: Option<String>,
    pub dyes: Option<Vec<Value>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentTab{
    pub tab: u8,
    pub name: String,
    pub is_active: bool,
    pub equipment: Vec<Equipment>,
    pub equipment_pvp: EquipmentPvP,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bag {
    pub id: u32,
    pub size: u8,
    pub inventory: Vec<Option<Inventory>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub id: u32,
    pub count: u32,
    pub charges: Option<u16>,
    pub infusions: Option<Vec<u32>>,
    pub upgrades: Option<Vec<u32>>,
    pub skin: Option<u16>,
    pub stats: Option<Stats>,
    pub dyes: Option<Vec<Value>>,
    pub binding: Option<String>,
    pub bound_to: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Items{
    pub id: u32,
    pub chat_link: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub types: String,
    pub rarity: String,
    pub level: u8,
    pub vendor_value: u32,
    pub default_skin: Option<u32>,
    pub flags: String,
    pub game_types: Vec<String>,
    pub restrictions: Vec<String>,
    pub upgrades_into : Option<Vec<Upgrades>>,
    pub upgrades_from: Option<Vec<Upgrades>>,
    pub details: Option<Vec<Value>>,

}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BuildTab{
    pub tab: u8,
    pub is_active: bool,
    pub build: Build,

}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Build{
    pub name: String,
    pub profession: String,
    pub specializations: Vec<Specializations>,
    pub skills: Skills,
    pub aquatic_skills: AquaticSkills,
    pub legends: Option<Vec<String>>,
    pub aquatic_legends: Option<Vec<String>>,
    pub pets: Option<Pet>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Specializations{
    pub pve: Vec<PvESpecialization>,
    pub pvp: Vec<PvPSpecialization>,
    pub wvw: Vec<WvWSpecialization>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PvESpecialization{
    pub id: u16,
    pub traits: Vec<u32>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PvPSpecialization{
    pub id: u16,
    pub traits: Vec<u32>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WvWSpecialization{
    pub id: u16,
    pub traits: Vec<u32>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Skills{
    pub pve: PvESkills,
    pub pvp: PvPSkills,
    pub wvw: WvWSkills,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AquaticSkills{
    pub pve: PvESkills,
    pub pvp: PvPSkills,
    pub wvw: WvWSkills,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PvESkills{
    pub heal: u32,
    pub utilities: Vec<u32>,
    pub elite: u32,
    pub legends: Vec<String>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PvPSkills{
    pub heal: u32,
    pub utilities: Vec<u32>,
    pub elite: u32,
    pub legends: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WvWSkills{
    pub heal: u32,
    pub utilities: Vec<u32>,
    pub elite: u32,
    pub legends: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Pet{
    pub id: Option<u16>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub skills: PetSkill,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PetSkill{
    pub id: u16,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentPvP{
    pub amulet: u16,
    pub rune: u16,
    pub sigils: Vec<u16>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Training{
    pub id: u16,
    pub spent: u32,
    pub done: bool,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Infusions{
    pub id: u32,
    pub flags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Upgrades{
    #[serde(rename = "type")]
    pub types: String,
    pub flag: Vec<String>,
    pub infusion_upgrade_flags: Vec<String>,
    pub suffix: String,
    pub infix_upgrade: Option<InfixUpgrade>,
    pub bonuses: Option<Vec<String>>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfixUpgrade{
    pub id: u32,
    pub attributes: Vec<Option<String>>,
    pub modifier: u32,
    pub buff: Option<Buff>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Buff{
    pub skill_id: u32,
    pub description: Option<String>
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemStats{
    pub id: u16,
    pub name: String,
    pub attributes: Vec<Attributes2>,
    pub multiplier: Option<f32>,
    pub value: Option<f32>,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Stats{
    pub id: u16,
    pub attributes: Attributes
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Discipline{
    pub discipline: String,
    pub rating: u32,
    pub active: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WvWAbilities {
    pub id: u16,
    pub rank: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Eq)]
pub struct Attributes {
    #[serde(rename = "Power")]
    pub power: Option<u16>,
    #[serde(rename = "Precision")]
    pub precision: Option<u16>,
    #[serde(rename = "Toughness")]
    pub toughness: Option<u16>,
    #[serde(rename = "Vitality")]
    pub vitality: Option<u16>,
    #[serde(rename = "ConditionDamage")]
    pub condition_damage: Option<u16>,
    #[serde(rename = "Ferocity")]
    pub ferocity: Option<u16>,
    #[serde(rename = "HealingPower")]
    pub healing_power: Option<u16>,
    #[serde(rename = "Expertise")]
    pub expertise: Option<u16>,
    #[serde(rename = "Concentration")]
    pub concentration: Option<u16>,
    #[serde(rename = "CritDamage")]
    pub critical_damage: Option<u16>,
    #[serde(rename = "ConditionDuration")]
    pub condition_duration: Option<u16>,
    #[serde(rename = "Health")]
    pub health: Option<u16>,
    #[serde(rename = "CriticalChance")]
    pub critical_chance: Option<u16>,
    #[serde(rename = "BoonDuration")]
    pub boon_duration: Option<u16>,
    #[serde(rename = "Armor")]
    pub armor: Option<u16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes2 {
    pub attribute: String,
    pub multiplier: f32,
    pub value: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipes{
    pub recipes: Option<Vec<u32>>,
}