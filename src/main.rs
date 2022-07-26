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
use keebLib::codes::layer;

use std::collections::HashMap;
use std::cell::RefCell;

use keebLib::codes::Keys::*;
use keebLib::codes::EnumInt;
use keebLib::codes::extras::________;

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
    // PLANNING
    // the keymap will contain u32 values, 0-255 are keycodes. 
    // 256 - 288 are reserved for layers. top and bottom inclusive.
    // functions will be stored in a hashmap by u32 values over 288.
    let mut latestf = RefCell::new(0);
    let functions: RefCell<HashMap<u32, fn(kc: u32, down:bool)>> = RefCell::new(HashMap::new());
    let layer_function = |f: fn(kc: u32, down: bool)| -> u32 {
        let mut functions = functions.borrow_mut();
        let lat = latestf.borrow().to_owned();
        let id = 289+lat;
        functions.insert(id, f);
        latestf.replace(lat+1);
        return id;
    };

    let active_layer = RefCell::new(0);
    let set_layer = |l: u32| {
        let mut active_layer = active_layer.borrow_mut();
        *active_layer = l;
    };
    fn yes(kc: u32, down: bool) {
        println!("yes");
    }
    // TEMPORARILY MUTABLE, in the future it will probably be modified to be immutable and better
    let layer_main = 
       RefCell::new( [
        KC_Escape.i(),  KC_F1.i(),  KC_F2.i(),  KC_F3.i(),  KC_F4.i(),  KC_F5.i(),  KC_F6.i(),  KC_F7.i(),  KC_F8.i(),  KC_F9.i(),  KC_F10.i(),  KC_F11.i(),  KC_F12.i(),  KC_PrintScreen.i(),                             KC_AudioMute.i(),
        KC_Grave.i(),   KC_Num1.i(),KC_Num2.i(),KC_Num3.i(),KC_Num4.i(),KC_Num5.i(),KC_Num6.i(),KC_Num7.i(),KC_Num8.i(),KC_Num9.i(),KC_Num0.i(), KC_Minus.i(), KC_Equal.i(),  KC_Backspace.i(),                            KC_Delete.i(),
        KC_Tab.i(),     KC_Q.i(),   KC_W.i(),   KC_E.i(),   KC_R.i(),   KC_T.i(),   KC_Y.i(),   KC_U.i(),   KC_I.i(),   KC_O.i(),   KC_P.i(),    KC_LeftBracket.i(), KC_RightBracket.i(), KC_Backslash.i(),                KC_PageUp.i(),
        KC_CapsLock.i(),KC_A.i(),   KC_S.i(),   KC_D.i(),   KC_F.i(),   KC_G.i(),   KC_H.i(),   KC_J.i(),   KC_K.i(),   KC_L.i(),   KC_Semicolon.i(), KC_Quote.i(), KC_Enter.i(),                                          KC_PageDown.i(),
        KC_LeftShift.i(),  KC_Z.i(),   KC_X.i(),   KC_C.i(),   KC_V.i(),   KC_B.i(),   KC_N.i(),   KC_M.i(),   KC_Comma.i(), KC_Dot.i(),  KC_Slash.i(), KC_RightShift.i(),                 KC_Up.i(),                      KC_End.i(),
       KC_LeftCtrl.i(), KC_Menu.i(), KC_LeftAlt.i(),                 KC_Space.i(),                            KC_RightAlt.i(), layer_function(yes),   KC_RightCtrl.i(),                 KC_Left.i(), KC_Down.i(), KC_Right.i()
    ].iter().map(|&e| e as u32).collect::<Vec<u32>>());
    // TEMPORARILY THERE IS NO ORGANIZED SYSTEM FOR LAYERS
    // THE FN KEY IS AT INDEX 78, SO WE CAN MODIFY IT TO BE A LAYER activation key to activate layer 1
    let layer_1 = 
        RefCell::new([
        KC_Escape,  KC_F1,  KC_F2,  KC_F3,  KC_F4,  KC_F5,  KC_F6,  KC_F7,  KC_F8,  KC_F9,  KC_F10,  KC_F11,  KC_F12,  KC_PrintScreen,                                   KC_AudioMute,
        KC_Grave,   KC_Num1,KC_Num2,KC_Num3,KC_Num4,KC_Num5,KC_Num6,KC_Num7,KC_Num8,KC_Num9,KC_Num0, KC_Minus, KC_Equal,  KC_Backspace,                                  KC_Delete,
        KC_Tab,     KC_Q,   KC_W,   KC_E,   KC_R,   KC_T,   KC_Y,   KC_U,   KC_I,   KC_O,   KC_P,    KC_LeftBracket, KC_RightBracket, KC_Backslash,                      KC_PageUp,
        KC_CapsLock,KC_A,   KC_S,   KC_D,   KC_F,   KC_G,   KC_H,   KC_J,   KC_K,   KC_L,   KC_Semicolon, KC_Quote, KC_Enter,                                            KC_PageDown,
        KC_LeftShift,       KC_Z,   KC_X,   KC_C,   KC_V,   KC_B,   KC_N,   KC_M,   KC_Comma, KC_Dot,  KC_Slash, KC_RightShift,                        KC_Up,            KC_End,
       KC_LeftCtrl, KC_Menu, KC_LeftAlt,                 KC_Space,                            KC_RightAlt, KC_RollOver,   KC_RightCtrl,      KC_Left, KC_Down, KC_Right
    ].iter().map(|&e| e as u32).collect::<Vec<u32>>());
            
    // keys down counts everything as a key, functions, keycodes and layer keys alike
    let keys_down: RefCell<HashMap<u32, bool>> = RefCell::new(HashMap::new());
    let set_key_down = |keycode: u32, down: bool| {
        let mut keys_down = keys_down.borrow_mut();
        if keys_down.contains_key(&keycode) {
            keys_down.insert(keycode as u32, down);
        } else {
            keys_down.insert(keycode as u32, down);
        }
    };
    let is_key_down = |kc:u32| -> bool {
        let keys_down = keys_down.borrow();
        keys_down.get(&kc).unwrap_or_else(|| return &false) == &true
    };

    let km= vec![
        layer_main.borrow(),
        layer_1.borrow()
    ];
    
    let get_keycode_from_map = |kc:u32, map: &Vec<u32>|{
        let index = gmmk::pro::default_keymap().iter().position(|&r| r as u32 == kc).unwrap(); // gets the index of the pressed key in the keymap, pressed key is the physical key
        let key = map[index as usize]; // gets the keycode of the pressed key in the keymap, pressed key is the physical key
        (index, key)
    };
    let mut on_key_down = |kc: u32| -> () {
        // set_key_down(kc, true);
        // never register the physical key, which kc is
        // we need to go through processing before deciding what key presses do

        let layer_main = layer_main.borrow().to_owned();
        // Follow keymaps
        let (index, key) = get_keycode_from_map(kc, &layer_main);
        
        
        // This checks if the value of the key is a layer activation key or a function key, both of which are not u8 or keycodes, then if theyre not it will register the key as normal
        let main_mapped_keycode = layer_main[index as usize]; // this is the keycode of the pressed key in the keymap, kc is the physical key.
        if(main_mapped_keycode > 255) {
            if main_mapped_keycode > 255 && main_mapped_keycode <= 288 {
                set_key_down(main_mapped_keycode, true);
                // this is a layer key because it's within bounds off 256 and 288
                let layer_index = &layer_main[index as usize] - 256;
                let layer = &km[layer_index as usize].to_owned();
                set_layer(layer_index);
                
            }
            else if main_mapped_keycode > 288 {
                set_key_down(main_mapped_keycode, true);
                // this is a function pointer because it's greater than 288
                let fns = functions.borrow();
                let fun = fns.get(&main_mapped_keycode).unwrap();
                // running the function with parameters of the key going down
                let fns = functions.borrow();
                fns.get(&main_mapped_keycode).unwrap()(kc, true);
            }
        }
        else {
            // this *should* never panic because above we have checked for the possibilities of the layer key being out of an 8 bit number's bounds
            set_key_down(kc, true);
            reg_key(layer_main[index as usize].try_into().unwrap());
        }
        // Follow layers

    }; 
    let on_key_up = |kc: u32| {
        // set_key_down(kc, false);

        let layer_main = layer_main.borrow().to_owned();
        // Follow keymaps
        let (index, key) = get_keycode_from_map(kc, &layer_main);
        // // Unregister layer keys if the layer key is lifted
        // unreg_key(key as u8);

        // This checks if the value of the key is a layer activation key or a function key, both of which are not u8 or keycodes, then if theyre not it will register the key as normal
        let main_mapped_keycode = layer_main[index as usize]; // this is the keycode of the pressed key in the keymap, kc is the physical key.
        if(main_mapped_keycode > 255) {
            if main_mapped_keycode > 255 && main_mapped_keycode <= 288 {
                set_key_down(main_mapped_keycode, false);
                // this is a layer key because it's within bounds off 256 and 288
                let layer_index = &layer_main[index as usize] - 256;
                let layer = &km[layer_index as usize].to_owned();
                set_layer(layer_index);
                
            }
            else if main_mapped_keycode > 288 {
                set_key_down(main_mapped_keycode, false);
                // this is a function pointer because it's greater than 288
                let fns = functions.borrow();
                let fun = fns.get(&main_mapped_keycode).unwrap();
                // running the function with parameters of the key going down
                let fns = functions.borrow();
                fns.get(&main_mapped_keycode).unwrap()(kc, false);
            }
        }
        else {
            // this *should* never panic because above we have checked for the possibilities of the layer key being out of an 8 bit number's bounds
            set_key_down(kc, false);
            reg_key(layer_main[index as usize].try_into().unwrap());
        }

        // Soft exit keybind
        if is_key_down(KC_RollOver as u32) {
            if key as u8 == KC_Backslash as u8 {
                let mut exit = exit.borrow_mut();
                *exit = true;
                unreg_key(KC_RollOver as u8);
                unreg_key(KC_Backslash as u8);
            }
        }

        // Follow layers
        // index is the index of the pressed key in the keymap, pressed key is the physical key, which could differ from the keycode of the pressed key. the index is used to get the physical locaiton of differing keys
        
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
                on_key_down(key_code as u32);
            }
            2 => {
                // key released
                let key_code = payload[1];
                on_key_up(key_code as u32);
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
        println!("{:?}", keys_down.borrow());
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
