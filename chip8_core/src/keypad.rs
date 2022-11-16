// use maplit::hashmap;
// Maybe don't use this, instead commit to using bevy for everything
//
/*
pub static DEFAULT_CONFIG: Lazy<KeyMap> = Lazy::new(|| {
    use Key::*;
    KeyMap {
        map: hashmap! {
            Key0 => vec!["0".to_string(), "`".to_string()],
            Key1 => vec!["1".to_string()],
            Key2 => vec!["2".to_string()],
            Key3 => vec!["3".to_string()],
            Key4 => vec!["4".to_string()],
            Key5 => vec!["5".to_string()],
            Key6 => vec!["6".to_string()],
            Key7 => vec!["7".to_string()],
            Key8 => vec!["8".to_string()],
            Key9 => vec!["9".to_string()],
            KeyA => vec!["q".to_string()],
            KeyB => vec!["w".to_string()],
            KeyC => vec!["e".to_string()],
            KeyD => vec!["r".to_string()],
            KeyE => vec!["t".to_string()],
            KeyF => vec!["y".to_string()]
        },
        reverse: None,
    }
});
*/

use enum_primitive_derive::Primitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Primitive)]
#[repr(u8)]
pub enum Key {
    Key0 = 0x0,
    Key1 = 0x1,
    Key2 = 0x2,
    Key3 = 0x3,
    Key4 = 0x4,
    Key5 = 0x5,
    Key6 = 0x6,
    Key7 = 0x7,
    Key8 = 0x8,
    Key9 = 0x9,
    KeyA = 0xA,
    KeyB = 0xB,
    KeyC = 0xC,
    KeyD = 0xD,
    KeyE = 0xE,
    KeyF = 0xF,
}

// impl From<u8> for Key {
//     fn from(value: u8) -> Self {
//         use Key::*;
//         match value {
//             0x0 => Key0,
//             0x1 => Key1,
//             0x2 => Key2,
//             0x3 => Key3,
//             0x4 => Key4,
//             0x5 => Key5,
//             0x6 => Key6,
//             0x7 => Key7,
//             0x8 => Key8,
//             0x9 => Key9,
//             0xA => KeyA,
//             0xB => KeyB,
//             0xC => KeyC,
//             0xD => KeyD,
//             0xE => KeyE,
//             0xF => KeyF,
//         }
//     }
// }

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Pressed,
    #[default]
    NotPressed,
}

/***
 * 1 	2 	3 	C
 * 4 	5 	6 	D
 * 7 	8 	9 	E
 * A 	0 	B 	F
 */

pub struct Keypad {
    state: [KeyState; 16],
}

impl Default for Keypad {
    fn default() -> Self {
        Keypad::new()
    }
}

impl Keypad {
    fn new() -> Self {
        Self {
            state: [KeyState::default(); 16],
        }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.get_key(key) == KeyState::Pressed
    }

    pub fn get_key(&self, key: Key) -> KeyState {
        self.state[key as usize]
    }

    pub fn set_key(&mut self, key: Key, state: KeyState) -> bool {
        let old_state = self.state[key as usize];
        self.state[key as usize] = state;
        old_state != state
    }
}
