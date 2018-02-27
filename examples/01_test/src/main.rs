#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate wanikani;

mod args;
use args::with_args;

use wanikani::Client as WaniKani;

fn main() {
    with_args(|args| {
        let wk = WaniKani::configure(args.flag_key.to_owned());

        if let Ok(user) = wk.user() {
            println!("{:#?}", user);
        };

        if let Ok(subjects1) = wk.subjects(|f| f.levels(&[1, 2, 3]).types(&["kanji", "vocabulary", "radical"]))
        {

            println!("{:#?}", subjects1);
        };

        if let Ok(assignment1) = wk.assignment(88214909) {
            println!("{:#?}", assignment1);
        };
    });
}
