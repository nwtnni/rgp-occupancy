use std::env;
use std::error;
use std::io::Write as _;
use std::fs;
use std::time;

use once_cell::sync;
use reqwest::blocking;

static COUNT: sync::Lazy<regex::Regex> = sync::Lazy::new(|| {
        regex::RegexBuilder::new(r#"'SBO'[^}]*'count'[^,]*(\d+),"#)
            .multi_line(true)
            .build()
            .expect("[INTERNAL ERROR]: invalid regex")

});

static URL: &str = "https://portal.rockgympro.com/portal/public/7a2ec613bb982d4ba91785c2cdb45902/occupancy?&iframeid=occupancyCounter&fId=1325";

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = env::args()
        .nth(1)
        .expect("Usage: `rock-spot <PATH_TO_LOG_FILE>`");

    let client = blocking::Client::builder()
        .user_agent("rock-spot-bot/1.0 nwtnni@gmail.com")
        .build()
        .expect("[INTERNAL ERROR]: invalid reqwest client");

    let mut log = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let html = client
        .get(URL)
        .send()
        .and_then(blocking::Response::text)?;

    let count = COUNT
        .captures(&*html)
        .and_then(|captures| captures.get(1))
        .expect("[INTERNAL ERROR]: count CSS selector returned nothing")
        .as_str();

    let time = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("[INTERNAL ERROR]: time went backwards");

    writeln!(&mut log, "{},{}", time.as_secs(), count)?;

    Ok(())
}
