use std::error::Error;
mod query;

use query::query;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    query().await?;

    Ok(())
}
