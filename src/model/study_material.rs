use super::Time;

#[derive(Clone, Debug, Deserialize)]
pub struct StudyMaterial {
    pub created_at:       Time,
    pub subject_id:       u32,
    pub subject_type:     String,
    pub meaning_note:     Option<String>,
    pub reading_note:     Option<String>,
    pub meaning_synonyms: Vec<String>,
}
