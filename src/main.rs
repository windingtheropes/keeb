// keeb created by windingtheropes
// this is a working test to show implementations with different keyboards. it works, but there's no customization aspect at the moment.
use hidapi::{DeviceInfo, HidApi, HidDevice, HidResult};

fn main() {    
    let api = hidapi::HidApi::new().unwrap();
    let gmmk_pro = gmmk::pro::gmmkPro();
    keebLib::keeb::startListening(&gmmk_pro, &api);
}
