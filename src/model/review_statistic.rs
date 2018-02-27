use super::Time;

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
