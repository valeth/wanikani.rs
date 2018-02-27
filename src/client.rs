
use reqwest::{Client as ReqwestClient};
use serde::de::DeserializeOwned;
use ::requester::WaniKaniRequester;
use ::Error;
use ::model::*;
use ::filters::*;

pub struct Client {
    api_key: String,
    requester: ReqwestClient,
}

macro_rules! define_request {
    ($name:ident(id) -> $ret:ty) => {
        pub fn $name(&self, id: u32) -> Result<Report<$ret>, Error> {
            self.request(format!("{}s/{}", stringify!($name), id), EmptyFilter::default())
        }
    };
    ($name:ident($filter:tt) -> [$ret:ty]) => {
        pub fn $name<F>(&self, f: F) -> Result<Collection<$ret>, Error>
        where F: FnOnce($filter) -> $filter
        {
            self.request(stringify!($name), f($filter::default()))
        }
    };
    ($name:ident -> $ret:ty) => {
        pub fn $name(&self) -> Result<Report<$ret>, Error> {
            self.request(stringify!($name), EmptyFilter::default())
        }
    };
}

macro_rules! define_requests {
    [ @req ($name:ident($filter:tt) -> [$ret:tt], $($rest:tt)*) -> ($($out:tt)*) ] => {
        define_requests![@req ($($rest)*) -> (define_request!($name($filter) -> [$ret]); $($out)*)];
    };
    [ @req ($name:ident(id) -> $ret:tt, $($rest:tt)*) -> ($($out:tt)*) ] => {
        define_requests![@req ($($rest)*) -> (define_request!($name(id) -> $ret); $($out)*)];
    };
    [ @req ($name:ident -> $ret:tt, $($rest:tt)*) -> ($($out:tt)*) ] => {
        define_requests![@req ($($rest)*) -> (define_request!($name -> $ret); $($out)*)];
    };
    [ @req ($name:ident -> $ret:tt) -> ($($out:tt)*) ] => {
        define_request!($name -> $ret); $($out)*
    };

    [ $($body:tt)* ] => {
        define_requests!(@req ($($body)*) -> ());
    };
}

impl Client {
    fn request<T, S, F>(&self, resource: S, filter: F) -> Result<T, Error>
        where T: DeserializeOwned,
              S: Into<String>,
              F: Filter
    {
        self.requester.api_request(&self.api_key, resource.into(), filter)
    }

    pub fn configure<S: Into<String>>(api_key: S) -> Client {
        Client {
            api_key: api_key.into(),
            requester: ReqwestClient::new()
        }
    }

    define_requests![
        subject(id)                                 -> Subject,
        review_statistic(id)                        -> ReviewStatistic,
        assignment(id)                              -> Assignment,
        review(id)                                  -> Review,
        study_material(id)                          -> StudyMaterial,
        level_progression(id)                       -> LevelProgression,
        reset(id)                                   -> Reset,
        subjects(SubjectsFilter)                    -> [Subject],
        review_statistics(ReviewStatisticsFilter)   -> [ReviewStatistic],
        assignments(AssignmentsFilter)              -> [Assignment],
        reviews(ReviewsFilter)                      -> [Review],
        study_materials(StudyMaterialsFilter)       -> [StudyMaterial],
        level_progressions(LevelProgressionsFilter) -> [LevelProgression],
        resets(ResetsFilter)                        -> [Reset],
        summary                                     -> Summary,
        user                                        -> User
    ];

}
