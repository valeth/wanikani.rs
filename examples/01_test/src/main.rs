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

        let user = wk.user();

        println!("{:#?}", user);

        let subjects1 = wk.subjects(|f| f
            .levels(&[1, 2, 3])
            .types(&["kanji", "vocabulary"]));
        
        println!("{:#?}", subjects1);

        let assignment = wk.assignment(88214909);

        println!("{:#?}", assignment);
    });
}
