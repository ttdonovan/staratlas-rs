use color_eyre::Result;

use shared_time as time;

mod app;
mod errors;
mod term;

fn main() -> Result<()> {
    errors::init_hooks()?;
    term::init()?;
    app::run()?;
    term::restore()?;
    Ok(())
}
