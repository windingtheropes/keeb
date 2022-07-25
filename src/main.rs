// keep in note leds are not mapped and probably won't ever be since I do not use rgb on my keyboard.
// this is a simple representation of how I can contact my computer with my keyboard and do some hid tasks
// this coul;d be used for more advanced macros and all presses are sent here, unreliability aside from reading keypresses from the os

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
    // 6: intercept mode: will disable sending keypresses to the operating system, to be intercepted by this program; turn on/off | [6, 1] turns on intercept mode
    // 7: ping function | [7] reminds the keyboard that the program is still running

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

const VENDOR_ID: u16 = 0x320F;
const PRODUCT_ID: u16 = 0x5044;

use hidapi;

use std::collections::HashMap;
use std::cell::RefCell;

use keebLib::codes::Keys::*;
use keebLib::codes::EnumInt;

fn char_to_keycode(s: String) -> Vec<u8> {
    let s = s.as_str();
    let res: Vec<u8> = match s {
        // Lower Alphabet
        "a" => vec![KC_A as u8],
        "b" => vec![KC_B as u8],
        "c" => vec![KC_C as u8],
        "d" => vec![KC_D as u8],
        "e" => vec![KC_E as u8],
        "f" => vec![KC_F as u8],
        "g" => vec![KC_G as u8],
        "h" => vec![KC_H as u8],
        "i" => vec![KC_I as u8],
        "j" => vec![KC_J as u8],
        "k" => vec![KC_K as u8],
        "l" => vec![KC_L as u8],
        "m" => vec![KC_M as u8],
        "n" => vec![KC_N as u8],
        "o" => vec![KC_O as u8],
        "p" => vec![KC_P as u8],
        "q" => vec![KC_Q as u8],
        "r" => vec![KC_R as u8],
        "s" => vec![KC_S as u8],
        "t" => vec![KC_T as u8],
        "u" => vec![KC_U as u8],
        "v" => vec![KC_V as u8],
        "w" => vec![KC_W as u8],
        "x" => vec![KC_X as u8],
        "y" => vec![KC_Y as u8],
        "z" => vec![KC_Z as u8],
        // Upper Alphabet
        "A" => vec![KC_LeftShift as u8, KC_A as u8],
        "B" => vec![KC_LeftShift as u8, KC_B as u8],
        "C" => vec![KC_LeftShift as u8, KC_C as u8],
        "D" => vec![KC_LeftShift as u8, KC_D as u8],
        "E" => vec![KC_LeftShift as u8, KC_E as u8],
        "F" => vec![KC_LeftShift as u8, KC_F as u8],
        "G" => vec![KC_LeftShift as u8, KC_G as u8],
        "H" => vec![KC_LeftShift as u8, KC_H as u8],
        "I" => vec![KC_LeftShift as u8, KC_I as u8],
        "J" => vec![KC_LeftShift as u8, KC_J as u8],
        "K" => vec![KC_LeftShift as u8, KC_K as u8],
        "L" => vec![KC_LeftShift as u8, KC_L as u8],
        "M" => vec![KC_LeftShift as u8, KC_M as u8],
        "N" => vec![KC_LeftShift as u8, KC_N as u8],
        "O" => vec![KC_LeftShift as u8, KC_O as u8],
        "P" => vec![KC_LeftShift as u8, KC_P as u8],
        "Q" => vec![KC_LeftShift as u8, KC_Q as u8],
        "R" => vec![KC_LeftShift as u8, KC_R as u8],
        "S" => vec![KC_LeftShift as u8, KC_S as u8],
        "T" => vec![KC_LeftShift as u8, KC_T as u8],
        "U" => vec![KC_LeftShift as u8, KC_U as u8],
        "V" => vec![KC_LeftShift as u8, KC_V as u8],
        "W" => vec![KC_LeftShift as u8, KC_W as u8],
        "X" => vec![KC_LeftShift as u8, KC_X as u8],
        "Y" => vec![KC_LeftShift as u8, KC_Y as u8],
        "Z" => vec![KC_LeftShift as u8, KC_Z as u8],
        // Number row
        "1" => vec![KC_Num1 as u8],
        "2" => vec![KC_Num2 as u8],
        "3" => vec![KC_Num3 as u8],
        "4" => vec![KC_Num4 as u8],
        "5" => vec![KC_Num5 as u8],
        "6" => vec![KC_Num6 as u8], 
        "7" => vec![KC_Num7 as u8],
        "8" => vec![KC_Num8 as u8],
        "9" => vec![KC_Num9 as u8],
        "0" => vec![KC_Num0 as u8],
        // Number row symbols
        "!" => vec![KC_LeftShift as u8, KC_Num1 as u8],
        "@" => vec![KC_LeftShift as u8, KC_Num2 as u8],
        "#" => vec![KC_LeftShift as u8, KC_Num3 as u8],
        "$" => vec![KC_LeftShift as u8, KC_Num4 as u8],
        "%" => vec![KC_LeftShift as u8, KC_Num5 as u8],
        "^" => vec![KC_LeftShift as u8, KC_Num6 as u8],
        "&" => vec![KC_LeftShift as u8, KC_Num7 as u8],
        "*" => vec![KC_LeftShift as u8, KC_Num8 as u8],
        "(" => vec![KC_LeftShift as u8, KC_Num9 as u8],
        ")" => vec![KC_LeftShift as u8, KC_Num0 as u8],
        // Other characters
        "`" => vec![KC_Grave as u8],
        "," => vec![KC_Comma as u8],
        "." => vec![KC_Dot as u8],
        "/" => vec![KC_Slash as u8],
        "\'" => vec![KC_Quote as u8],
        ";" => vec![KC_Semicolon as u8],
        "\\" => vec![KC_Backslash as u8],
        "[" => vec![KC_LeftBracket as u8],
        "]" => vec![KC_RightBracket as u8],
        "=" => vec![KC_Equal as u8],
        "-" => vec![KC_Minus as u8],
        // Other characters shifted
        "~" => vec![KC_LeftShift as u8, KC_Grave as u8],
        "<" => vec![KC_LeftShift as u8, KC_Comma as u8],
        ">" => vec![KC_LeftShift as u8, KC_Dot as u8],
        "?" => vec![KC_LeftShift as u8, KC_Slash as u8],
        "\"" => vec![KC_LeftShift as u8, KC_Quote as u8],
        ":" => vec![KC_LeftShift as u8, KC_Semicolon as u8],
        "|" => vec![KC_LeftShift as u8, KC_Backslash as u8],
        "{" => vec![
            KC_LeftShift as u8,
            KC_LeftBracket as u8,
        ],
        "}" => vec![
            KC_LeftShift as u8,
            KC_RightBracket as u8,
        ],
        "+" => vec![KC_LeftShift as u8, KC_Equal as u8],
        "_" => vec![KC_LeftShift as u8, KC_Minus as u8],
        // "*" => vec![KpAsterisk as u8],
        " " => vec![KC_Space as u8],
        // Otherwise it's 0
        _ => vec![KC_No as u8],
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
    println!("Keeb started");
    let mut exit = RefCell::new(false);
    // the default keymap

    // the keyboard keymap
    let keymap = [
        [
            1
        ]
    ];

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

    let conditional_set_color = |led: u8, r: u8, g: u8, b: u8| {
        let buf = [0x1, 7, led, r, g, b];
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

    let intercept_mode = |on: u8| {
        let buf = [0x1, 6, on];
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

    let intercepts = |kc: u8, e: u8| {
        // if kc == Escape as u8
        // {   
        //     if e == 0 {
        //         intercept_mode(1);
        //         reg_key(A as u8);
        //     } else {
        //         intercept_mode(0);
        //         unreg_key(A as u8);
        //     }
        // }
    };

    let layer_main = 
        [
        KC_Escape,  KC_F1,  KC_F2,  KC_F3,  KC_F4,  KC_F5,  KC_F6,  KC_F7,  KC_F8,  KC_F9,  KC_F10,  KC_F11,  KC_F12,  KC_PrintScreen,                                   KC_AudioMute,
        KC_Grave,   KC_Num1,KC_Num2,KC_Num3,KC_Num4,KC_Num5,KC_Num6,KC_Num7,KC_Num8,KC_Num9,KC_Num0, KC_Minus, KC_Equal,  KC_Backspace,                                  KC_Delete,
        KC_Tab,     KC_Q,   KC_W,   KC_E,   KC_R,   KC_T,   KC_Y,   KC_U,   KC_I,   KC_O,   KC_P,    KC_LeftBracket, KC_RightBracket, KC_Backslash,                      KC_PageUp,
        KC_CapsLock,KC_A,   KC_S,   KC_D,   KC_F,   KC_G,   KC_H,   KC_J,   KC_K,   KC_L,   KC_Semicolon, KC_Quote, KC_Enter,                                            KC_PageDown,
        KC_LeftShift,       KC_Z,   KC_X,   KC_C,   KC_V,   KC_B,   KC_N,   KC_M,   KC_Comma, KC_Dot,  KC_Slash, KC_RightShift,                        KC_Up,            KC_End,
       KC_LeftCtrl, KC_Menu, KC_LeftAlt,                 KC_Space,                            KC_RightAlt, KC_RollOver,   KC_RightCtrl,      KC_Left, KC_Down, KC_Right
    ].iter().map(|&e| e as u32).collect::<Vec<u32>>();
    
    let keys_down = RefCell::new(HashMap::new());
    let set_key_down = |keycode: u8, down: bool| {
        let mut keys_down = keys_down.borrow_mut();
        if keys_down.contains_key(&keycode) {
            keys_down.insert(keycode, down);
        } else {
            keys_down.insert(keycode, down);
        }
    };
    let is_key_down = |kc:u8| -> bool {
        let keys_down = keys_down.borrow();
        keys_down.get(&kc).unwrap_or_else(|| return &false) == &true
    };

    let km: Vec<Vec<u32>> = vec![
        layer_main
    ];
    
    let get_keycode_from_map = |kc:u8, map: &Vec<u32>|{
        let index = gmmk::pro::default_keymap().iter().position(|&r| r as u8 == kc).unwrap(); // gets the index of the pressed key in the keymap, pressed key is the physical key
        let key = map[index as usize]; // gets the keycode of the pressed key in the keymap, pressed key is the physical key
        (index, key)
    };

    let mut on_key_down = |kc: u8| {
        set_key_down(kc, true);

        // Follow keymaps
        let (index, key) = get_keycode_from_map(kc, &km[0]);
        reg_key(key as u8);
    }; 
    let on_key_up = |kc: u8| {
        set_key_down(kc, false);

        // Follow keymaps
        let (index, key) = get_keycode_from_map(kc, &km[0]);
        // Unregister layer keys if the layer key is lifted
        unreg_key(key as u8);

        if is_key_down(KC_RollOver as u8) {
            if key as u8 == KC_Backslash as u8 {
                let mut exit = exit.borrow_mut();
                *exit = true;
                unreg_key(KC_RollOver as u8);
                unreg_key(KC_Backslash as u8);
            }
        }
    };

    let on_knob_turned = |cw:bool|{
        match cw {
            true => tap_code(KC_AudioVolUp as u8),
            false => tap_code(KC_AudioVolDown as u8),
        }
    };

    let on_keyboard_event = |payload: &[u8]| {
        match payload[0] {
            1 => {
                // key pressed
                let key_code = payload[1];
                on_key_down(key_code);
            }
            2 => {
                // key released
                let key_code = payload[1];
                on_key_up(key_code);
            }
            3 => {
                // rotary 
                let cw = payload[1] == 1;
                on_knob_turned(cw);
            }
            _ => {}
        }
    };

    intercept_mode(1);
    // input loop
    loop {
        let exit = exit.borrow().to_owned();
        if exit == true { intercept_mode(0); break; }
        // receiver configuration
        let mut buf = [0u8; 2];
        let res = device.read(&mut buf[..]).unwrap();
        // received data from the keyboard
        let payload = &buf[..res];
        // parse the payload
        if payload[0] != 0 {
            // this means there was some kind of event
            on_keyboard_event(payload);
        }
    }
    println!("Keeb exiting...");
}
