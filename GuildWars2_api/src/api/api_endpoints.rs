use crate::api_enums::*;

pub trait EndpointProvider {
    fn get_endpoint(&self) -> &'static str;
}

impl EndpointProvider for AccountCall {
    fn get_endpoint(&self) -> &'static str {
        match self {
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
            AccountCall::WizardsVaultDaily => "account/wizardsvault/daily",
            AccountCall::WizardsVaultListings => "account/wizardsvaultlistings",
            AccountCall::WizardsVaultSpecial => "account/wizardsvaultspecial",
            AccountCall::WizardsVaultWeekly => "account/wizardsVaultweekly",
            AccountCall::WorldBosses => "account/worldbosses",
            AccountCall::Characters => "characters",
            AccountCall::Trading => "commerce/transactions",
            AccountCall::CreateSubToken => "createsubtoken",
            AccountCall::PvPStats => "pvp/stats",
            AccountCall::PvPGames => "pvp/games",
            AccountCall::PvPStandings => "pvp/standings",
            AccountCall::TokenInfo => "tokeninfo",
        }
    }
}

impl EndpointProvider for DailyReward {
    fn get_endpoint(&self) -> &'static str {
        match self {
            DailyReward::DailyCrafting => "dailycrafting",
            DailyReward::MapChests => "mapchests",
            DailyReward::WorldBosses => "worldbosses",
        }
    }
}
impl EndpointProvider for GameMechanics {
    fn get_endpoint(&self) -> &'static str {
        match self {
            GameMechanics::JadeBots => "jadebots",
            GameMechanics::LegendaryArmory => "legendaryarmory",
            GameMechanics::Masteries => "masteries",
            GameMechanics::Mounts => "mounts",
            GameMechanics::MountSkins => "mounts/skins",
            GameMechanics::MountTypes => "mounts/types",
            GameMechanics::Outfits => "outfits",
            GameMechanics::Pets => "pets",
            GameMechanics::Progression => "progression",
            GameMechanics::Races => "races",
            GameMechanics::Specializations => "specializations",
            GameMechanics::Skiffs => "skiffs",
            GameMechanics::Skills => "skills",
            GameMechanics::Traits => "traits",
            GameMechanics::Legends => "legends",
        }
    }
}

impl EndpointProvider for Guild {
    fn get_endpoint(&self) -> &'static str {
        match self {
            Guild::GuildName => "guild",
            Guild::Emblems => "emblems",
            Guild::Permissions => "guild/permissions",
            Guild::Search => "guild/search",
            Guild::Upgrades => "guild/upgrades"
        }
    }

}

impl EndpointProvider for AuthGuild {
    fn get_endpoint(&self) -> &'static str {
        match self {
            AuthGuild::Log => "guild/:id/log",
            AuthGuild::Members => "guild/:id/members",
            AuthGuild::Ranks => "guild/:id/ranks",
            AuthGuild::Stash => "guild/:id/stash",
            AuthGuild::Storage => "guild/:id/storage",
            AuthGuild::Treasury => "guild/:id/treasury",
            AuthGuild::Teams => "guild/:id/teams",
            AuthGuild::Upgrades => "guild/:id/upgrades",
        }
    }
}

impl EndpointProvider for Home {
    fn get_endpoint(&self) -> &'static str {
        match self {
            Home::Cats => "home/cats",
            Home::Nodes => "home/nodes",
            Home::Decorations => "homestead/decorations",
            Home::Category => "homestead/categories",
            Home::Glyphs => "homestead/glyphs",
        }
    }
}

impl EndpointProvider for Items {
    fn get_endpoint(&self) -> &'static str {
        match self {
            Items::Finishers => "finishers",
            Items::Item => "items",
            Items::ItemStats => "itemstats",
            Items::Materials => "materials",
            Items::PvPAmulets => "pvp/amulets",
            Items::Recipes => "recipes",
            Items::RecipesSearch => "recipes/search",
            Items::Skins => "skins",
        }
    }
}

impl EndpointProvider for Map {
    fn get_endpoint(&self) -> &'static str {
        match self {
            Map::Continents => "continents",
            Map::Maps => "maps",
        }
    }
}

impl EndpointProvider for Miscellaneous {
    fn get_endpoint(&self) -> &'static str {
        match self {
            Miscellaneous::Build => "build",
            Miscellaneous::Colors => "colors",
            Miscellaneous::Currencies => "currencies",
            Miscellaneous::Dungeons => "dungeons",
            Miscellaneous::Files => "files",
            Miscellaneous::Quaggans => "quaggans",
            Miscellaneous::Minis => "minis",
            Miscellaneous::Novelties => "novelties",
            Miscellaneous::Raids => "raids",
            Miscellaneous::Titles => "titles",
            Miscellaneous::Worlds => "worlds",
        }
    }
}

impl EndpointProvider for Story {
    fn get_endpoint(&self) -> &'static str {
        match self {
            Story::Answers => "backstory/answers",
            Story::Questions => "backstory/questions",
            Story::Stories => "backstory/stories",
            Story::Seasons => "backstory/seasons",
            Story::Quests => "backstory/quests",
        }
    }
}

impl EndpointProvider for PvP {
    fn get_endpoint(&self) -> &'static str {
        match self {
            PvP::PVP => "pvp",
            PvP::Ranks => "pvp/ranks",
            PvP::Seasons => "pvp/seasons",
            PvP::Leaderboards => "pvp/seasons/leaderboards",
        }
    }
}

impl EndpointProvider for TradingPost {
    fn get_endpoint(&self) -> &'static str {
        match self {
            TradingPost::Delivery => "commerce/delivery",
            TradingPost::Exchange => "commerce/exchange",
            TradingPost::Coins => "commerce/exchange/coins",
            TradingPost::Gems => "commerce/exchange/gems",
            TradingPost::Listings => "commerce/listings",
            TradingPost::Prices => "commerce/prices",
            TradingPost::Transactions => "commerce/transactions",
        }
    }
}

impl EndpointProvider for WizardsVault {
    fn get_endpoint(&self) -> &'static str {
        match self {
            WizardsVault::Vault => "wizardsvault",
            WizardsVault::Listings => "wizardsvault/listings",
            WizardsVault::Objectives => "wizardsvault/objectives",
        }
    }
}

impl EndpointProvider for WvW {
    fn get_endpoint(&self) -> &'static str {
        match self {
            WvW::WVW => "wvw",
            WvW::Abilities => "wvw/abilities",
            WvW::Matches => "wvw/matches",
            WvW::Objectives => "wvw/objectives",
            WvW::Ranks => "wvw/ranks",
            WvW::Upgrades => "wvw/upgrades",
        }
    }
}
