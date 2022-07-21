 // 0.17.1
pub mod codes {
    use strum::IntoEnumIterator; // 0.17.1
    use strum_macros::EnumIter;
    // note:
    // Num1 is 1 in the number row
    // Kp1 is 1 in the keypad
    #[derive(Debug, EnumIter, Copy, Clone)]
    pub enum StandardKeys {
        No = 0x00,
        RollOver = 1,
        PostFail = 2,
        Undefined = 3,
        A = 4,
        B = 5,
        C = 6,
        D = 7,
        E = 8,
        F = 9,
        G = 10,
        H = 11,
        I = 12,
        J = 13,
        K = 14,
        L = 15,
        M = 16, // 0x10
        N = 17,
        O = 18,
        P = 19,
        Q = 20,
        R = 21,
        S = 22,
        T = 23,
        U = 24,
        V = 25,
        W = 26,
        X = 27,
        Y = 28,
        Z = 29,
        Num1 = 30,
        Num2 = 31,
        Num3 = 32, // 0x20
        Num4 = 33,
        Num5 = 34,
        Num6 = 35,
        Num7 = 36,
        Num8 = 37,
        Num9 = 38,
        Num0 = 39,
        Enter = 40,
        Escape = 41,
        Backspace = 42,
        Tab = 43,
        Space = 44,
        Minus = 45,
        Equal = 46,
        LeftBracket = 47,
        RightBracket = 48, // 0x30
        Backslash = 49,
        NonusHash = 50,
        Semicolon = 51,
        Quote = 52,
        Grave = 53,
        Comma = 54,
        Dot = 55,
        Slash = 56,
        CapsLock = 57,
        F1 = 58,
        F2 = 59,
        F3 = 60,
        F4 = 61,
        F5 = 62,
        F6 = 63,
        F7 = 64,// 0x40
        F8 = 65,
        F9 = 66,
        F10 = 67,
        F11 = 68,
        F12 = 69,
        PrintScreen = 70,
        ScrollLock = 71,
        Pause = 72,
        Insert = 73,
        Home = 74,
        PageUp = 75,
        Delete = 76,
        End = 77,
        PageDown = 78,
        Right = 79,
        Left = 80, // 0x50
        Down = 81,
        Up = 82,
        NumLock = 83,
        KpSlash = 84,
        KpAsterisk = 85,
        KpMinus = 86,
        KpPlus = 87,
        KpEnter = 88,
        Kp1 = 89,
        Kp2 = 90,
        Kp3 = 91,
        Kp4 = 92,
        Kp5 = 93,
        Kp6 = 94,
        Kp7 = 95,
        Kp8 = 96, // 0x60
        Kp9 = 97,
        Kp0 = 98,
        KpDot = 99,
        NonusBackslash = 100,
        Application = 101,
        KbPower = 102,
        KpEqual = 103,
        F13 = 104,
        F14 = 105,
        F15 = 106,
        F16 = 107,
        F17 = 108,                    
        F18 = 109,
        F19 = 110,
        F20 = 111,
        F21 = 112, // 0x70
        F22 = 113,
        F23 = 114,
        F24 = 115,
        Execute = 116,
        Help = 117,
        Menu = 118,
        Select = 119,
        Stop = 120,
        Again = 121,
        Undo = 122,
        Cut = 123,
        Copy = 124,
        Paste = 125,
        Find = 126,
        KbMute = 127,
        KbVolumeUp = 128, // 0x80
        KbVolumeDown = 129,
        LockingCapsLock = 130,
        LockingNumLock = 131,
        LockingScrollLock = 132,
        KpComma = 133,
        KpEqualAs400 = 134,
        International1 = 135,
        International2 = 136,
        International3 = 137,
        International4 = 138,
        International5 = 139,
        International6 = 140,
        International7 = 141,
        International8 = 142,
        International9 = 143,
        Language1 = 144, // 0x90
        Language2 = 145,
        Language3 = 146,
        Language4 = 147,
        Language5 = 148,
        Language6 = 149,
        Language7 = 150,
        Language8 = 151,
        Language9 = 152,
        AlternateErase = 153,
        SystemRequest= 154,
        Cancel = 155, 
        Clear = 156,
        Prior = 157,
        Return = 158,
        Separator = 159,
        Out = 160, // 0xA0
        Oper = 161,
        ClearAgain = 162,
        Crsel = 163,
        Exsel = 164,

        // 165-190 are special keys defined below

        /* Modifiers */
        LeftCtrl = 0xE0, // 224
        LeftShift = 225,
        LeftAlt = 226, 
        LeftGui = 227,
        RightCtrl = 228,
        RightShift = 229,
        RightAlt = 230,
        RightGui = 231,
        
        // **********************************************
        // * 0xF0-0xFF are unallocated in the HID spec. *
        // * QMK uses these for Mouse Keys - see below. *
        // **********************************************
    }

    impl StandardKeys {
        pub fn get_key(k: u8) -> Option<StandardKeys> {
            for key in StandardKeys::iter() {
                if key as u8 == k {
                    return Some(key)
                }
            }
            None
        }
    }
    pub trait EnumInt {
        fn as_int(&self) -> u8;
    }

    impl EnumInt for StandardKeys {
        fn as_int(&self) -> u8 {
            *self as u8
        }
    }

    pub trait LedKeyboard {
        fn key_to_led(key: StandardKeys) -> Option<u8>;
    }

    /* Media and Function keys */
    pub enum SpecialKeys {
        /* Generic Desktop Page (0x01) */
        SystemPower = 0xA5, //165
        SystemSleep = 166,
        SystemWake = 167,

        /* Consumer Page (0x0C) */
        AudioMute = 168,
        AudioVolUp = 169,
        AudioVolDown = 170,
        MediaNextTrack = 171,
        MediaPrevTrack = 172,
        MediaStop = 173,
        MediaPlayPause = 174,
        MediaSelect = 175,
        MediaEject = 176, // 0xB0
        Mail = 177,
        Calculator = 178,
        MyComputer = 179,
        WwwSearch = 180,
        WwwHome = 181,
        WwwBack = 182,
        WwwForward = 183,
        WwwStop = 184,
        WwwRefresh = 185,
        WwwFavorites = 186,
        MediaFastForward = 187,
        MediaRewind = 188,
        BrightnessUp = 189,
        BrightnessDown = 190,
    }

    pub enum MouseKeys {
        /* Mouse Buttons */
        MsUp = 0xED, // 237
        MsDown = 238,
        MsLeft = 239,
        MsRight = 0xF0, // 240
        MsBtn1 = 241,
        MsBtn2 = 242,
        MsBtn3 = 243,
        MsBtn4 = 244,
        MsBtn5 = 245,
        MsBtn6 = 246,
        MsBtn7 = 247,
        MsBtn8 = 248,

        /* Mouse Wheel */
        MsWhUp = 249,
        MsWhDown = 250,
        MsWhLeft = 251,
        MsWhRight = 252,

        /* Acceleration */
        MsAccel0 = 253,
        MsAccel1 = 254,
        MsAccel2 = 255, // 0xFF
    }

    #[derive(Clone, Copy)]
    // led naming based on the default layout of this keyboard.
    pub enum GmmkProLed {
        Esc = 0,    //0,Esc,K13
        Grave = 1,    //1,~,K16
        Tab = 2,    //2,Tab,K11
        CapsLock = 3,   //3,Caps,K21
        LeftShift = 4,   //4,Sh_l,K00
        LeftCtrl = 5,   //5,Ct_l,K06
        F1 = 6,     //6,F1,K26
        Num1 = 7,      //7,1,K17
        Q = 8,      //8,Q,K10
        A = 9,      //9,A,K12
        Z = 10,     //10,Z,K14
        Lwin = 11,  //11,Win_l,K90
        F2 = 12,    //12,F2,K36
        Num2 = 13,     //13,2,K27
        W = 14,     //14,W,K20
        S = 15,     //15,S,K22
        X = 16,     //16,X,K24
        LeftAlt = 17,  //17,Alt_l,K93
        F3 = 18,    //18,F3,K31
        Num3 = 19,     //19,3,K37
        E = 20,     //20,E,K30
        D = 21,     //21,D,K32
        C = 22,     //22,C,K34
        F4 = 23,    //23,F4,K33
        Num4 = 24,     //24,4,K47
        R = 25,     //25,R,K40
        F = 26,     //26,F,K42
        V = 27,     //27,V,K44
        F5 = 28,    //28,F5,K07
        Num5 = 29,     //29,5,K46
        T = 30,     //30,T,K41
        G = 31,     //31,G,K43
        B = 32,     //32,B,K45
        Space = 33,   //33,Space,K94
        F6 = 34,    //34,F6,K63
        Num6 = 35,     //35,6,K56
        Y = 36,     //36,Y,K51
        H = 37,     //37,H,K53
        N = 38,     //38,N,K55
        F7 = 39,    //39,F7,K71
        Num7 = 40,     //40,7,K57
        U = 41,     //41,U,K50
        J = 42,     //42,J,K52
        M = 43,     //43,M,K54
        F8 = 44,    //44,F8,K76
        Num8 = 45,     //45,8,K67
        I = 46,     //46,I,K60
        K = 47,     //47,K,K62
        Comm = 48,  //48,,,K64
        RightAlt = 49,  //49,Alt_r,K95
        F9 = 50,    //50,F9,Ka6
        Num9 = 51,     //51,9,K77
        O = 52,     //52,O,K70
        L = 53,     //53,L,K72
        Dot = 54,   //54,.,K74
        Fn = 55,    //55,Fn,K92
        F10 = 56,   //56,F10,Ka7
        Num0 = 57,     //57,0,K87
        P = 58,     //58,P,K80
        Semicolon = 59,  //59,;,K82
        Slash = 60,  //60,?,K85
        F11 = 61,   //61,F11,Ka3
        Minus = 62,  //62,-,K86
        LeftBracket = 63,  //63,[,K81
        Quote = 64,  //64,",K83
        RightCtrl = 65,  //65,Ct_r,K04
        F12 = 66,   //66,F12,Ka5
        L1 = 67,    //67,,L01
        R1 = 68,    //68,,L11
        PrintScreen = 69,   //69,Prt,K97
        L2 = 70,    //70,,L02
        R2 = 71,    //71,,L12
        Home = 72,   //72,Home,K65
        L3 = 73,    //73,,L03
        R3 = 74,    //74,,L13
        Pgup = 75,  //75,Pgup,K15
        L4 = 76,    //76,,L04
        R4 = 77,    //77,,L14
        Equal = 78,   //78,=,K66
        Right = 79, //79,Right,K05
        L5 = 80,    //80,,L05
        R5 = 81,    //81,,L15
        End = 82,   //82,End,K75
        L6 = 83,    //83,,L06
        R6 = 84,    //84,,L16
        Backspace = 85,  //85,Bspc,Ka1
        Pgdn = 86,  //86,Pgdn,K25
        L7 = 87,    //87,,L07
        R7 = 88,    //88,,L17
        RightBracket = 89,  //89,],K61
        RightShift = 90,  //90,Sh_r,K91
        L8 = 91,    //91,,L08
        R8 = 92,    //92,,L18
        Backslash = 93,  //93,\,Ka2
        Up = 94,    //94,Up,K35
        Left = 95,  //95,Left,K03
        Enter = 96,   //96,Enter,Ka4
        Down = 97,  //97,Down,K73
    }
    impl EnumInt for GmmkProLed {
        fn as_int(&self) -> u8 {
            *self as u8
        }
    }

    impl LedKeyboard for GmmkProLed {
        fn key_to_led(key: StandardKeys) -> Option<u8> {
            let res = match key {
                // Alphabet
                StandardKeys::A => Some(GmmkProLed::A.as_int()),
                StandardKeys::B => Some(GmmkProLed::B.as_int()),
                StandardKeys::C => Some(GmmkProLed::C.as_int()),
                StandardKeys::D => Some(GmmkProLed::D.as_int()),
                StandardKeys::E => Some(GmmkProLed::E.as_int()),
                StandardKeys::F => Some(GmmkProLed::F.as_int()),
                StandardKeys::G => Some(GmmkProLed::G.as_int()),
                StandardKeys::H => Some(GmmkProLed::H.as_int()),
                StandardKeys::I => Some(GmmkProLed::I.as_int()),
                StandardKeys::J => Some(GmmkProLed::J.as_int()),
                StandardKeys::K => Some(GmmkProLed::K.as_int()),
                StandardKeys::L => Some(GmmkProLed::L.as_int()),
                StandardKeys::M => Some(GmmkProLed::M.as_int()),
                StandardKeys::N => Some(GmmkProLed::N.as_int()),
                StandardKeys::O => Some(GmmkProLed::O.as_int()),
                StandardKeys::P => Some(GmmkProLed::P.as_int()),
                StandardKeys::Q => Some(GmmkProLed::Q.as_int()),
                StandardKeys::R => Some(GmmkProLed::R.as_int()),
                StandardKeys::S => Some(GmmkProLed::S.as_int()),
                StandardKeys::T => Some(GmmkProLed::T.as_int()),
                StandardKeys::U => Some(GmmkProLed::U.as_int()),
                StandardKeys::V => Some(GmmkProLed::V.as_int()),
                StandardKeys::W => Some(GmmkProLed::W.as_int()),
                StandardKeys::X => Some(GmmkProLed::X.as_int()),
                StandardKeys::Y => Some(GmmkProLed::Y.as_int()),
                StandardKeys::Z => Some(GmmkProLed::Z.as_int()),
                // Number row
                StandardKeys::Num1 => Some(GmmkProLed::Num1.as_int()),
                StandardKeys::Num2 => Some(GmmkProLed::Num2.as_int()),
                StandardKeys::Num3 => Some(GmmkProLed::Num3.as_int()),
                StandardKeys::Num4 => Some(GmmkProLed::Num4.as_int()),
                StandardKeys::Num5 => Some(GmmkProLed::Num5.as_int()),
                StandardKeys::Num6 => Some(GmmkProLed::Num6.as_int()),
                StandardKeys::Num7 => Some(GmmkProLed::Num7.as_int()),
                StandardKeys::Num8 => Some(GmmkProLed::Num8.as_int()),
                StandardKeys::Num9 => Some(GmmkProLed::Num9.as_int()),
                StandardKeys::Num0 => Some(GmmkProLed::Num0.as_int()),
                // Function row
                StandardKeys::F1 => Some(GmmkProLed::F1.as_int()),
                StandardKeys::F2 => Some(GmmkProLed::F2.as_int()),
                StandardKeys::F3 => Some(GmmkProLed::F3.as_int()),
                StandardKeys::F4 => Some(GmmkProLed::F4.as_int()),
                StandardKeys::F5 => Some(GmmkProLed::F5.as_int()),
                StandardKeys::F6 => Some(GmmkProLed::F6.as_int()),
                StandardKeys::F7 => Some(GmmkProLed::F7.as_int()),
                StandardKeys::F8 => Some(GmmkProLed::F8.as_int()),
                StandardKeys::F9 => Some(GmmkProLed::F9.as_int()),
                StandardKeys::F10 => Some(GmmkProLed::F10.as_int()),
                StandardKeys::F11 => Some(GmmkProLed::F11.as_int()),
                StandardKeys::F12 => Some(GmmkProLed::F12.as_int()),
                // Modifier keys
                StandardKeys::Tab => Some(GmmkProLed::Tab.as_int()),
                StandardKeys::CapsLock => Some(GmmkProLed::CapsLock.as_int()),
                StandardKeys::LeftShift => Some(GmmkProLed::LeftShift.as_int()),
                StandardKeys::LeftCtrl => Some(GmmkProLed::LeftCtrl.as_int()),
                StandardKeys::Menu => Some(GmmkProLed::Lwin.as_int()),
                StandardKeys::LeftAlt => Some(GmmkProLed::LeftAlt.as_int()),
                StandardKeys::Backspace => Some(GmmkProLed::Backspace.as_int()),
                StandardKeys::Enter => Some(GmmkProLed::Enter.as_int()),
                StandardKeys::RightShift => Some(GmmkProLed::RightShift.as_int()),
                StandardKeys::RightAlt => Some(GmmkProLed::RightAlt.as_int()),
                StandardKeys::RightCtrl => Some(GmmkProLed::RightCtrl.as_int()),
                StandardKeys::PrintScreen => Some(GmmkProLed::PrintScreen.as_int()),
                // FN
                StandardKeys::RollOver => Some(GmmkProLed::Fn.as_int()),
                // Right side bar of keys
                StandardKeys::Delete => Some(GmmkProLed::Home.as_int()),
                StandardKeys::Insert => Some(GmmkProLed::Pgup.as_int()),
                StandardKeys::PageDown => Some(GmmkProLed::Pgdn.as_int()),
                StandardKeys::End => Some(GmmkProLed::End.as_int()),
                // Other Symbols
                StandardKeys::Grave => Some(GmmkProLed::Grave.as_int()),
                StandardKeys::Minus => Some(GmmkProLed::Minus.as_int()),
                StandardKeys::Equal => Some(GmmkProLed::Equal.as_int()),
                StandardKeys::LeftBracket => Some(GmmkProLed::LeftBracket.as_int()),
                StandardKeys::RightBracket => Some(GmmkProLed::RightBracket.as_int()),
                StandardKeys::Backslash => Some(GmmkProLed::Backslash.as_int()),
                StandardKeys::Semicolon => Some(GmmkProLed::Semicolon.as_int()),
                StandardKeys::Quote => Some(GmmkProLed::Quote.as_int()),
                StandardKeys::Comma => Some(GmmkProLed::Comm.as_int()),
                StandardKeys::Dot => Some(GmmkProLed::Dot.as_int()),
                StandardKeys::Slash => Some(GmmkProLed::Slash.as_int()),
                // Space
                StandardKeys::Space => Some(GmmkProLed::Space.as_int()),
                
                _ => None,
            };
            res
        }
    }
}
