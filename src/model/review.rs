use super::Time;

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
