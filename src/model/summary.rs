use super::Time;

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
