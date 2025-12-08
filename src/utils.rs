///
///
///
pub fn open_centred_floating_terminal_window() {
    let _ = create_popup_window(PopupWindowOptions {
        border: WindowBorder::Rounded,
        window_width_ratio: Some(0.7),
        window_height_ratio: Some(0.7),
        auto_width: false,
        auto_height: false,
        buffer: None,
    });

    // Run the `:terminal` command inside the popup window's buffer
    let command = "terminal";
    let infos = CmdInfos::builder().cmd(command).build();
    let opts = CmdOpts::builder().output(false).build();
    let _ = vim_cmd(&infos, &opts);
}

///
/// Close all other windwos and keep the current one
///
pub fn kill_other_windows() {
    let windows = list_wins();
    let current_win = Window::current();

    for win in windows {
        if win.handle() != current_win.handle() {
            let _ = win.close(false);
        }
    }
}

///
/// Toggle spell checking
///
pub fn toggle_spell_checking() {
    let current_win = Window::current();
    let opts = OptionOpts::builder().win(current_win).build();
    let toggled_value = !get_option_value::<bool>("spell", &opts).unwrap();
    let _ = set_option_value("spell", toggled_value, &opts);
}

use crate::picker::{PopupWindowOptions, create_popup_window};
use nvim::api::{
    Window, cmd as vim_cmd, get_option_value, list_wins,
    opts::{CmdOpts, OptionOpts},
    set_option_value,
    types::{CmdInfos, WindowBorder},
};
use nvim_oxi::{self as nvim};
