///
/// Picker options
///
pub struct PickerOptions {
    window_opts: PopupWindowOptions,
}

///
/// Create picker with the given list
///
fn create_picker_with_options<F>(options: PickerOptions, list: Vec<&str>, selected_callback: F)
where
    F: FnOnce(),
{
}


use crate::picker::popup_window::PopupWindowOptions;

use nvim::api::{
    Buffer, Window, cmd as vim_cmd, get_option_value, list_wins, open_win,
    opts::{CmdOpts, OptionOpts, OptionScope},
    set_option_value,
    types::{CmdInfos, WindowBorder, WindowConfig, WindowRelativeTo},
};
use nvim_oxi::{self as nvim};
