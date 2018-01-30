use reqwest::{Client as ReqwestClient};
use serde::de::DeserializeOwned;
use ::requester::WaniKaniRequester;
use ::Error;
use ::model::*;

pub struct Client {
    api_key: String,
    requester: ReqwestClient,
}

impl Client {
    fn request<T, S>(&self, resource: S) -> Result<T, Error>
        where T: DeserializeOwned,
              S: Into<String>
    {
        self.requester.api_request(&self.api_key, resource.into())
    }

    pub fn configure<S: Into<String>>(api_key: S) -> Client {
        Client {
            api_key: api_key.into(),
            requester: ReqwestClient::new()
        }
    }

    pub fn user(&self) -> Result<Report<User>, Error> {
        self.request("user")
    }

    pub fn subjects(&self) -> Result<Collection<Report<Subject>>, Error> {
        self.request("subjects")
    }

    pub fn subject(&self, id: u32) -> Result<Report<Subject>, Error> {
        self.request(format!("subjects/{}", id))
    }

    pub fn assignments(&self) -> Result<Collection<Report<Assignment>>, Error> {
        self.request("assignments")
    }

    pub fn assignment(&self, id: u32) -> Result<Report<Assignment>, Error> {
        self.request(format!("assignments/{}", id))
    }

    pub fn review_statistic(&self, id: u32) -> Result<Report<ReviewStatistic>, Error> {
        self.request(format!("review_statistics/{}", id))
    }

    pub fn review_statistics(&self) -> Result<Collection<Report<ReviewStatistic>>, Error> {
        self.request("review_statistics")
    }

    pub fn study_material(&self, id: u32) -> Result<Report<StudyMaterial>, Error> {
        self.request(format!("study_materials/{}", id))
    }

    pub fn study_materials(&self) -> Result<Collection<Report<StudyMaterial>>, Error> {
        self.request("study_materials")
    }

    pub fn summary(&self) -> Result<Report<Summary>, Error> {
        self.request("summary")
    }

    pub fn review(&self, id: u32) -> Result<Report<Review>, Error> {
        self.request(format!("reviews/{}", id))
    }

    pub fn reviews(&self) -> Result<Collection<Report<Review>>, Error> {
        self.request("reviews")
    }

    pub fn level_progression(&self, id: u32) -> Result<Report<LevelProgression>, Error> {
        self.request(format!("level_progressions/{}", id))
    }

    pub fn level_progressions(&self) -> Result<Collection<Report<LevelProgression>>, Error> {
        self.request("level_progressions")
    }

    pub fn reset(&self, id: u32) -> Result<Report<Reset>, Error> {
        self.request(format!("resets/{}", id))
    }

    pub fn resets(&self) -> Result<Collection<Report<Reset>>, Error> {
        self.request("resets")
    }
}
