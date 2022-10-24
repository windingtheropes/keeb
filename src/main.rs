// keeb created by windingtheropes
fn main() {    
    let api = hidapi::HidApi::new().unwrap();
    keeb_lib::keeb::start_listener(&gmmk::pro::gmmk_pro(), true, &api);
}
