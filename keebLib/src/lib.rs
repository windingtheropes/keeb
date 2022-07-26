pub mod codes {
    use strum::IntoEnumIterator; // 0.17.1
    pub use strum_macros::EnumIter;
    // note:
    // Num1 is 1 in the number row
    // Kp1 is 1 in the keypad
    #[derive(Debug, EnumIter, Copy, Clone)]
    #[allow(non_camel_case_types)]
    pub enum Keys {
        KC_No = 0x00,
        KC_RollOver = 1,
        KC_PostFail = 2,
        KC_Undefined = 3,
        KC_A = 4,
        KC_B = 5,
        KC_C = 6,
        KC_D = 7,
        KC_E = 8,
        KC_F = 9,
        KC_G = 10,
        KC_H = 11,
        KC_I = 12,
        KC_J = 13,
        KC_K = 14,
        KC_L = 15,
        KC_M = 16, // 0x10
        KC_N = 17,
        KC_O = 18,
        KC_P = 19,
        KC_Q = 20,
        KC_R = 21,
        KC_S = 22,
        KC_T = 23,
        KC_U = 24,
        KC_V = 25,
        KC_W = 26,
        KC_X = 27,
        KC_Y = 28,
        KC_Z = 29,
        KC_Num1 = 30,
        KC_Num2 = 31,
        KC_Num3 = 32, // 0x20
        KC_Num4 = 33,
        KC_Num5 = 34,
        KC_Num6 = 35,
        KC_Num7 = 36,
        KC_Num8 = 37,
        KC_Num9 = 38,
        KC_Num0 = 39,
        KC_Enter = 40,
        KC_Escape = 41,
        KC_Backspace = 42,
        KC_Tab = 43,
        KC_Space = 44,
        KC_Minus = 45,
        KC_Equal = 46,
        KC_LeftBracket = 47,
        KC_RightBracket = 48, // 0x30
        KC_Backslash = 49,
        KC_NonusHash = 50,
        KC_Semicolon = 51,
        KC_Quote = 52,
        KC_Grave = 53,
        KC_Comma = 54,
        KC_Dot = 55,
        KC_Slash = 56,
        KC_CapsLock = 57,
        KC_F1 = 58,
        KC_F2 = 59,
        KC_F3 = 60,
        KC_F4 = 61,
        KC_F5 = 62,
        KC_F6 = 63,
        KC_F7 = 64, // 0x40
        KC_F8 = 65,
        KC_F9 = 66,
        KC_F10 = 67,
        KC_F11 = 68,
        KC_F12 = 69,
        KC_PrintScreen = 70,
        KC_ScrollLock = 71,
        KC_Pause = 72,
        KC_Insert = 73,
        KC_Home = 74,
        KC_PageUp = 75,
        KC_Delete = 76,
        KC_End = 77,
        KC_PageDown = 78,
        KC_Right = 79,
        KC_Left = 80, // 0x50
        KC_Down = 81,
        KC_Up = 82,
        KC_NumLock = 83,
        KC_KpSlash = 84,
        KC_KpAsterisk = 85,
        KC_KpMinus = 86,
        KC_KpPlus = 87,
        KC_KpEnter = 88,
        KC_Kp1 = 89,
        KC_Kp2 = 90,
        KC_Kp3 = 91,
        KC_Kp4 = 92,
        KC_Kp5 = 93,
        KC_Kp6 = 94,
        KC_Kp7 = 95,
        KC_Kp8 = 96, // 0x60
        KC_Kp9 = 97,
        KC_Kp0 = 98,
        KC_KpDot = 99,
        KC_NonusBackslash = 100,
        KC_Application = 101,
        KC_KbPower = 102,
        KC_KpEqual = 103,
        KC_F13 = 104,
        KC_F14 = 105,
        KC_F15 = 106,
        KC_F16 = 107,
        KC_F17 = 108,
        KC_F18 = 109,
        KC_F19 = 110,
        KC_F20 = 111,
        KC_F21 = 112, // 0x70
        KC_F22 = 113,
        KC_F23 = 114,
        KC_F24 = 115,
        KC_Execute = 116,
        KC_Help = 117,
        KC_Menu = 118,
        KC_Select = 119,
        KC_Stop = 120,
        KC_Again = 121,
        KC_Undo = 122,
        KC_Cut = 123,
        KC_Copy = 124,
        KC_Paste = 125,
        KC_Find = 126,
        KC_KbMute = 127,
        KC_KbVolumeUp = 128, // 0x80
        KC_KbVolumeDown = 129,
        KC_LockingCapsLock = 130,
        KC_LockingNumLock = 131,
        KC_LockingScrollLock = 132,
        KC_KpComma = 133,
        KC_KpEqualAs400 = 134,
        KC_International1 = 135,
        KC_International2 = 136,
        KC_International3 = 137,
        KC_International4 = 138,
        KC_International5 = 139,
        KC_International6 = 140,
        KC_International7 = 141,
        KC_International8 = 142,
        KC_International9 = 143,
        KC_Language1 = 144, // 0x90
        KC_Language2 = 145,
        KC_Language3 = 146,
        KC_Language4 = 147,
        KC_Language5 = 148,
        KC_Language6 = 149,
        KC_Language7 = 150,
        KC_Language8 = 151,
        KC_Language9 = 152,
        KC_AlternateErase = 153,
        KC_SystemRequest = 154,
        KC_Cancel = 155,
        KC_Clear = 156,
        KC_Prior = 157,
        KC_Return = 158,
        KC_Separator = 159,
        KC_Out = 160, // 0xA0
        KC_Oper = 161,
        KC_ClearAgain = 162,
        KC_Crsel = 163,
        KC_Exsel = 164,

        // special keys
        /* Generic Desktop Page (0x01) */
        KC_SystemPower = 0xA5, //165
        KC_SystemSleep = 166,
        KC_SystemWake = 167,

        /* Consumer Page (0x0C) */
        KC_AudioMute = 168,
        KC_AudioVolUp = 169,
        KC_AudioVolDown = 170,
        KC_MediaNextTrack = 171,
        KC_MediaPrevTrack = 172,
        KC_MediaStop = 173,
        KC_MediaPlayPause = 174,
        KC_MediaSelect = 175,
        KC_MediaEject = 176, // 0xB0
        KC_Mail = 177,
        KC_Calculator = 178,
        KC_MyComputer = 179,
        KC_WwwSearch = 180,
        KC_WwwHome = 181,
        KC_WwwBack = 182,
        KC_WwwForward = 183,
        KC_WwwStop = 184,
        KC_WwwRefresh = 185,
        KC_WwwFavorites = 186,
        KC_MediaFastForward = 187,
        KC_MediaRewind = 188,
        KC_BrightnessUp = 189,
        KC_BrightnessDown = 190,
        // end special keys
        /* Modifiers */
        KC_LeftCtrl = 0xE0, // 224
        KC_LeftShift = 225,
        KC_LeftAlt = 226,
        KC_LeftGui = 227,
        KC_RightCtrl = 228,
        KC_RightShift = 229,
        KC_RightAlt = 230,
        KC_RightGui = 231,

        // **********************************************
        // * 0xF0-0xFF are unallocated in the HID spec. *
        // * QMK uses these for Mouse Keys - see below. *
        // **********************************************
        /* Mouse Buttons */
        KC_MsUp = 0xED, // 237
        KC_MsDown = 238,
        KC_MsLeft = 239,
        KC_MsRight = 0xF0, // 240
        KC_MsBtn1 = 241,
        KC_MsBtn2 = 242,
        KC_MsBtn3 = 243,
        KC_MsBtn4 = 244,
        KC_MsBtn5 = 245,
        KC_MsBtn6 = 246,
        KC_MsBtn7 = 247,
        KC_MsBtn8 = 248,

        /* Mouse Wheel */
        KC_MsWhUp = 249,
        KC_MsWhDown = 250,
        KC_MsWhLeft = 251,
        KC_MsWhRight = 252,

        /* Acceleration */
        KC_MsAccel0 = 253,
        KC_MsAccel1 = 254,
        KC_MsAccel2 = 255, // 0xFF
    }

    impl Keys {
        pub fn get_key(k: u8) -> Option<Keys> {
            for key in Keys::iter() {
                if key as u8 == k {
                    return Some(key);
                }
            }
            None
        }
    }

    pub fn char_to_keycode(s: String) -> Vec<u8> {
        let s = s.as_str();
        let res: Vec<u8> = match s {
            // Lower Alphabet
            "a" => vec![Keys::KC_A as u8],
            "b" => vec![Keys::KC_B as u8],
            "c" => vec![Keys::KC_C as u8],
            "d" => vec![Keys::KC_D as u8],
            "e" => vec![Keys::KC_E as u8],
            "f" => vec![Keys::KC_F as u8],
            "g" => vec![Keys::KC_G as u8],
            "h" => vec![Keys::KC_H as u8],
            "i" => vec![Keys::KC_I as u8],
            "j" => vec![Keys::KC_J as u8],
            "k" => vec![Keys::KC_K as u8],
            "l" => vec![Keys::KC_L as u8],
            "m" => vec![Keys::KC_M as u8],
            "n" => vec![Keys::KC_N as u8],
            "o" => vec![Keys::KC_O as u8],
            "p" => vec![Keys::KC_P as u8],
            "q" => vec![Keys::KC_Q as u8],
            "r" => vec![Keys::KC_R as u8],
            "s" => vec![Keys::KC_S as u8],
            "t" => vec![Keys::KC_T as u8],
            "u" => vec![Keys::KC_U as u8],
            "v" => vec![Keys::KC_V as u8],
            "w" => vec![Keys::KC_W as u8],
            "x" => vec![Keys::KC_X as u8],
            "y" => vec![Keys::KC_Y as u8],
            "z" => vec![Keys::KC_Z as u8],
            // Upper Alphabet
            "A" => vec![Keys::KC_LeftShift as u8, Keys::KC_A as u8],
            "B" => vec![Keys::KC_LeftShift as u8, Keys::KC_B as u8],
            "C" => vec![Keys::KC_LeftShift as u8, Keys::KC_C as u8],
            "D" => vec![Keys::KC_LeftShift as u8, Keys::KC_D as u8],
            "E" => vec![Keys::KC_LeftShift as u8, Keys::KC_E as u8],
            "F" => vec![Keys::KC_LeftShift as u8, Keys::KC_F as u8],
            "G" => vec![Keys::KC_LeftShift as u8, Keys::KC_G as u8],
            "H" => vec![Keys::KC_LeftShift as u8, Keys::KC_H as u8],
            "I" => vec![Keys::KC_LeftShift as u8, Keys::KC_I as u8],
            "J" => vec![Keys::KC_LeftShift as u8, Keys::KC_J as u8],
            "K" => vec![Keys::KC_LeftShift as u8, Keys::KC_K as u8],
            "L" => vec![Keys::KC_LeftShift as u8, Keys::KC_L as u8],
            "M" => vec![Keys::KC_LeftShift as u8, Keys::KC_M as u8],
            "N" => vec![Keys::KC_LeftShift as u8, Keys::KC_N as u8],
            "O" => vec![Keys::KC_LeftShift as u8, Keys::KC_O as u8],
            "P" => vec![Keys::KC_LeftShift as u8, Keys::KC_P as u8],
            "Q" => vec![Keys::KC_LeftShift as u8, Keys::KC_Q as u8],
            "R" => vec![Keys::KC_LeftShift as u8, Keys::KC_R as u8],
            "S" => vec![Keys::KC_LeftShift as u8, Keys::KC_S as u8],
            "T" => vec![Keys::KC_LeftShift as u8, Keys::KC_T as u8],
            "U" => vec![Keys::KC_LeftShift as u8, Keys::KC_U as u8],
            "V" => vec![Keys::KC_LeftShift as u8, Keys::KC_V as u8],
            "W" => vec![Keys::KC_LeftShift as u8, Keys::KC_W as u8],
            "X" => vec![Keys::KC_LeftShift as u8, Keys::KC_X as u8],
            "Y" => vec![Keys::KC_LeftShift as u8, Keys::KC_Y as u8],
            "Z" => vec![Keys::KC_LeftShift as u8, Keys::KC_Z as u8],
            // Number row
            "1" => vec![Keys::KC_Num1 as u8],
            "2" => vec![Keys::KC_Num2 as u8],
            "3" => vec![Keys::KC_Num3 as u8],
            "4" => vec![Keys::KC_Num4 as u8],
            "5" => vec![Keys::KC_Num5 as u8],
            "6" => vec![Keys::KC_Num6 as u8], 
            "7" => vec![Keys::KC_Num7 as u8],
            "8" => vec![Keys::KC_Num8 as u8],
            "9" => vec![Keys::KC_Num9 as u8],
            "0" => vec![Keys::KC_Num0 as u8],
            // Number row symbols
            "!" => vec![Keys::KC_LeftShift as u8, Keys::KC_Num1 as u8],
            "@" => vec![Keys::KC_LeftShift as u8, Keys::KC_Num2 as u8],
            "#" => vec![Keys::KC_LeftShift as u8, Keys::KC_Num3 as u8],
            "$" => vec![Keys::KC_LeftShift as u8, Keys::KC_Num4 as u8],
            "%" => vec![Keys::KC_LeftShift as u8, Keys::KC_Num5 as u8],
            "^" => vec![Keys::KC_LeftShift as u8, Keys::KC_Num6 as u8],
            "&" => vec![Keys::KC_LeftShift as u8, Keys::KC_Num7 as u8],
            "*" => vec![Keys::KC_LeftShift as u8, Keys::KC_Num8 as u8],
            "(" => vec![Keys::KC_LeftShift as u8, Keys::KC_Num9 as u8],
            ")" => vec![Keys::KC_LeftShift as u8, Keys::KC_Num0 as u8],
            // Other characters
            "`" => vec![Keys::KC_Grave as u8],
            "," => vec![Keys::KC_Comma as u8],
            "." => vec![Keys::KC_Dot as u8],
            "/" => vec![Keys::KC_Slash as u8],
            "\'"=> vec![Keys::KC_Quote as u8],
            ";" => vec![Keys::KC_Semicolon as u8],
            "\\"=> vec![Keys::KC_Backslash as u8],
            "[" => vec![Keys::KC_LeftBracket as u8],
            "]" => vec![Keys::KC_RightBracket as u8],
            "=" => vec![Keys::KC_Equal as u8],
            "-" => vec![Keys::KC_Minus as u8],
            // Other characters shifted
            "~" => vec![Keys::KC_LeftShift as u8, Keys::KC_Grave as u8],
            "<" => vec![Keys::KC_LeftShift as u8, Keys::KC_Comma as u8],
            ">" => vec![Keys::KC_LeftShift as u8, Keys::KC_Dot as u8],
            "?" => vec![Keys::KC_LeftShift as u8, Keys::KC_Slash as u8],
            "\""=> vec![Keys::KC_LeftShift as u8, Keys::KC_Quote as u8],
            ":" => vec![Keys::KC_LeftShift as u8, Keys::KC_Semicolon as u8],
            "|" => vec![Keys::KC_LeftShift as u8, Keys::KC_Backslash as u8],
            "{" => vec![
                Keys::KC_LeftShift as u8,
                Keys::KC_LeftBracket as u8,
            ],
            "}" => vec![
                Keys::KC_LeftShift as u8,
                Keys::KC_RightBracket as u8,
            ],
            "+" => vec![Keys::KC_LeftShift as u8, Keys::KC_Equal as u8],
            "_" => vec![Keys::KC_LeftShift as u8, Keys::KC_Minus as u8],
            // "*" => vec![KpAsterisk as u8],
            " " => vec![Keys::KC_Space as u8],
            // Otherwise it's 0
            _ => vec![Keys::KC_No as u8],
        };
        res
    }
    
    pub trait EnumInt {
        // as int
        fn i(&self) -> u32;
        fn as_int(&self) -> u8;
    }

    impl EnumInt for Keys {
        fn i(&self) -> u32 {
            *self as u32
        }
        fn as_int(&self) -> u8 {
            *self as u8
        }
    }

    pub trait KeebLed {
        fn as_led(key: Keys) -> Option<u8>;
    }

    pub fn layer(layer: u32) -> u32 {
        if layer < 1 || layer > 32 { panic!("Layer must be between 1 and 32"); }
        (255 as u32 + layer as u32) as u32
    }
    pub enum extras {
        ________ = 0x00,
    }
}
