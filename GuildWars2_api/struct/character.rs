use std::thread::local_impl::EagerStorage;
use datetime::Date;

pub struct Character{
    name: String,
    race: String,
    gender: String,
    flags: [String],
    profession: String,
    level: i8,
    guild: String,
    age: i8,
    created: Date,
    death: i64,
    crafting: [Discipline],
    title: i16,
    backstory: [String],
    wvw_abilities: [String],
    equipment: [Equipment],
}
pub struct Discipline{
    name: String,
    rating: i16,
    active: bool,
}
pub struct Equipment{
    id: i32,
    slot: String,
    upgrades: Option<[i32]>,
    skin: Option<i32>,
    stats: Option<[(String, i16)]>,
    binding: String,
    dye: Option<[i32]>,

}