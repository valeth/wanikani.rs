use super::Time;

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
