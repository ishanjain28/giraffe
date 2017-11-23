
use std::env;

#[derive(Debug, PartialEq)]
pub enum ws {
    X11,
    Wayland,
}

pub fn detect_ws() -> Result<ws, env::VarError> {
    let wayland = env::var("WAYLAND_DISPLAY");

    let x11 = env::var("DISPLAY");

    match wayland {
        Ok(v) => Ok(ws::Wayland), 
        Err(e) => {
            if e == env::VarError::NotPresent {

                match x11 {
                    Ok(v) => Ok(ws::X11), 
                    Err(e) => Err(e),
                }
            } else {
                Err(e)
            }
        }
    }
}
