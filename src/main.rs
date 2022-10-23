// keeb created by windingtheropes
// this is a working test to show implementations with different keyboards. it works, but there's no customization aspect at the moment.
use keebLib::keeb::Manager;
fn main() {    
    let api = hidapi::HidApi::new().unwrap();
    let gmmk_pro = gmmk::pro::gmmk_pro();
    let mut manager = Manager::new(gmmk_pro, api);
    manager.intercept(true);
    manager.listen()
}
