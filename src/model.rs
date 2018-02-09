use chrono::prelude::{DateTime, Utc};
use std::collections::HashMap as Map;

type Time = DateTime<Utc>;

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Report<T> {
    pub id:              Option<u32>,
    pub object:          String,
    pub url:             String,
    pub data_updated_at: Time,
    pub data:            T,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Collection<T> {
    pub object:          String,
    pub url:             String,
    pub data_updated_at: Time,
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
    created_at:             Time,
    subject_id:             u32,
    meaning_correct:        u32,
    meaning_incorrect:      u32,
    meaning_max_streak:     u32,
    meaning_current_streak: u32,
    reading_correct:        u32,
    reading_incorrect:      u32,
    reading_max_streak:     u32,
    reading_current_streak: u32,
    percentage_correct:     u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StudyMaterial {
    created_at:       Time,
    subject_id:       u32,
    subject_type:     String,
    meaning_note:     Option<String>,
    reading_note:     Option<String>,
    meaning_synonyms: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Summary {
    reviews: Vec<SummaryItem>,
    lessons: Vec<SummaryItem>,
}

#[derive(Clone, Debug, Deserialize)]
struct SummaryItem {
    available_at: Time,
    subject_ids:  Vec<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Review {
    created_at:                Time,
    assignment_id:             u32,
    subject_id:                u32,
    starting_srs_stage:        u32,
    starting_srs_stage_name:   String,
    ending_srs_stage:          u32,
    ending_srs_stage_name:     String,
    incorrect_meaning_answers: Option<u32>,
    incorrect_reading_answers: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LevelProgression {
    level:        u8,
    created_at:   Time,
    unlocked_at:  Option<Time>,
    started_at:   Option<Time>,
    passed_at:    Option<Time>,
    completed_at: Option<Time>,
    abandoned_at: Option<Time>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Reset {
    created_at:     Time,
    original_level: u8,
    target_level:   u8,
    confirmed_at:   Time,
}
