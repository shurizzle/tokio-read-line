extern crate tokio;
extern crate tokio_read_line;

use std::io::Write;

use futures::StreamExt;
use tokio_read_line::{ReadLines, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut lines = ReadLines::new()?;
    print!("> ");
    std::io::stdout().flush()?;
    match lines.next().await {
        None => println!("None"),
        Some(res) => println!("Line: {:#?}", res?),
    }

    Ok(())
}
