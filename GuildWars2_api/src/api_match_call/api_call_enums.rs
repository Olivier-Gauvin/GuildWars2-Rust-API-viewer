
pub enum AccountCall{
    Account,
    Achievements,
    Bank,
    DailyCrafting,
    Dungeons,
    Dyes,
    Finishers,
    Gliders,
    Cats,
    Nodes,
    Decorations,
    Glyphs,
    //shared inventory space
    Inventory,
    Jadebots,
    Luck,
    LegendaryArmory,
    MailCarriers,
    MapChests,
    Masteries,
    MasteryPoints,
    Materials,
    Minis,
    MountsSkins,
    MountsTypes,
    Novelties,
    Outfits,
    Progression,
    PvPHeroes,
    Raids,
    Recipes,
    Skiff,
    Skins,
    Titles,
    Wallet,
    WizardsVaultDaily,
    WizardsVaultListings,
    WizardsVaultSpecial,
    WizardsVaultWeekly,
    WorldBosses,
    Characters,
    Trading,
    CreateSubToken,
    PvPStats,
    PvPGames,
    PvPStandings,
    TokenInfo,
}

pub enum DailyReward{
    DailyCrafting,
    MapChests,
    WorldBosses,
}

pub enum GameMechanics{
    JadeBots,
    LegendaryArmory,
    Masteries,
    Mounts,
    MountSkins,
    MountTypes,
    Outfits,
    Pets,
    Progression,
    Races,
    Specializations,
    Skiffs,
    Skills,
    Traits,
    Legends,
}

pub enum Guild{
    Guild,
    Emblems,
    Permission,
    GuildSearch,
    Upgrades,
}

pub enum AuthGuild{
    Log,
    Members,
    Ranks,
    Stash,
    Storage,
    Treasury,
    Teams,
    Upgrades,
}

pub enum Home{
    Cats,
    Nodes,
    Decorations,
    Category,
    Glyphs,
}

pub enum Items{
    Finishers,
    Item,
    ItemStats,
    Materials,
    PvPAmulets,
    Recipes,
    RecipesSearch,
    Skins,
}

pub enum Map{
    Continents,
    Maps,
}

pub enum Miscellaneous {
    Build,
    Colors,
    Currencies,
    Dungeons,
    Files,
    Quaggans,
    Minis,
    Novelties,
    Raids,
    Titles,
    Worlds,
}

pub enum Story{
    Answers,
    Questions,
    Stories,
    Seasons,
    Quests,
}

pub enum PvP{
    PVP,
    Ranks,
    Seasons,
    Leaderboards,
}

pub enum TradingPost{
    Delivery,
    Exchange,
    Coins,
    Gems,
    Listings,
    Prices,
    Transactions,
}

pub enum WizardsVault{
    Vault,
    Listings,
    Objectives,
}

pub enum WvW{
    WVW,
    Abilities,
    Matches,
    Objectives,
    Ranks,
    Upgrades,
}