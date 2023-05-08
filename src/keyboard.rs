use crate::ps2::{Controller, ControllerConfigFlags};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Key {
    Release,
    Unknown,
    Letter(char),
    Space,
    Enter,
    LeftShift,
    RightShift,
    CapsLock
}

pub struct Keyboard {
    controller: Controller,
    release: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            controller: unsafe { Controller::new() },
            release: false,
        }
    }

    pub fn initialize(&mut self) {
        let mut config = self.controller.read_config().unwrap();
        config.set(ControllerConfigFlags::ENABLE_KEYBOARD_INTERRUPT | ControllerConfigFlags::ENABLE_MOUSE_INTERRUPT | ControllerConfigFlags::ENABLE_TRANSLATE, false);
        self.controller.write_config(config).unwrap();
    }

    pub fn read_input(&mut self) -> Option<Key> {
        match self.controller.read_data() {
            Ok(scancode) => {
                let key = match_scancode(scancode);
                return match (self.release, key) {
                    (_, Key::Release) => { self.release = true; None }
                    (true, Key::LeftShift | Key::RightShift) => { self.release = false; Some(key) }
                    (false, Key::CapsLock) => { Some(key) }
                    (true, _) => { self.release = false; None }
                    (false, _) => { Some(key) }
                }
            }
            Err(_) => return None
        }
    }

    pub fn read_debug_input(&mut self) -> Option<u8> {
        match self.controller.read_data() {
            Ok(scancode) => return Some(scancode),
            Err(_) => return None
        }
    }

    // match keyboard.read_debug_input() {
    //     Some(scancode) => print!("{}\n", scancode),
    //     None => ()
    // }
}

fn match_scancode(scancode: u8) -> Key {
    return match scancode {
        0x15 => Key::Letter('q'),
        0x1d => Key::Letter('w'),
        0x24 => Key::Letter('e'),
        0x2d => Key::Letter('r'),
        0x2c => Key::Letter('t'),
        0x35 => Key::Letter('y'),
        0x3c => Key::Letter('u'),
        0x43 => Key::Letter('i'),
        0x44 => Key::Letter('o'),
        0x4d => Key::Letter('p'),
        0x1c => Key::Letter('a'),
        0x1b => Key::Letter('s'),
        0x23 => Key::Letter('d'),
        0x2b => Key::Letter('f'),
        0x34 => Key::Letter('g'),
        0x33 => Key::Letter('h'),
        0x3b => Key::Letter('j'),
        0x42 => Key::Letter('k'),
        0x4b => Key::Letter('l'),
        0x1a => Key::Letter('z'),
        0x22 => Key::Letter('x'),
        0x21 => Key::Letter('c'),
        0x2a => Key::Letter('v'),
        0x32 => Key::Letter('b'),
        0x31 => Key::Letter('n'),
        0x3a => Key::Letter('m'),

        0x16 => Key::Letter('1'),
        0x1e => Key::Letter('2'),
        0x26 => Key::Letter('3'),
        0x25 => Key::Letter('4'),
        0x2e => Key::Letter('5'),
        0x36 => Key::Letter('6'),
        0x3d => Key::Letter('7'),
        0x3e => Key::Letter('8'),
        0x46 => Key::Letter('9'),
        0x45 => Key::Letter('0'),

        0x29 => Key::Space,
        0x5a => Key::Enter,
        0x12 => Key::LeftShift,
        0x59 => Key::RightShift,
        0x58 => Key::CapsLock,
        0xf0 => Key::Release,

        _ => Key::Unknown                      
    };
}