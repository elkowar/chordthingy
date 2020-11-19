use app::App;
use mappings::Mappings;

use anyhow::*;

pub mod app;
pub mod history;
pub mod input;
pub mod mappings;
use input::input_device::*;

fn main() -> Result<()> {
    let mappings_file = std::fs::File::open("/home/leon/.config/chordthingy/mappings.json")?;
    let mappings = Mappings::from_reader(mappings_file)?;
    dbg!(&mappings);
    let mut app = App::new(mappings)?;
    app.run()?;
    Ok(())
}
