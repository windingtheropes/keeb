// keeb created by windingtheropes
// this is a working test to show implementations with different keyboards. it works, but there's no customization aspect at the moment.

fn main() {    
    let api = hidapi::HidApi::new().unwrap();
    let gmmk_pro = gmmk::pro::gmmk_pro();
    keebLib::keeb::start_listener(&gmmk_pro, true, &api);
}
