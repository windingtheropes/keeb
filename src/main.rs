#![allow(dead_code)]

// keep in note leds are not mapped and probably won't ever be since I do not use rgb on my keyboard.
// this is a simple representation of how I can contact my computer with my keyboard and do some hid tasks
// this coul;d be used for more advanced macros and all presses are sent here, unreliability aside from reading keypresses from the os
const VENDOR_ID: u16 = 0x320F;
const PRODUCT_ID: u16 = 0x5044;
use std::thread;
use std::time::{Duration, SystemTime};

mod keycodes;

use hidapi::HidDevice;
use keycodes::codes::*;

fn char_to_keycode(s: String) -> Vec<u8> {
    let s = s.as_str();
    let res: Vec<u8> = match s {
        // Lower Alphabet
        "a" => vec![StandardKeys::A as u8],
        "b" => vec![StandardKeys::B as u8],
        "c" => vec![StandardKeys::C as u8],
        "d" => vec![StandardKeys::D as u8],
        "e" => vec![StandardKeys::E as u8],
        "f" => vec![StandardKeys::F as u8],
        "g" => vec![StandardKeys::G as u8],
        "h" => vec![StandardKeys::H as u8],
        "i" => vec![StandardKeys::I as u8],
        "j" => vec![StandardKeys::J as u8],
        "k" => vec![StandardKeys::K as u8],
        "l" => vec![StandardKeys::L as u8],
        "m" => vec![StandardKeys::M as u8],
        "n" => vec![StandardKeys::N as u8],
        "o" => vec![StandardKeys::O as u8],
        "p" => vec![StandardKeys::P as u8],
        "q" => vec![StandardKeys::Q as u8],
        "r" => vec![StandardKeys::R as u8],
        "s" => vec![StandardKeys::S as u8],
        "t" => vec![StandardKeys::T as u8],
        "u" => vec![StandardKeys::U as u8],
        "v" => vec![StandardKeys::V as u8],
        "w" => vec![StandardKeys::W as u8],
        "x" => vec![StandardKeys::X as u8],
        "y" => vec![StandardKeys::Y as u8],
        "z" => vec![StandardKeys::Z as u8],
        // Upper Alphabet
        "A" => vec![StandardKeys::LeftShift as u8, StandardKeys::A as u8],
        "B" => vec![StandardKeys::LeftShift as u8, StandardKeys::B as u8],
        "C" => vec![StandardKeys::LeftShift as u8, StandardKeys::C as u8],
        "D" => vec![StandardKeys::LeftShift as u8, StandardKeys::D as u8],
        "E" => vec![StandardKeys::LeftShift as u8, StandardKeys::E as u8],
        "F" => vec![StandardKeys::LeftShift as u8, StandardKeys::F as u8],
        "G" => vec![StandardKeys::LeftShift as u8, StandardKeys::G as u8],
        "H" => vec![StandardKeys::LeftShift as u8, StandardKeys::H as u8],
        "I" => vec![StandardKeys::LeftShift as u8, StandardKeys::I as u8],
        "J" => vec![StandardKeys::LeftShift as u8, StandardKeys::J as u8],
        "K" => vec![StandardKeys::LeftShift as u8, StandardKeys::K as u8],
        "L" => vec![StandardKeys::LeftShift as u8, StandardKeys::L as u8],
        "M" => vec![StandardKeys::LeftShift as u8, StandardKeys::M as u8],
        "N" => vec![StandardKeys::LeftShift as u8, StandardKeys::N as u8],
        "O" => vec![StandardKeys::LeftShift as u8, StandardKeys::O as u8],
        "P" => vec![StandardKeys::LeftShift as u8, StandardKeys::P as u8],
        "Q" => vec![StandardKeys::LeftShift as u8, StandardKeys::Q as u8],
        "R" => vec![StandardKeys::LeftShift as u8, StandardKeys::R as u8],
        "S" => vec![StandardKeys::LeftShift as u8, StandardKeys::S as u8],
        "T" => vec![StandardKeys::LeftShift as u8, StandardKeys::T as u8],
        "U" => vec![StandardKeys::LeftShift as u8, StandardKeys::U as u8],
        "V" => vec![StandardKeys::LeftShift as u8, StandardKeys::V as u8],
        "W" => vec![StandardKeys::LeftShift as u8, StandardKeys::W as u8],
        "X" => vec![StandardKeys::LeftShift as u8, StandardKeys::X as u8],
        "Y" => vec![StandardKeys::LeftShift as u8, StandardKeys::Y as u8],
        "Z" => vec![StandardKeys::LeftShift as u8, StandardKeys::Z as u8],
        // Number row
        "1" => vec![StandardKeys::Num1 as u8],
        "2" => vec![StandardKeys::Num2 as u8],
        "3" => vec![StandardKeys::Num3 as u8],
        "4" => vec![StandardKeys::Num4 as u8],
        "5" => vec![StandardKeys::Num5 as u8],
        "6" => vec![StandardKeys::Num6 as u8],
        "7" => vec![StandardKeys::Num7 as u8],
        "8" => vec![StandardKeys::Num8 as u8],
        "9" => vec![StandardKeys::Num9 as u8],
        "0" => vec![StandardKeys::Num0 as u8],
        // Number row symbols
        "!" => vec![StandardKeys::LeftShift as u8, StandardKeys::Num1 as u8],
        "@" => vec![StandardKeys::LeftShift as u8, StandardKeys::Num2 as u8],
        "#" => vec![StandardKeys::LeftShift as u8, StandardKeys::Num3 as u8],
        "$" => vec![StandardKeys::LeftShift as u8, StandardKeys::Num4 as u8],
        "%" => vec![StandardKeys::LeftShift as u8, StandardKeys::Num5 as u8],
        "^" => vec![StandardKeys::LeftShift as u8, StandardKeys::Num6 as u8],
        "&" => vec![StandardKeys::LeftShift as u8, StandardKeys::Num7 as u8],
        "*" => vec![StandardKeys::LeftShift as u8, StandardKeys::Num8 as u8],
        "(" => vec![StandardKeys::LeftShift as u8, StandardKeys::Num9 as u8],
        ")" => vec![StandardKeys::LeftShift as u8, StandardKeys::Num0 as u8],
        // Other characters
        "`" => vec![StandardKeys::Grave as u8],
        "," => vec![StandardKeys::Comma as u8],
        "." => vec![StandardKeys::Dot as u8],
        "/" => vec![StandardKeys::Slash as u8],
        "\'" => vec![StandardKeys::Quote as u8],
        ";" => vec![StandardKeys::Semicolon as u8],
        "\\" => vec![StandardKeys::Backslash as u8],
        "[" => vec![StandardKeys::LeftBracket as u8],
        "]" => vec![StandardKeys::RightBracket as u8],
        "=" => vec![StandardKeys::Equal as u8],
        "-" => vec![StandardKeys::Minus as u8],
        // Other characters shifted
        "~" => vec![StandardKeys::LeftShift as u8, StandardKeys::Grave as u8],
        "<" => vec![StandardKeys::LeftShift as u8, StandardKeys::Comma as u8],
        ">" => vec![StandardKeys::LeftShift as u8, StandardKeys::Dot as u8],
        "?" => vec![StandardKeys::LeftShift as u8, StandardKeys::Slash as u8],
        "\"" => vec![StandardKeys::LeftShift as u8, StandardKeys::Quote as u8],
        ":" => vec![StandardKeys::LeftShift as u8, StandardKeys::Semicolon as u8],
        "|" => vec![StandardKeys::LeftShift as u8, StandardKeys::Backslash as u8],
        "{" => vec![
            StandardKeys::LeftShift as u8,
            StandardKeys::LeftBracket as u8,
        ],
        "}" => vec![
            StandardKeys::LeftShift as u8,
            StandardKeys::RightBracket as u8,
        ],
        "+" => vec![StandardKeys::LeftShift as u8, StandardKeys::Equal as u8],
        "_" => vec![StandardKeys::LeftShift as u8, StandardKeys::Minus as u8],
        // "*" => vec![StandardKeys::KpAsterisk as u8],
        " " => vec![StandardKeys::Space as u8],
        // Otherwise it's 0
        _ => vec![StandardKeys::No as u8],
    };
    res
}

fn keycode_as_int<T: EnumInt>(e: T) -> u8 {
    e.as_int()
}

fn string_to_keycode(s: String) -> Vec<Vec<u8>> {
    let char_vec: Vec<char> = s.chars().collect();
    let mut chars: Vec<Vec<u8>> = vec![];
    for c in char_vec {
        chars.push(char_to_keycode(c.to_string()));
    }
    chars
}
fn main() {
    let api = hidapi::HidApi::new().unwrap();
    let device = api.open(VENDOR_ID, PRODUCT_ID).unwrap();

    let set_color = |led: u8, r: u8, g: u8, b: u8| {
        let buf = [0x1, 1, led, r, g, b];
        device.write(&buf).unwrap();
    };

    let set_color_all = |r: u8, g: u8, b: u8| {
        let buf = [0x1, 2, r, g, b];
        device.write(&buf).unwrap();
    };

    let set_color_side = |side: u8, r: u8, g: u8, b: u8| {
        let buf = [0x1, 3, side, r, g, b];
        device.write(&buf).unwrap();
    };

    let tap_code = |kc: u8| {
        let buf = [0x1, 4, kc];
        device.write(&buf).unwrap();
    };

    let reg_key = |kc: u8| {
        let buf = [0x1, 5, 1, kc];
        device.write(&buf).unwrap();
    };

    let unreg_key = |kc: u8| {
        let buf = [0x1, 5, 0, kc];
        device.write(&buf).unwrap();
    };

    let type_string = |s: String| {
        let codes = string_to_keycode(s);
        for i in codes {
            for u in &i {
                reg_key(u.to_owned());
            }
            for u in &i {
                unreg_key(u.to_owned());
            }
        }
    };
    // let blink_key = |led: u8, r:u8, g:u8, b:u8, ms: u64| {
    //     thread::spawn(|| {
    //         set_color(led, r, g, b);
    //         thread::sleep(Duration::from_millis(ms));
    //         set_color(led, 0, 0, 0);
    //     })
    // };

    // Writing data to the device
    // ********
    // let buf = [0x1, 1, 0];
    // device.write(&buf).unwrap();
    // ********
    // the length of the payload must always be one more byte than the length of the actual desired readable payload, by preceeding it with a 0x1 byte
    // an example payload
    // [1, 1, 255, 255, 255]
    // [method, param, param, param, param, etc]
    // methods
    // 1: single rgb | [1, 45, 255, 255, 255] turns key 45 to white
    // 2: all rgb | [2, 255, 255, 255] turns the entire keyboard lighting to white
    // 3: side rgb | [3, 0, 255, 255, 255] 0 = left, 1 = right, this turns the left side to white
    // 4: high level key manipulation; key tap | [4, 0] taps keycode 0
    // 5: low level key manipulation; key register/unregister | [5, 1, 0] registers keycode 1. input registered until unregistered.

    // Reading data from the device
    // ********
    // let mut buf = [0u8; 2];
    // let res = device.read(&mut buf[..]).unwrap();
    // let payload = &buf[..res];
    // ********
    // payload structure is the same, except separated into types and data instead of methods
    // [1, 100]
    // [type, param]
    // types
    // 1: key pressed | [1, 45] key 45 was pressed down
    // 2: key released | [1, 45] key 45 was released. this is the most common area to key key input.
    // 3: rotary encoder turned | [1, 1] the rotary encoder was turned clockwise

    loop {
        let mut buf = [0u8; 2];
        let res = device.read(&mut buf[..]).unwrap();
        let payload = &buf[..res];
        match payload[0] {
            1 => {
                // key pressed
            }
            2 => {
                // key released
                let key_code = payload[1];
                let led_code = GmmkProLed::key_to_led(StandardKeys::get_key(key_code).unwrap());
                if key_code == StandardKeys::RollOver as u8 {
                    set_color(led_code.unwrap(), 255, 0, 0);
                } else {
                }
            }
            _ => {}
        }
    }
}
