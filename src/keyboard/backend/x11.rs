use anyhow::*;
//use x11rb::{self, connection::Connection};

use crate::keyboard::{Backend, KeyEvent};

pub struct X11Backend {}

//impl X11Backend {
//pub fn new() -> Result<Self> {
//let (conn, screen_num) = x11rb::connect(None)?;
//let screen = &conn.setup().roots[screen_num];
//screen.default_colormap

//let win_aux =
////x11rb::protocol::xproto::CreateWindowAux::new().background_pixel(screen.white_pixel);

//Ok(X11Backend {})
//}
//}

//impl Backend for X11Backend {
//fn handle_events<F: FnMut(KeyEvent)>(&self, f: F) -> Result<()> {
//loop {
//let event = self
//.connection
//.wait_for_event()
//.context("Error waiting for event from x11")?;
//match event {
//x11rb::protocol::Event::KeyPress(event) => {
//event.detail;
//}
//_ => {}
//}
//}
//}

//fn send_key_event(&self, event: KeyEvent) -> Result<()> {}
//}
