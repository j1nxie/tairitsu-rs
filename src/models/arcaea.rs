use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArcaeaResponse {
    pub success: bool,
    pub value: UserData,
}

#[derive(Deserialize)]
pub struct UserData {
    pub friends: Vec<Friend>,
    pub user_id: u64,
    pub name: String,
    pub user_code: String,
    pub display_name: String,
    pub character: u32,
    pub country: String,
    pub course_banners: Vec<String>,
    pub max_friend: u32,
    pub rating: i32,
    pub join_date: u64,
}

#[derive(Deserialize)]
pub struct Friend {
    pub character: u32,
    // TODO: come back to this
    // pub recent_score: Vec<FriendRecentScore>,
    pub rating: i32,
    pub is_char_uncapped_override: bool,
    pub is_char_uncapped: bool,
    pub is_mutual: bool,
    pub user_id: u64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct ArcaeaSettings {
    pub is_hide_rating: bool,
    pub favorite_character: u32,
    pub max_stamina_notification_enabled: bool,
}

#[derive(Deserialize)]
pub struct RecentScore {
    pub song_id: String,
    pub difficulty: Difficulty,
    pub score: u32,
    pub shiny_perfect_count: u32,
    pub perfect_count: u32,
    pub near_count: u32,
    pub miss_count: u32,
    pub modifier: u32,
    pub clear_type: ClearType,
    pub best_clear_type: ClearType,
    pub health: u32,
    pub time_played: u64,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Difficulty {
    Past = 0,
    Present = 1,
    Future = 2,
    Beyond = 3,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ClearType {
    // TODO: figure out the clear types, these are random values i assumed.
    EC = 0,
    NC = 1,
    HC = 2,
}
