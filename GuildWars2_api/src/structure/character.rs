use serde::{Serialize, Deserialize};
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
    tab: u8,
    name: String,
    is_active: bool,
    equipment: Vec<Equipment>,
    equipment_pvp: EquipmentPvP,
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
    id: u32,
    chat_link: String,
    name: String,
    icon: Option<String>,
    description: Option<String>,
    #[serde(rename = "type")]
    types: String,
    rarity: String,
    level: u8,
    vendor_value: u32,
    default_skin: Option<u32>,
    flags: String,
    game_types: Vec<String>,
    restrictions: Vec<String>,
    upgrades_into : Option<Vec<Upgrades>>,
    upgrades_from: Option<Vec<Upgrades>>,
    details: Option<Vec<Value>>,

}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BuildTab{
    tab: u8,
    is_active: bool,
    build: Build,

}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Build{
    name: String,
    profession: String,
    specializations: Vec<Specializations>,
    skills: Skills,
    aquatic_skills: AquaticSkills,
    legends: Option<Vec<String>>,
    aquatic_legends: Option<Vec<String>>,
    pets: Option<Pet>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Specializations{
    pve: Vec<PvESpecialization>,
    pvp: Vec<PvPSpecialization>,
    wvw: Vec<WvWSpecialization>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PvESpecialization{
    id: u16,
    traits: Vec<u32>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PvPSpecialization{
    id: u16,
    traits: Vec<u32>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WvWSpecialization{
    id: u16,
    traits: Vec<u32>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Skills{
    pve: PvESkills,
    pvp: PvPSkills,
    wvw: WvWSkills,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AquaticSkills{
    pve: PvESkills,
    pvp: PvPSkills,
    wvw: WvWSkills,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PvESkills{
    heal: u32,
    utilities: Vec<u32>,
    elite: u32,
    legends: Vec<String>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PvPSkills{
    heal: u32,
    utilities: Vec<u32>,
    elite: u32,
    legends: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WvWSkills{
    heal: u32,
    utilities: Vec<u32>,
    elite: u32,
    legends: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Pet{
    id: Option<u16>,
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    skills: PetSkill,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PetSkill{
    id: u16,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentPvP{
    amulet: u16,
    rune: u16,
    sigils: Vec<u16>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Training{
    id: u16,
    spent: u32,
    done: bool,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Infusions{
    id: u32,
    flags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Upgrades{
    #[serde(rename = "type")]
    types: String,
    flag: Vec<String>,
    infusion_upgrade_flags: Vec<String>,
    suffix: String,
    infix_upgrade: Option<InfixUpgrade>,
    bonuses: Option<Vec<String>>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfixUpgrade{
    id: u32,
    attributes: Vec<Option<String>>,
    modifier: u32,
    buff: Option<Buff>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Buff{
    skill_id: u32,
    description: Option<String>
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemStats{
    pub id: u32,
    pub name: Option<String>,
    pub attributes: Attributes,
    pub multiplier: Option<f32>,
    pub value: Option<f32>,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Stats{
    id: u16,
    attributes: Attributes
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
pub struct Recipes{
    recipes: Option<Vec<u32>>,
}

