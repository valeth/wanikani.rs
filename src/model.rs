use chrono::prelude::{DateTime, Utc};
use std::collections::HashMap as Map;

type Time = DateTime<Utc>;

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Report<T> {
    pub id:              Option<u32>,
    pub object:          ObjectType,
    pub url:             String,
    pub data_updated_at: Option<Time>,
    pub data:            T,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    User,
    Kanji,
    Vocabulary,
    Radical,
    Assignment,
    ReviewStatistic,
    StudyMaterial,
    Summary,
    Review,
    LevelProgression,
    Reset,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Collection<T> {
    pub object:          String,
    pub url:             String,
    pub data_updated_at: Option<Time>,
    pub pages:           Pages,
    pub total_count:     u32,
    pub data:            Vec<T>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pages {
    pub per_page:     u32,
    pub next_url:     Option<String>,
    pub previous_url: Option<String>
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct User {
    pub username:                          String,
    pub level:                             u8,
    pub max_level_granted_by_subscription: u8,
    pub started_at:                        Time,
    pub subscribed:                        bool,
    pub current_vacation_started_at:       Option<Time>,
    pub profile_url:                       String,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Subject {
    pub level:                 u8,
    pub created_at:            Time,
    pub slug:                  String,
    pub document_url:          String,
    pub characters:            Option<String>,
    pub meanings:              Vec<Meaning>,
    pub character_images:      Option<Vec<Map<String, String>>>, // only radicals
    pub readings:              Option<Vec<Reading>>,             // only kanji, vocabulary
    pub parts_of_speech:       Option<Vec<String>>,              // only vocabulary
    pub component_subject_ids: Option<Vec<u32>>,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Meaning {
    pub meaning: String,
    pub primary: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Reading {
    #[serde(rename="type")]
    pub kind:    Option<String>,    // only kanji
    pub primary: bool,
    pub reading: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Assignment {
    pub created_at:     Time,
    pub subject_id:     u32,
    pub subject_type:   String,
    pub level:          u8,
    pub srs_stage:      u8,
    pub srs_stage_name: String,
    pub unlocked_at:    Option<Time>,
    pub started_at:     Option<Time>,
    pub passed_at:      Option<Time>,
    pub burned_at:      Option<Time>,
    pub available_at:   Option<Time>,
    pub resurrected_at: Option<Time>,
    pub passed:         bool,
    pub resurrected:    bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReviewStatistic {
    pub created_at:             Time,
    pub subject_id:             u32,
    pub meaning_correct:        u32,
    pub meaning_incorrect:      u32,
    pub meaning_max_streak:     u32,
    pub meaning_current_streak: u32,
    pub reading_correct:        u32,
    pub reading_incorrect:      u32,
    pub reading_max_streak:     u32,
    pub reading_current_streak: u32,
    pub percentage_correct:     u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StudyMaterial {
    pub created_at:       Time,
    pub subject_id:       u32,
    pub subject_type:     String,
    pub meaning_note:     Option<String>,
    pub reading_note:     Option<String>,
    pub meaning_synonyms: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Summary {
    pub reviews: Vec<SummaryItem>,
    pub lessons: Vec<SummaryItem>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SummaryItem {
    pub available_at: Time,
    pub subject_ids:  Vec<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Review {
    pub created_at:                Time,
    pub assignment_id:             u32,
    pub subject_id:                u32,
    pub starting_srs_stage:        u32,
    pub starting_srs_stage_name:   String,
    pub ending_srs_stage:          u32,
    pub ending_srs_stage_name:     String,
    pub incorrect_meaning_answers: Option<u32>,
    pub incorrect_reading_answers: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LevelProgression {
    pub level:        u8,
    pub created_at:   Time,
    pub unlocked_at:  Option<Time>,
    pub started_at:   Option<Time>,
    pub passed_at:    Option<Time>,
    pub completed_at: Option<Time>,
    pub abandoned_at: Option<Time>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Reset {
    pub created_at:     Time,
    pub original_level: u8,
    pub target_level:   u8,
    pub confirmed_at:   Time,
}
