use app::App;
use keyboard::ev_dev::EvDevBackend;
use mappings::Mappings;

use anyhow::*;

pub mod app;
pub mod history;
pub mod keyboard;
pub mod mappings;

fn main() -> Result<()> {
    let mappings_file = std::fs::File::open("/home/leon/.config/chordthingy/mappings.json")?;
    let mappings = Mappings::from_reader(mappings_file)?;
    dbg!(&mappings);

    let backend = init_evdev_backend()?;

    let mut app = App::new(backend, mappings)?;
    app.run()?;
    Ok(())
}

fn init_evdev_backend() -> Result<EvDevBackend> {
    let device_path = std::env::args()
        .nth(1)
        .unwrap_or("/dev/input/event24".to_owned());

    let device_file = std::fs::File::open(device_path)?;

    let mut device = evdev_rs::Device::new().context("Error getting device")?;
    device.set_fd(device_file)?;
    let input_device = evdev_rs::UInputDevice::create_from_device(&device)?;

    Ok(EvDevBackend::new(input_device, device))
}
