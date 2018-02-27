#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate wanikani;

use docopt::Docopt;
use wanikani::Client as WaniKani;

// Just some boilerplate to read the API key
pub const USAGE: &'static str = "
WaniKani API test client.

Usage:
  01_test --key=<key>
  01_test [options]

Options:
  -h, --help       Show this message
  -k, --key=<key>  The WaniKani API key
";

#[derive(Debug, Deserialize)]
struct Args {
    pub flag_key: String,
}

fn with_args<F>(func: F)
where F: FnOnce(&Args) -> ()
{
    let args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    func(&args);
}

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
