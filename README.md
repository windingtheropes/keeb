# keeb
Open source keyboard management software for QMK keyboards.

# using keeb
Currently, keeb is only mapped and set up for the GMMK Pro, with lots of stuff hardcoded to it for quicker testing.
## modularity (planned)
It's planned in the future to add a completely modular structure for keeb, allowing anyone to create a crate with keymaps and led mappings for their QMK keyboards, then basically plug and play from there. Keeb is far away from this and is only at an early working prototype.
# firmware
Keeb requires keeb-compatible firmware to communicate with your keyboard. As stated above, keeb only works with the Gmmk Pro at this time, and only has firmware for it. Though, as compared to the main app, porting keeb's firmware to another keyboard should not be much of a hastle. You can obtain the keeb firmware for the Gmmk Pro ANSI [here](https://github.com/windingtheropes/qmk_firmware/tree/windingtheropes/keyboards/gmmk/pro/rev1/ansi/keymaps/keeb)


