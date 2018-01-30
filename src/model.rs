use chrono::prelude::{DateTime, Utc};
use std::collections::HashMap as Map;

type Time = DateTime<Utc>;

#[derive(Clone, Debug, Deserialize)]
pub struct Report<T> {
    id:              Option<u32>,
    object:          String,
    url:             String,
    data_updated_at: Time,
    data:            T,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Collection<T> {
    object:          String,
    url:             String,
    data_updated_at: Time,
    pages:           Pages,
    total_count:     u32,
    data:            Vec<T>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pages {
    per_page:     u32,
    next_url:     Option<String>,
    previous_url: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    username:                          String,
    level:                             u8,
    max_level_granted_by_subscription: u8,
    started_at:                        Time,
    subscribed:                        bool,
    current_vacation_started_at:       Option<Time>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Subject {
    level:                 u8,
    created_at:            Time,
    slug:                  String,
    document_url:          String,
    characters:            Option<String>,
    meanings:              Vec<Meaning>,
    character_images:      Option<Vec<Map<String, String>>>, // only radicals
    readings:              Option<Vec<Reading>>,             // only kanji, vocabulary
    parts_of_speech:       Option<Vec<String>>,              // only vocabulary
    component_subject_ids: Option<Vec<u32>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Meaning {
    meaning: String,
    primary: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Reading {
    #[serde(rename="type")]
    kind:    String,
    primary: bool,
    reading: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Assignment {
    created_at:     Time,
    subject_id:     u32,
    subject_type:   String,
    level:          u8,
    srs_stage:      u8,
    srs_stage_name: String,
    unlocked_at:    Option<Time>,
    started_at:     Option<Time>,
    passed_at:      Option<Time>,
    burned_at:      Option<Time>,
    available_at:   Option<Time>,
    resurrected_at: Option<Time>,
    passed:         bool,
    resurrected:    bool,
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
