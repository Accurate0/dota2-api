use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dota2Match {
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub players: Vec<Player>,
    #[serde(rename = "radiant_win")]
    pub radiant_win: bool,
    pub duration: i64,
    #[serde(rename = "pre_game_duration")]
    pub pre_game_duration: i64,
    #[serde(rename = "start_time")]
    pub start_time: i64,
    #[serde(rename = "match_id")]
    pub match_id: i64,
    #[serde(rename = "match_seq_num")]
    pub match_seq_num: i64,
    #[serde(rename = "tower_status_radiant")]
    pub tower_status_radiant: i64,
    #[serde(rename = "tower_status_dire")]
    pub tower_status_dire: i64,
    #[serde(rename = "barracks_status_radiant")]
    pub barracks_status_radiant: i64,
    #[serde(rename = "barracks_status_dire")]
    pub barracks_status_dire: i64,
    pub cluster: i64,
    #[serde(rename = "first_blood_time")]
    pub first_blood_time: i64,
    #[serde(rename = "lobby_type")]
    pub lobby_type: i64,
    #[serde(rename = "human_players")]
    pub human_players: i64,
    pub leagueid: i64,
    #[serde(rename = "positive_votes")]
    pub positive_votes: i64,
    #[serde(rename = "negative_votes")]
    pub negative_votes: i64,
    #[serde(rename = "game_mode")]
    pub game_mode: i64,
    pub flags: i64,
    pub engine: i64,
    #[serde(rename = "radiant_score")]
    pub radiant_score: i64,
    #[serde(rename = "dire_score")]
    pub dire_score: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(rename = "account_id")]
    pub account_id: i64,
    #[serde(rename = "player_slot")]
    pub player_slot: i64,
    #[serde(rename = "team_number")]
    pub team_number: i64,
    #[serde(rename = "team_slot")]
    pub team_slot: i64,
    #[serde(rename = "hero_id")]
    pub hero_id: i64,
    #[serde(rename = "item_0")]
    pub item_0: i64,
    #[serde(rename = "item_1")]
    pub item_1: i64,
    #[serde(rename = "item_2")]
    pub item_2: i64,
    #[serde(rename = "item_3")]
    pub item_3: i64,
    #[serde(rename = "item_4")]
    pub item_4: i64,
    #[serde(rename = "item_5")]
    pub item_5: i64,
    #[serde(rename = "backpack_0")]
    pub backpack_0: i64,
    #[serde(rename = "backpack_1")]
    pub backpack_1: i64,
    #[serde(rename = "backpack_2")]
    pub backpack_2: i64,
    #[serde(rename = "item_neutral")]
    pub item_neutral: i64,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    #[serde(rename = "leaver_status")]
    pub leaver_status: i64,
    #[serde(rename = "last_hits")]
    pub last_hits: i64,
    pub denies: i64,
    #[serde(rename = "gold_per_min")]
    pub gold_per_min: i64,
    #[serde(rename = "xp_per_min")]
    pub xp_per_min: i64,
    pub level: i64,
    #[serde(rename = "net_worth")]
    pub net_worth: i64,
    #[serde(rename = "aghanims_scepter")]
    pub aghanims_scepter: i64,
    #[serde(rename = "aghanims_shard")]
    pub aghanims_shard: i64,
    pub moonshard: i64,
    #[serde(rename = "hero_damage")]
    pub hero_damage: i64,
    #[serde(rename = "tower_damage")]
    pub tower_damage: i64,
    #[serde(rename = "hero_healing")]
    pub hero_healing: i64,
    pub gold: i64,
    #[serde(rename = "gold_spent")]
    pub gold_spent: i64,
    #[serde(rename = "scaled_hero_damage")]
    pub scaled_hero_damage: i64,
    #[serde(rename = "scaled_tower_damage")]
    pub scaled_tower_damage: i64,
    #[serde(rename = "scaled_hero_healing")]
    pub scaled_hero_healing: i64,
    #[serde(rename = "ability_upgrades")]
    pub ability_upgrades: Vec<AbilityUpgrade>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityUpgrade {
    pub ability: i64,
    pub time: i64,
    pub level: i64,
}
