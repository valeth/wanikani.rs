use docopt::Docopt;

pub const USAGE: &'static str = "
WaniKani API test client.

Usage:
  01_test --key=<key> user
  01_test --key=<key> assignment <assignment-id>
  01_test --key=<key> assignments
  01_test --key=<key> subject <subject-id>
  01_test --key=<key> subjects
  01_test --key=<key> review-statistic <review-statistic-id>
  01_test --key=<key> review-statistics
  01_test --key=<key> study-material <study-material-id>
  01_test --key=<key> study-materials
  01_test --key=<key> summary
  01_test --key=<key> review <review-id>
  01_test --key=<key> reviews
  01_test --key=<key> level-progression <level-progression-id>
  01_test --key=<key> level-progressions
  01_test --key=<key> reset <reset-id>
  01_test --key=<key> resets
  01_test (-h | --help)

Options:
  -h, --help       Show this message
  --key=<key>      The WaniKani API key
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub cmd_user: bool,
    pub cmd_assignment: bool,
    pub cmd_assignments: bool,
    pub cmd_subject: bool,
    pub cmd_subjects: bool,
    pub cmd_review_statistic: bool,
    pub cmd_review_statistics: bool,
    pub cmd_study_material: bool,
    pub cmd_study_materials: bool,
    pub cmd_summary: bool,
    pub cmd_review: bool,
    pub cmd_reviews: bool,
    pub cmd_level_progression: bool,
    pub cmd_level_progressions: bool,
    pub cmd_reset: bool,
    pub cmd_resets: bool,
    pub arg_assignment_id: u32,
    pub arg_subject_id: u32,
    pub arg_review_statistic_id: u32,
    pub arg_study_material_id: u32,
    pub arg_review_id: u32,
    pub arg_level_progression_id: u32,
    pub arg_reset_id: u32,
    pub flag_key:          String,
}

pub fn parse_args() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
}