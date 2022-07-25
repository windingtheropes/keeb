pub mod pro {
    use keebLib::codes::{Keys, EnumInt, KeebLed};
    use keebLib::codes::Keys::*;

    //    +--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+        +--------+
//    | Esc    | F1     | F2     | F3     | F4     | F5     | F6     | F7     | F8     | F9     | F10    | F11    | F12    | PrScr  |        | Mute   |
//    +--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+        +--------+
//    +--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+        +--------+
//    | `~     | 1      | 2      | 3      | 4      | 5      | 6      | 7      | 8      | 9      | 0      | -      | =      | BackSp |        | Del   |
//    +--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+        +--------+
//    | Tab    | Q      | W      | E      | R      | T      | Y      | U      | I      | O      | P      | [      | ]      | \      |        | Pgup   |
//    +--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+        +--------+
//    | Caps   | A      | S      | D      | F      | G      | H      | J      | K      | L      | ;      | '      | Enter           |        | Pgdn    |
//    +--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+-----------------+        +--------+
//    | Shift           | Z      | X      | C      | V      | B      | N      | M      | ,      | .      | /      | Shift           +--------+ End     |
//    +-----------------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+-----------------+ Up     +--------+
//    +--------+--------+--------+--------------------------------------------------------------+--------+--------+--------+--------+--------+--------+
//    | Ctrl   | Alt    | Super  |                            Space                             | Alt    | FN  | Ctrl   | Left   | Down   | Right  |
//    +--------+--------+--------+--------------------------------------------------------------+--------+--------+--------+--------+--------+--------+

// key 78 is fn key
    pub const fn default_keymap() -> [keebLib::codes::Keys; 83] {
        [
        KC_Escape,  KC_F1,  KC_F2,  KC_F3,  KC_F4,  KC_F5,  KC_F6,  KC_F7,  KC_F8,  KC_F9,  KC_F10,  KC_F11,  KC_F12,  KC_PrintScreen,                                   KC_AudioMute,
        KC_Grave,   KC_Num1,KC_Num2,KC_Num3,KC_Num4,KC_Num5,KC_Num6,KC_Num7,KC_Num8,KC_Num9,KC_Num0, KC_Minus, KC_Equal,  KC_Backspace,                                  KC_Delete,
        KC_Tab,     KC_Q,   KC_W,   KC_E,   KC_R,   KC_T,   KC_Y,   KC_U,   KC_I,   KC_O,   KC_P,    KC_LeftBracket, KC_RightBracket, KC_Backslash,                      KC_PageUp,
        KC_CapsLock,KC_A,   KC_S,   KC_D,   KC_F,   KC_G,   KC_H,   KC_J,   KC_K,   KC_L,   KC_Semicolon, KC_Quote, KC_Enter,                                            KC_PageDown,
        KC_LeftShift,       KC_Z,   KC_X,   KC_C,   KC_V,   KC_B,   KC_N,   KC_M,   KC_Comma, KC_Dot,  KC_Slash, KC_RightShift,                        KC_Up,            KC_End,
       KC_LeftCtrl, KC_Menu, KC_LeftAlt,                 KC_Space,                            KC_RightAlt, KC_RollOver,   KC_RightCtrl,      KC_Left, KC_Down, KC_Right
    ]
    }

    // Number of keys on the keyboard
    pub const fn keycount() -> usize {
        default_keymap().len()
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy)]
    // led naming based on the default layout of this keyboard.
    pub enum Leds {
        LED_Esc = 0,           //0,Esc,K13
        LED_Grave = 1,         //1,~,K16
        LED_Tab = 2,           //2,Tab,K11
        LED_CapsLock = 3,      //3,Caps,K21
        LED_LeftShift = 4,     //4,Sh_l,K00
        LED_LeftCtrl = 5,      //5,Ct_l,K06
        LED_F1 = 6,            //6,F1,K26
        LED_Num1 = 7,          //7,1,K17
        LED_Q = 8,             //8,Q,K10
        LED_A = 9,             //9,A,K12
        LED_Z = 10,            //10,Z,K14
        LED_Lwin = 11,         //11,Win_l,K90
        LED_F2 = 12,           //12,F2,K36
        LED_Num2 = 13,         //13,2,K27
        LED_W = 14,            //14,W,K20
        LED_S = 15,            //15,S,K22
        LED_X = 16,            //16,X,K24
        LED_LeftAlt = 17,      //17,Alt_l,K93
        LED_F3 = 18,           //18,F3,K31
        LED_Num3 = 19,         //19,3,K37
        LED_E = 20,            //20,E,K30
        LED_D = 21,            //21,D,K32
        LED_C = 22,            //22,C,K34
        LED_F4 = 23,           //23,F4,K33
        LED_Num4 = 24,         //24,4,K47
        LED_R = 25,            //25,R,K40
        LED_F = 26,            //26,F,K42
        LED_V = 27,            //27,V,K44
        LED_F5 = 28,           //28,F5,K07
        LED_Num5 = 29,         //29,5,K46
        LED_T = 30,            //30,T,K41
        LED_G = 31,            //31,G,K43
        LED_B = 32,            //32,B,K45
        LED_Space = 33,        //33,Space,K94
        LED_F6 = 34,           //34,F6,K63
        LED_Num6 = 35,         //35,6,K56
        LED_Y = 36,            //36,Y,K51
        LED_H = 37,            //37,H,K53
        LED_N = 38,            //38,N,K55
        LED_F7 = 39,           //39,F7,K71
        LED_Num7 = 40,         //40,7,K57
        LED_U = 41,            //41,U,K50
        LED_J = 42,            //42,J,K52
        LED_M = 43,            //43,M,K54
        LED_F8 = 44,           //44,F8,K76
        LED_Num8 = 45,         //45,8,K67
        LED_I = 46,            //46,I,K60
        LED_K = 47,            //47,K,K62
        LED_Comm = 48,         //48,,,K64
        LED_RightAlt = 49,     //49,Alt_r,K95
        LED_F9 = 50,           //50,F9,Ka6
        LED_Num9 = 51,         //51,9,K77
        LED_O = 52,            //52,O,K70
        LED_L = 53,            //53,L,K72
        LED_Dot = 54,          //54,.,K74
        LED_Fn = 55,           //55,Fn,K92
        LED_F10 = 56,          //56,F10,Ka7
        LED_Num0 = 57,         //57,0,K87
        LED_P = 58,            //58,P,K80
        LED_Semicolon = 59,    //59,;,K82
        LED_Slash = 60,        //60,?,K85
        LED_F11 = 61,          //61,F11,Ka3
        LED_Minus = 62,        //62,-,K86
        LED_LeftBracket = 63,  //63,[,K81
        LED_Quote = 64,        //64,",K83
        LED_RightCtrl = 65,    //65,Ct_r,K04
        LED_F12 = 66,          //66,F12,Ka5
        LED_L1 = 67,           //67,,L01
        LED_R1 = 68,           //68,,L11
        LED_PrintScreen = 69,  //69,Prt,K97
        LED_L2 = 70,           //70,,L02
        LED_R2 = 71,           //71,,L12
        LED_Home = 72,         //72,Home,K65
        LED_L3 = 73,           //73,,L03
        LED_R3 = 74,           //74,,L13
        LED_Pgup = 75,         //75,Pgup,K15
        LED_L4 = 76,           //76,,L04
        LED_R4 = 77,           //77,,L14
        LED_Equal = 78,        //78,=,K66
        LED_Right = 79,        //79,Right,K05
        LED_L5 = 80,           //80,,L05
        LED_R5 = 81,           //81,,L15
        LED_End = 82,          //82,End,K75
        LED_L6 = 83,           //83,,L06
        LED_R6 = 84,           //84,,L16
        LED_Backspace = 85,    //85,Bspc,Ka1
        LED_Pgdn = 86,         //86,Pgdn,K25
        LED_L7 = 87,           //87,,L07
        LED_R7 = 88,           //88,,L17
        LED_RightBracket = 89, //89,],K61
        LED_RightShift = 90,   //90,Sh_r,K91
        LED_L8 = 91,           //91,,L08
        LED_R8 = 92,           //92,,L18
        LED_Backslash = 93,    //93,\,Ka2
        LED_Up = 94,           //94,Up,K35
        LED_Left = 95,         //95,Left,K03
        LED_Enter = 96,        //96,Enter,Ka4
        LED_Down = 97,         //97,Down,K73
    }

    impl EnumInt for Leds {
        fn as_int(&self) -> u8 {
            *self as u8
        }
    }

    impl KeebLed for Leds {
        fn as_led(key: Keys) -> Option<u8> {
            let res = match key {
                // Alphabet
                Keys::KC_A => Some(Leds::LED_A.as_int()),
                Keys::KC_B => Some(Leds::LED_B.as_int()),
                Keys::KC_C => Some(Leds::LED_C.as_int()),
                Keys::KC_D => Some(Leds::LED_D.as_int()),
                Keys::KC_E => Some(Leds::LED_E.as_int()),
                Keys::KC_F => Some(Leds::LED_F.as_int()),
                Keys::KC_G => Some(Leds::LED_G.as_int()),
                Keys::KC_H => Some(Leds::LED_H.as_int()),
                Keys::KC_I => Some(Leds::LED_I.as_int()),
                Keys::KC_J => Some(Leds::LED_J.as_int()),
                Keys::KC_K => Some(Leds::LED_K.as_int()),
                Keys::KC_L => Some(Leds::LED_L.as_int()),
                Keys::KC_M => Some(Leds::LED_M.as_int()),
                Keys::KC_N => Some(Leds::LED_N.as_int()),
                Keys::KC_O => Some(Leds::LED_O.as_int()),
                Keys::KC_P => Some(Leds::LED_P.as_int()),
                Keys::KC_Q => Some(Leds::LED_Q.as_int()),
                Keys::KC_R => Some(Leds::LED_R.as_int()),
                Keys::KC_S => Some(Leds::LED_S.as_int()),
                Keys::KC_T => Some(Leds::LED_T.as_int()),
                Keys::KC_U => Some(Leds::LED_U.as_int()),
                Keys::KC_V => Some(Leds::LED_V.as_int()),
                Keys::KC_W => Some(Leds::LED_W.as_int()),
                Keys::KC_X => Some(Leds::LED_X.as_int()),
                Keys::KC_Y => Some(Leds::LED_Y.as_int()),
                Keys::KC_Z => Some(Leds::LED_Z.as_int()),
                // Number row
                Keys::KC_Num1 => Some(Leds::LED_Num1.as_int()),
                Keys::KC_Num2 => Some(Leds::LED_Num2.as_int()),
                Keys::KC_Num3 => Some(Leds::LED_Num3.as_int()),
                Keys::KC_Num4 => Some(Leds::LED_Num4.as_int()),
                Keys::KC_Num5 => Some(Leds::LED_Num5.as_int()),
                Keys::KC_Num6 => Some(Leds::LED_Num6.as_int()),
                Keys::KC_Num7 => Some(Leds::LED_Num7.as_int()),
                Keys::KC_Num8 => Some(Leds::LED_Num8.as_int()),
                Keys::KC_Num9 => Some(Leds::LED_Num9.as_int()),
                Keys::KC_Num0 => Some(Leds::LED_Num0.as_int()),
                // Function row
                Keys::KC_F1 => Some(Leds::LED_F1.as_int()),
                Keys::KC_F2 => Some(Leds::LED_F2.as_int()),
                Keys::KC_F3 => Some(Leds::LED_F3.as_int()),
                Keys::KC_F4 => Some(Leds::LED_F4.as_int()),
                Keys::KC_F5 => Some(Leds::LED_F5.as_int()),
                Keys::KC_F6 => Some(Leds::LED_F6.as_int()),
                Keys::KC_F7 => Some(Leds::LED_F7.as_int()),
                Keys::KC_F8 => Some(Leds::LED_F8.as_int()),
                Keys::KC_F9 => Some(Leds::LED_F9.as_int()),
                Keys::KC_F10 => Some(Leds::LED_F10.as_int()),
                Keys::KC_F11 => Some(Leds::LED_F11.as_int()),
                Keys::KC_F12 => Some(Leds::LED_F12.as_int()),
                // Modifier keys
                Keys::KC_Tab => Some(Leds::LED_Tab.as_int()),
                Keys::KC_CapsLock => Some(Leds::LED_CapsLock.as_int()),
                Keys::KC_LeftShift => Some(Leds::LED_LeftShift.as_int()),
                Keys::KC_LeftCtrl => Some(Leds::LED_LeftCtrl.as_int()),
                Keys::KC_Menu => Some(Leds::LED_Lwin.as_int()),
                Keys::KC_LeftAlt => Some(Leds::LED_LeftAlt.as_int()),
                Keys::KC_Backspace => Some(Leds::LED_Backspace.as_int()),
                Keys::KC_Enter => Some(Leds::LED_Enter.as_int()),
                Keys::KC_RightShift => Some(Leds::LED_RightShift.as_int()),
                Keys::KC_RightAlt => Some(Leds::LED_RightAlt.as_int()),
                Keys::KC_RightCtrl => Some(Leds::LED_RightCtrl.as_int()),
                Keys::KC_PrintScreen => Some(Leds::LED_PrintScreen.as_int()),
                // FN
                Keys::KC_RollOver => Some(Leds::LED_Fn.as_int()),
                // Right side bar of keys
                Keys::KC_Delete => Some(Leds::LED_Home.as_int()),
                Keys::KC_Insert => Some(Leds::LED_Pgup.as_int()),
                Keys::KC_PageDown => Some(Leds::LED_Pgdn.as_int()),
                Keys::KC_End => Some(Leds::LED_End.as_int()),
                // Other Symbols
                Keys::KC_Grave => Some(Leds::LED_Grave.as_int()),
                Keys::KC_Minus => Some(Leds::LED_Minus.as_int()),
                Keys::KC_Equal => Some(Leds::LED_Equal.as_int()),
                Keys::KC_LeftBracket => Some(Leds::LED_LeftBracket.as_int()),
                Keys::KC_RightBracket => Some(Leds::LED_RightBracket.as_int()),
                Keys::KC_Backslash => Some(Leds::LED_Backslash.as_int()),
                Keys::KC_Semicolon => Some(Leds::LED_Semicolon.as_int()),
                Keys::KC_Quote => Some(Leds::LED_Quote.as_int()),
                Keys::KC_Comma => Some(Leds::LED_Comm.as_int()),
                Keys::KC_Dot => Some(Leds::LED_Dot.as_int()),
                Keys::KC_Slash => Some(Leds::LED_Slash.as_int()),
                // Space
                Keys::KC_Space => Some(Leds::LED_Space.as_int()),

                _ => None,
            };
            res
        }
    }
}