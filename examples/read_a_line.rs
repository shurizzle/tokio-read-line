extern crate tokio;
extern crate tokio_read_line;

use std::io::Write;

use tokio_read_line::{ReadLines, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut lines = ReadLines::new()?;
    print!("> ");
    std::io::stdout().flush()?;
    println!("Line: {:?}", lines.next().await);

    Ok(())
}
