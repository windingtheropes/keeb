// keep in note leds are not mapped and probably won't ever be since I do not use rgb on my keyboard.
// this is a simple representation of how I can contact my computer with my keyboard and do some hid tasks
// this coul;d be used for more advanced macros and all presses are sent here, unreliability aside from reading keypresses from the os
const VENDOR_ID: u16 = 0x320F;
const PRODUCT_ID: u16 = 0x5044;
use std::{thread, time::Duration};

enum LED {
    LedEsc = 0,     //0,Esc,K13
    LedGrv = 1,     //1,~,K16
    LedTab = 2,     //2,Tab,K11
    LedCaps = 3,   //3,Caps,K21
    LedLsft = 4,   //4,Sh_l,K00
    LedLctl = 5,   //5,Ct_l,K06
    LedF1 = 6,     //6,F1,K26
    Led1 = 7,      //7,1,K17
    LedQ = 8,      //8,Q,K10
    LedA = 9,      //9,A,K12
    LedZ = 10,     //10,Z,K14
    LedLwin = 11,  //11,Win_l,K90
    LedF2 = 12,    //12,F2,K36
    Led2 = 13,     //13,2,K27
    LedW = 14,     //14,W,K20
    LedS = 15,     //15,S,K22
    LedX = 16,     //16,X,K24
    LedLalt = 17,  //17,Alt_l,K93
    LedF3 = 18,    //18,F3,K31
    Led3 = 19,     //19,3,K37
    LedE = 20,     //20,E,K30
    LedD = 21,     //21,D,K32
    LedC = 22,     //22,C,K34
    LedF4 = 23,    //23,F4,K33
    Led4 = 24,     //24,4,K47
    LedR = 25,     //25,R,K40
    LedF = 26,     //26,F,K42
    LedV = 27,     //27,V,K44
    LedF5 = 28,    //28,F5,K07
    Led5 = 29,     //29,5,K46
    LedT = 30,     //30,T,K41
    LedG = 31,     //31,G,K43
    LedB = 32,     //32,B,K45
    LedSpc = 33,   //33,Space,K94
    LedF6 = 34,    //34,F6,K63
    Led6 = 35,     //35,6,K56
    LedY = 36,     //36,Y,K51
    LedH = 37,     //37,H,K53
    LedN = 38,     //38,N,K55
    LedF7 = 39,    //39,F7,K71
    Led7 = 40,     //40,7,K57
    LedU = 41,     //41,U,K50
    LedJ = 42,     //42,J,K52
    LedM = 43,     //43,M,K54
    LedF8 = 44,    //44,F8,K76
    Led8 = 45,     //45,8,K67
    LedI = 46,     //46,I,K60
    LedK = 47,     //47,K,K62
    LedComm = 48,  //48,,,K64
    LedRalt = 49,  //49,Alt_r,K95
    LedF9 = 50,    //50,F9,Ka6
    Led9 = 51,     //51,9,K77
    LedO = 52,     //52,O,K70
    LedL = 53,     //53,L,K72
    LedDot = 54,   //54,.,K74
    LedFn = 55,    //55,Fn,K92
    LedF10 = 56,   //56,F10,Ka7
    Led0 = 57,     //57,0,K87
    LedP = 58,     //58,P,K80
    LedScln = 59,  //59,;,K82
    LedSlsh = 60,  //60,?,K85
    LedF11 = 61,   //61,F11,Ka3
    LedMins = 62,  //62,-,K86
    LedLbrc = 63,  //63,[,K81
    LedQuot = 64,  //64,",K83
    LedRctl = 65,  //65,Ct_r,K04
    LedF12 = 66,   //66,F12,Ka5
    LedL1 = 67,    //67,Led,L01
    LedR1 = 68,    //68,Led,L11
    LedPrt = 69,   //69,Prt,K97
    LedL2 = 70,    //70,Led,L02
    LedR2 = 71,    //71,Led,L12
    LedDel = 72,   //72,Del,K65
    LedL3 = 73,    //73,Led,L03
    LedR3 = 74,    //74,Led,L13
    LedPgup = 75,  //75,Pgup,K15
    LedL4 = 76,    //76,Led,L04
    LedR4 = 77,    //77,Led,L14
    LedEql = 78,   //78,=,K66
    LedRight = 79, //79,Right,K05
    LedL5 = 80,    //80,Led,L05
    LedR5 = 81,    //81,Led,L15
    LedEnd = 82,   //82,End,K75
    LedL6 = 83,    //83,Led,L06
    LedR6 = 84,    //84,Led,L16
    LedBspc = 85,  //85,Bspc,Ka1
    LedPgdn = 86,  //86,Pgdn,K25
    LedL7 = 87,    //87,Led,L07
    LedR7 = 88,    //88,Led,L17
    LedRbrc = 89,  //89,],K61
    LedRsft = 90,  //90,Sh_r,K91
    LedL8 = 91,    //91,Led,L08
    LedR8 = 92,    //92,Led,L18
    LedBsls = 93,  //93,\,Ka2
    LedUp = 94,    //94,Up,K35
    LedLeft = 95,  //95,Left,K03
    LedEnt = 96,   //96,Enter,Ka4
    LedDown = 97,  //97,Down,K73
}

fn key_to_led(key:u8) -> u8{ // this will never be completed and it was purely for testing purposes.
    match key{
        41 => 0,
        53 => 1,
        43 => 2, 
        57 => 3,
        225 => 4, 
        224 => 5,
        58 => 6, 
        30 => 7,
        20 => 8,
        4 => 9,
        29 => 10,
        227 => 11,
        59 => 12,
        31 => 13,
        26 => 14,
        22 => 15,
        27 => 16,
        226 => 17,
        60 => 18,
        32 => 19,
        8 => 20,
        7 => 21,
        6 => 22,
        61 => 23,
        33 => 24,
        21 => 25,
        9 => 26,
        25 => 27,
        62 => 28,
        34 => 29,
        23 => 30,
        10 => 31,
        5 => 32,
        44 => 33,
        63 => 34,
        35 => 35,
        28 => 36,
        11 => 37,
        17 => 38,
        64 => 39,
        _ => 0,
    }
}

use hidapi::HidDevice;

fn set_color_all(r: u8, g: u8, b: u8, device: &HidDevice) {
    let buf = [0x1, 2, r, g, b];
    device.write(&buf).unwrap();
}

fn set_color(led: u8, r: u8, g: u8, b: u8, device: &HidDevice) {
    let buf = [0x1, 1, led, r, g, b];
    device.write(&buf).unwrap();
}

fn blink_key(led: u8, device: &HidDevice)
{
    let interval = 255 / 12;
    let mut curr = 255;
    set_color(led, 255, 255, 255, &device);
    for i in 0..12 {
        curr -= interval;
        set_color(led, curr, curr, curr, &device);
        thread::sleep(Duration::from_micros(1200))
    }
    set_color(led, 0, 0, 0, &device);
}
fn main() {
    let api = hidapi::HidApi::new().unwrap();
    let device = api.open(VENDOR_ID, PRODUCT_ID).unwrap();

    blink_key(45, &device);

    // Write data to device
    // the length of the payload must always be one more byte than the length of the actual desired readable payload, by preceeding it with a 0x1 byte
    // 1: simple single
    // 2: simple all

    // writing data
    // the payload must be one longer than the desired readable result is, by 0x1 at front, which will not show.
    // set_color_all(255, 255, 255, &device);
    // thread::sleep(Duration::from_millis(4000));
    // set_color_all(0, 0, 0, &device);

    // set_color(led_location_map::LED_ESC as u8, 255, 255, 255, &device);
    // thread::sleep(Duration::from_secs(1));
    // set_color(led_location_map::LED_ESC as u8, 0, 0, 0, &device);

    // reading data
    // u8 is the value type and 2 is the length
    // loop {
    //     let mut buf = [0u8; 2];
    //     let res = device.read(&mut buf[..]).unwrap();
    //     println!("Read: {:?}", &buf[..res]);
    //     let r = &buf[..res];
    //     blink_key(key_to_led(r[1]), &device);
    // }
    // reading
    // let mut buf = [0u8; 2];
    // let res = device.read(&mut buf[..]).unwrap();
    // println!("Read: {:?}", &buf[..res]);
}
