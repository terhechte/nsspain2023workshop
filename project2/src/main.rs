mod project;
mod utils;

use crate::utils::AsTable;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    color_backtrace::install();

    let items = crate::project::parse("tweets.json".to_string())?;

    let output = crate::project::top_words(items, 100);

    output.print_table();

    Ok(())
}
