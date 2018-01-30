#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate wanikani;

mod args;
use args::{parse_args, Args};
use wanikani::Client as WaniKani;


// TODO: Use some kind of macro for this, because this is ridiculous.
fn query(wk: &WaniKani, args: &Args) {
    if args.cmd_user {
        println!("{:#?}", wk.user());
    } else if args.cmd_subject {
        println!("{:#?}", wk.subject(args.arg_subject_id.to_owned()));
    } else if args.cmd_subjects {
        println!("{:#?}", wk.subjects());
    } else if args.cmd_assignment {
        println!("{:#?}", wk.assignment(args.arg_assignment_id.to_owned()));
    } else if args.cmd_assignments {
        println!("{:#?}", wk.assignments());
    } else if args.cmd_review_statistic {
        println!("{:#?}", wk.review_statistic(args.arg_review_statistic_id.to_owned()));
    } else if args.cmd_review_statistics {
        println!("{:#?}", wk.review_statistics());
    } else if args.cmd_study_material {
        println!("{:#?}", wk.study_material(args.arg_study_material_id.to_owned()));
    } else if args.cmd_study_materials {
        println!("{:#?}", wk.study_materials());
    } else if args.cmd_summary {
        println!("{:#?}", wk.summary());
    } else if args.cmd_review {
        println!("{:#?}", wk.review(args.arg_review_id.to_owned()));
    } else if args.cmd_reviews {
        println!("{:#?}", wk.reviews());
    } else if args.cmd_level_progression {
        println!("{:#?}", wk.level_progression(args.arg_level_progression_id.to_owned()));
    } else if args.cmd_level_progressions {
        println!("{:#?}", wk.level_progressions());
    } else if args.cmd_reset {
        println!("{:#?}", wk.reset(args.arg_reset_id.to_owned()));
    } else if args.cmd_resets {
        println!("{:#?}", wk.resets());
    }
}

fn main() {
    let args = parse_args();

    let wk = WaniKani::configure(args.flag_key.to_owned());

    query(&wk, &args);
}