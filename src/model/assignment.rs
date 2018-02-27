use super::Time;

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Assignment {
    pub created_at:     Time,
    pub subject_id:     u32,
    pub subject_type:   String,
    pub level:          u8,
    pub srs_stage:      u8,
    pub srs_stage_name: String,
    pub unlocked_at:    Option<Time>,
    pub started_at:     Option<Time>,
    pub passed_at:      Option<Time>,
    pub burned_at:      Option<Time>,
    pub available_at:   Option<Time>,
    pub resurrected_at: Option<Time>,
    pub passed:         bool,
    pub resurrected:    bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::{date, maybe_date, fixture};
    use model::{Report, ObjectType};
    use serde_json;

    #[test]
    fn parse_assignment() {
        let file = fixture("assignment");
        let assignment: Report<Assignment> = serde_json::from_reader(file).unwrap();

        assert_eq!(
            assignment,
            Report {
                id: Some(88214909),
                object: ObjectType::Assignment,
                url: "https://www.wanikani.com/api/v2/assignments/88214909".to_string(),
                data_updated_at: maybe_date("2018-01-21T19:23:58.171063Z"),
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
        );
    }
}
