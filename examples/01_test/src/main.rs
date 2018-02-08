#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate wanikani;

#[macro_use]
mod args;
use args::with_args;

use wanikani::Client as WaniKani;

fn main() {
    with_args(|args| {
        let wk = WaniKani::configure(args.flag_key.to_owned());
        let id = args.arg_id.unwrap_or_default().to_owned();

        match_and_print![args:
            user               => wk.user(),
            summary            => wk.summary(),
            subject            => wk.subject(id),
            subjects           => wk.subjects(),
            assignment         => wk.assignment(id),
            assignments        => wk.assignments(),
            review_statistic   => wk.review_statistic(id),
            review_statistics  => wk.review_statistics(),
            study_material     => wk.study_material(id),
            study_materials    => wk.study_materials(),
            review             => wk.review(id),
            reviews            => wk.reviews(),
            level_progression  => wk.level_progression(id),
            level_progressions => wk.level_progressions(),
            reset              => wk.reset(id),
            resets             => wk.resets(),
        ];
    });
}