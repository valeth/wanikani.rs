use super::Time;

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct User {
    pub username:                          String,
    pub level:                             u8,
    pub max_level_granted_by_subscription: u8,
    pub started_at:                        Time,
    pub subscribed:                        bool,
    pub current_vacation_started_at:       Option<Time>,
    pub profile_url:                       String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::{date, maybe_date, fixture};
    use model::{Report, ObjectType};
    use serde_json;

    #[test]
    fn parse() {
        let file = fixture("user");
        let user: Report<User> = serde_json::from_reader(file).unwrap();

        assert_eq!(
            user,
            Report {
                id: None,
                object: ObjectType::User,
                url: "https://www.wanikani.com/api/v2/user".to_string(),
                data_updated_at: maybe_date("2018-01-30T16:10:33.327606Z"),
                data: User {
                    username: "valeth".to_string(),
                    level: 11,
                    max_level_granted_by_subscription: 60,
                    profile_url: "https://www.wanikani.com/users/valeth".to_string(),
                    started_at: date("2016-07-25T07:00:23.687659Z"),
                    subscribed: true,
                    current_vacation_started_at: None,
                },
            }
        );
    }
}
