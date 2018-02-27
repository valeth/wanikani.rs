use std::collections::HashMap as Map;
use super::Time;

#[derive(Clone, PartialEq, Debug, Deserialize)]
#[serde(untagged)]
pub enum Subject {
    Kanji {
        level: u8,
        created_at: Time,
        slug: String,
        document_url: String,
        characters: String,
        meanings: Vec<Meaning>,
        readings: Vec<KanjiReading>,
        component_subject_ids: Vec<u32>,
    },
    Vocabulary {
        level: u8,
        created_at: Time,
        slug: String,
        document_url: String,
        characters: String,
        meanings: Vec<Meaning>,
        readings: Vec<VocabularyReading>,
        parts_of_speech: Vec<String>,
        component_subject_ids: Vec<u32>,
    },
    Radical {
        level: u8,
        created_at: Time,
        slug: String,
        document_url: String,
        characters: Option<String>,
        meanings: Vec<Meaning>,
        character_images: Option<Vec<Map<String, String>>>,
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Meaning {
    pub meaning: String,
    pub primary: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KanjiReading {
    Onyomi  { primary: bool, reading: String },
    Kunyomi { primary: bool, reading: String },
    Nanori  { primary: bool, reading: String },
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct VocabularyReading {
    primary: bool,
    reading: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::{date, maybe_date, fixture};
    use model::{Report, ObjectType};
    use serde_json;

    #[test]
    fn parse() {
        let file = fixture("subject");
        let report: Report<Subject> = serde_json::from_reader(file).unwrap();

        assert_eq!(
            report,
            Report {
                id: Some(1000),
                object: ObjectType::Kanji,
                url: "https://www.wanikani.com/api/v2/subjects/1000".to_string(),
                data_updated_at: maybe_date("2017-10-04T18:56:28.019412Z"),
                data: Subject::Kanji {
                    level: 17,
                    created_at: date("2012-10-30T19:49:28.123286Z"),
                    slug:"兵".to_string(),
                    document_url: "https://www.wanikani.com/kanji/%E5%85%B5".to_string(),
                    characters: "兵".to_string(),
                    meanings: vec![
                        Meaning {
                            meaning: "Soldier".to_string(),
                            primary: true,
                        }
                    ],
                    readings: vec![
                        KanjiReading::Onyomi {
                            primary: true,
                            reading: "へい".to_string(),
                        },
                        KanjiReading::Onyomi {
                            primary: true,
                            reading: "ひょう".to_string(),
                        },
                        KanjiReading::Kunyomi {
                            primary: false,
                            reading: "None".to_string(),
                        }
                    ],
                    component_subject_ids: vec![115, 2, 1],
                }
            }
        );
    }
}
