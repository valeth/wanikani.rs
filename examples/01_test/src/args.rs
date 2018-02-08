use docopt::Docopt;

pub const USAGE: &'static str = "
WaniKani API test client.

Usage:
  01_test --key=<key> <command> [<id>]
  01_test [options]

Options:
  -h, --help       Show this message
  -k, --key=<key>  The WaniKani API key

Commands:
  user
  assignment <assignment-id>
  assignments
  subject <subject-id>
  subjects
  review-statistic <review-statistic-id>
  review-statistics
  study-material <study-material-id>
  study-materials
  summary
  review <review-id>
  reviews
  level-progression <level-progression-id>
  level-progressions
  reset <reset-id>
  resets
";

macro_rules! match_and_print {
    [ @matching ($args:ident: $cmd:ident => $call:expr, $($rest:tt)+) -> ($($out:tt)*) ] => {
        match_and_print!(@matching
            ($args: $($rest)+) -> ($($out)* stringify!($cmd) => { println!("{:#?}", $call); },)
        )
    };
    [ @matching ($args:ident: $cmd:ident => $call:expr,) -> ($($out:tt)*) ] => {
        match $args.arg_command.as_str() {
            $($out)*
            stringify!($cmd) => { println!("{:#?}", $call); },
            _ => { println!("Unknown command!"); },
        }
    };

    [ $($body:tt)* ] => {
        match_and_print![@matching ($($body)*) -> ()];
    };
}

#[derive(Debug, Deserialize)]
pub struct Args {
    pub arg_command: String,
    pub arg_id:      Option<u32>,
    pub flag_key:    String,
}

pub fn with_args<F>(func: F)
where F: FnOnce(&Args) -> ()
{
    let args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    func(&args);
}