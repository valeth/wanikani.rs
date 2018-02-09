use docopt::Docopt;

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
pub struct Args {
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
