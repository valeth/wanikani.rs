use chrono::prelude::{DateTime, Utc};

mod assignment;
mod level_progression;
mod reset;
mod review_statistic;
mod review;
mod study_material;
mod subject;
mod summary;
mod user;

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
    pub data:            Vec<Report<T>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pages {
    pub per_page:     u32,
    pub next_url:     Option<String>,
    pub previous_url: Option<String>
}

pub mod prelude {
    pub use super::assignment::Assignment;
    pub use super::level_progression::LevelProgression;
    pub use super::reset::Reset;
    pub use super::review_statistic::ReviewStatistic;
    pub use super::review::Review;
    pub use super::study_material::StudyMaterial;
    pub use super::subject::Subject;
    pub use super::summary::Summary;
    pub use super::user::User;
    pub use super::{Collection, Report};
}
