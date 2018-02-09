use std::fmt::Debug;
use serde::Serialize;
use serde_urlencoded;

type SerializationError = serde_urlencoded::ser::Error;

pub trait Filter: Serialize + Debug {
    fn encode(&self) -> Result<String, SerializationError>;
}

macro_rules! define_filter_function {
    ($name:ident -> Vec<$type:ty>) => {
        pub fn $name<S>(mut self, values: Vec<S>) -> Self
        where S: ToString
        {
            self.$name = Some(vec_to_string(values));
            self
        }
    };
    ($name:ident -> String) => {
        pub fn $name<S>(mut self, value: S) -> Self
        where S: ToString
        {
            self.$name = Some(value.to_string());
            self
        }
    };
    ($name:ident -> $type:ty) => {
        pub fn $name(mut self, value: $type) -> Self {
            self.$name = Some(value);
            self
        }
    };
}

macro_rules! define_filter {
    ( @st ($name:ident $field:ident => Vec<$type:ty>, $($rest:tt)*) -> ($($out:tt)*) ) => {
        define_filter!(@st
            ($name $($rest)*) -> ($field: Option<String>, $($out)*)
        );
    };
    ( @st ($name:ident $field:ident => $type:ty, $($rest:tt)*) -> ($($out:tt)*) ) => {
        define_filter!(@st
            ($name $($rest)*) -> ($field: Option<$type>, $($out)*)
        );
    };
    ( @st ($name:ident) -> ($($out:tt)*) ) => {
        #[derive(Debug, Default, Serialize)]
        pub struct $name { $($out)* }
    };

    ( @im ($name:ident $field:ident => Vec<$type:tt>, $($rest:tt)*) -> ($($out:tt)*) ) => {
        define_filter!(@im
            ($name $($rest)*) -> (define_filter_function!($field -> Vec<$type>); $($out)*)
        );
    };
    ( @im ($name:ident $field:ident => $type:tt, $($rest:tt)*) -> ($($out:tt)*) ) => {
        define_filter!(@im
            ($name $($rest)*) -> (define_filter_function!($field -> $type); $($out)*)
        );
    };
    ( @im ($name:ident) -> ($($out:tt)*) ) => {
        impl $name {
            $($out)*
        }

        impl Filter for $name {
            fn encode(&self) -> Result<String, SerializationError> {
                serde_urlencoded::to_string(self)
            }
        }
    };
    
    [ $($body:tt)* ] => {
        define_filter!(@st ($($body)*) -> ());
        define_filter!(@im ($($body)*) -> ());
    };
}

define_filter!(SubjectsFilter
    ids           => Vec<u32>,
    types         => Vec<String>,
    slugs         => Vec<String>,
    levels        => Vec<u32>,
    updated_after => String,
);

define_filter!(AssignmentsFilter
    ids              => Vec<u32>,
    created_at       => String,
    subject_ids      => Vec<u32>,
    subject_types    => Vec<String>,
    levels           => Vec<u8>,
    available_before => String,
    available_after  => String,
    srs_stages       => Vec<String>,
    unlocked         => bool,
    started          => bool,
    passed           => bool,
    burned           => bool,
    resurrected      => bool,
    updated_after    => String,
);

define_filter!(ReviewStatisticsFilter
    ids                       => Vec<u32>,
    subject_ids               => Vec<u32>,
    subject_types             => Vec<String>,
    updated_after             => String,
    percentages_greater_than  => u8,
    percentages_less_than     => u8,
);

define_filter!(StudyMaterialsFilter
    ids            => Vec<u32>,
    subject_ids    => Vec<u32>,
    subject_types  => Vec<String>,
    updated_after  => String,
);

define_filter!(ReviewsFilter
    ids            => Vec<u32>,
    assignment_ids => Vec<u32>,
    subject_ids    => Vec<u32>,
    updated_after  => String,
);

define_filter!(LevelProgressionsFilter
    ids           => Vec<u32>,
    updated_after => String,
);

define_filter!(ResetsFilter
    ids           => Vec<u32>,
    updated_after => String,
);

define_filter!(EmptyFilter
    nil           => bool,
);

fn vec_to_string<S>(v: Vec<S>) -> String
where S: ToString
{
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_filter() {
        assert_eq!(
            EmptyFilter::default().encode(),
            Ok("".to_string())
        );
    }

    #[test]
    fn subjects_filter() {
        let filter = SubjectsFilter::default()
            .ids(vec![13,21,53])
            .types(vec![3,10,11])
            .slugs(vec!["this-is", "a-great", "s-l-u-g"])
            .levels(vec![1,2,3])
            .updated_after("2018-01-21T19:23:58.171063Z");

        assert_eq!(&filter.ids, &Some(String::from("13,21,53")));
        assert_eq!(&filter.types, &Some(String::from("3,10,11")));
        assert_eq!(&filter.slugs, &Some(String::from("this-is,a-great,s-l-u-g")));
        assert_eq!(&filter.updated_after, &Some(String::from("2018-01-21T19:23:58.171063Z")));

        assert_eq!(
            filter.encode(),
            Ok("updated_after=2018-01-21T19%3A23%3A58.171063Z\
                &levels=1%2C2%2C3\
                &slugs=this-is%2Ca-great%2Cs-l-u-g\
                &types=3%2C10%2C11\
                &ids=13%2C21%2C53".to_string())
        );
    }

    #[test]
    fn assignments_filter() {
        let query = AssignmentsFilter::default()
            .ids(vec![11,20,100])
            .subject_ids(vec![10,14,30])
            .subject_types(vec![20,42,120])
            .levels(vec![1,2,3])
            .srs_stages(vec![1,2,3])
            .unlocked(true)
            .started(true)
            .passed(true)
            .burned(true)
            .resurrected(false)
            .available_before("2018-01-21T19:23:58.171063Z")
            .available_after("2018-01-21T19:23:58.171063Z")
            .created_at("2018-01-21T19:23:58.171063Z")
            .updated_after("2018-01-21T19:23:58.171063Z")
            .encode();

        assert_eq!(
            query,
            Ok("updated_after=2018-01-21T19%3A23%3A58.171063Z\
                &resurrected=false\
                &burned=true\
                &passed=true\
                &started=true\
                &unlocked=true\
                &srs_stages=1%2C2%2C3\
                &available_after=2018-01-21T19%3A23%3A58.171063Z\
                &available_before=2018-01-21T19%3A23%3A58.171063Z\
                &levels=1%2C2%2C3\
                &subject_types=20%2C42%2C120\
                &subject_ids=10%2C14%2C30\
                &created_at=2018-01-21T19%3A23%3A58.171063Z\
                &ids=11%2C20%2C100".to_string())
        );
    }

    #[test]
    fn review_statistics_filter() {
        let query = ReviewStatisticsFilter::default()
            .ids(vec![12,22,32])
            .subject_ids(vec![3,1,101])
            .subject_types(vec!["vocabulary"])
            .updated_after("2018-01-21T19:23:58.171063Z")
            .percentages_greater_than(70)
            .percentages_less_than(90)
            .encode();

        assert_eq!(
            query,
            Ok("percentages_less_than=90\
                &percentages_greater_than=70\
                &updated_after=2018-01-21T19%3A23%3A58.171063Z\
                &subject_types=vocabulary\
                &subject_ids=3%2C1%2C101\
                &ids=12%2C22%2C32".to_string())
        );
    }

    #[test]
    fn study_materials_filter() {
        let query = StudyMaterialsFilter::default()
            .ids(vec![2,4,1])
            .subject_ids(vec![123,111,11,32])
            .subject_types(vec!["kanji", "radicals"])
            .updated_after("2018-01-21T19:23:58.171063Z")
            .encode();
        
        assert_eq!(
            query,
            Ok("updated_after=2018-01-21T19%3A23%3A58.171063Z\
                &subject_types=kanji%2Cradicals\
                &subject_ids=123%2C111%2C11%2C32\
                &ids=2%2C4%2C1".to_string())
        );
    }

    #[test]
    fn reviews_filter() {
        let query = ReviewsFilter::default()
            .ids(vec![3,1,5])
            .assignment_ids(vec![11,14,112])
            .subject_ids(vec![13,112,1,3])
            .updated_after("2018-01-21T19:23:58.171063Z")
            .encode();

        assert_eq!(
            query,
            Ok("updated_after=2018-01-21T19%3A23%3A58.171063Z\
                &subject_ids=13%2C112%2C1%2C3\
                &assignment_ids=11%2C14%2C112\
                &ids=3%2C1%2C5".to_string())
        );
    }

    #[test]
    fn level_progressions_filter() {
        let query = LevelProgressionsFilter::default()
            .ids(vec![12,3,4,1])
            .updated_after("2018-01-21T19:23:58.171063Z")
            .encode();

        assert_eq!(
            query,
            Ok("updated_after=2018-01-21T19%3A23%3A58.171063Z\
                &ids=12%2C3%2C4%2C1".to_string())
        );
    }

    #[test]
    fn resets_filter() {
        let query = ResetsFilter::default()
            .ids(vec![10,80,9,12,14])
            .updated_after("2018-01-21T19:23:58.171063Z")
            .encode();

        assert_eq!(
            query,
            Ok("updated_after=2018-01-21T19%3A23%3A58.171063Z\
                &ids=10%2C80%2C9%2C12%2C14".to_string())
        );
    }
}
