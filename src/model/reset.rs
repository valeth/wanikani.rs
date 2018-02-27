use super::Time;

#[derive(Clone, Debug, Deserialize)]
pub struct Reset {
    pub created_at:     Time,
    pub original_level: u8,
    pub target_level:   u8,
    pub confirmed_at:   Time,
}
