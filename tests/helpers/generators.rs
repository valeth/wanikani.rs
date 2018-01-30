use chrono::prelude::{DateTime, Utc};
use wanikani::model::*;

fn date(datestr: &str) -> DateTime<Utc> {
    datestr.parse::<DateTime<Utc>>().unwrap()
}

fn maybe_date(datestr: &str) -> Option<DateTime<Utc>> {
    match datestr.parse::<DateTime<Utc>>() {
        Err(_) => None,
        Ok(date) => Some(date),
    }
}

pub fn gen_user() -> Report<User> {
    Report {
        id: None,
        object: "user".to_string(),
        url: "https://www.wanikani.com/api/v2/user".to_string(),
        data_updated_at: "2018-01-30T16:10:33.327606Z".parse::<DateTime<Utc>>().unwrap(),
        data: User {
            username: "valeth".to_string(),
            level: 11,
            max_level_granted_by_subscription: 60,
            profile_url: "https://www.wanikani.com/users/valeth".to_string(),
            started_at: "2016-07-25T07:00:23.687659Z".parse::<DateTime<Utc>>().unwrap(),
            subscribed: true,
            current_vacation_started_at: None,
        },
    }
}

pub fn gen_subject() -> Report<Subject> {
    Report {
        id: Some(1000),
        object: "kanji".to_string(),
        url: "https://www.wanikani.com/api/v2/subjects/1000".to_string(),
        data_updated_at: "2017-10-04T18:56:28.019412Z".parse::<DateTime<Utc>>().unwrap(),
        data: Subject {
            level: 17,
            created_at: "2012-10-30T19:49:28.123286Z".parse::<DateTime<Utc>>().unwrap(),
            slug:"兵".to_string(),
            document_url: "https://www.wanikani.com/kanji/%E5%85%B5".to_string(),
            characters: Some("兵".to_string()),
            meanings: vec![
                Meaning {
                    meaning: "Soldier".to_string(),
                    primary: true,
                }
            ],
            readings: Some(vec![
                Reading {
                    kind: "onyomi".to_string(),
                    primary: true,
                    reading: "へい".to_string(),
                },
                Reading {
                    kind: "onyomi".to_string(),
                    primary: true,
                    reading: "ひょう".to_string(),
                },
                Reading {
                    kind: "kunyomi".to_string(),
                    primary: false,
                    reading: "None".to_string(),
                }
            ]),
            component_subject_ids: Some(vec![115, 2, 1]),
            parts_of_speech: None,
            character_images: None,
        }
    }
}

pub fn gen_assignment() -> Report<Assignment> {
    Report {
        id: Some(88214909),
        object: "assignment".to_string(),
        url: "https://www.wanikani.com/api/v2/assignments/88214909".to_string(),
        data_updated_at: date("2018-01-21T19:23:58.171063Z"),
        data: Assignment {
            created_at: date("2017-11-26T10:00:14.628401Z"),
        subject_id: 542,
        subject_type: "kanji".to_string(),
        level: 4,
        srs_stage: 8,
        srs_stage_name: "Enlightened".to_string(),
        unlocked_at: maybe_date("2017-11-26T10:00:14.651719Z"),
        started_at: maybe_date("2017-11-27T12:28:44.093736Z"),
        passed_at: maybe_date("2017-12-01T20:13:41.075740Z"),
        burned_at: None,
        available_at: maybe_date("2018-05-21T18:00:00.000000Z"),
        resurrected_at: None,
        passed: true,
        resurrected: false
        }
    }
}
