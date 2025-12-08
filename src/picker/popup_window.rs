///
///
///
pub struct PopupWindowOptions {
    pub border: WindowBorder,
    pub window_width_ratio: Option<f32>,  // Default is `0.5`
    pub window_height_ratio: Option<f32>, // Default is `0.5`
    pub auto_width: bool,                 // Only works when `window_width_ratio` is `None`
    pub auto_height: bool,                // Only works when `window_height_ratio` is `None`
}

///
///
///
pub struct ScreenSize {
    pub width: u32,
    pub height: u32,
}

///
///
///
#[inline]
pub fn get_screen_size() -> ScreenSize {
    let _opts = OptionOpts::builder().scope(OptionScope::Local).build();
    ScreenSize {
        width: get_option_value::<u32>("columns", &_opts).unwrap(),
        height: get_option_value::<u32>("lines", &_opts).unwrap(),
    }
}

///
/// Create a popup window witht the given buffer
///
pub fn create_popup_window(opts: PopupWindowOptions) -> Option<i32> {
    let screen_size = get_screen_size();

    // Calculate popup window width and height
    let default_width_ratio = match opts.window_width_ratio {
        Some(w_ratio) => w_ratio,
        None => 0.5f32,
    };
    let default_height_ratio = match opts.window_height_ratio {
        Some(h_ratio) => h_ratio,
        None => 0.5f32,
    };
    let width = ((screen_size.width as f32) * default_width_ratio).floor();
    let height = ((screen_size.height as f32) * default_height_ratio).floor();

    // Center window in `editor` area by calculating the (left, top)
    let cols = (((screen_size.width as f32 - width) / 2f32).floor()) as u32;
    let rows = (((screen_size.height as f32 - height) / 2f32).floor()) as u32;

    // // Debug print
    // nvim::print!(
    //     concat!(
    //         "\n>>> Terminal size: {{",
    //         "\n\t terminal_width: {}",
    //         "\n\t terminal_height: {}",
    //         "\n\t popup_width: {}",
    //         "\n\t popup_height: {}",
    //         "\n\t popup_left: {}",
    //         "\n\t popup_right: {}",
    //         "\n}}"
    //     ),
    //     terminal_width,
    //     terminal_height,
    //     width,
    //     height,
    //     cols,
    //     rows,
    // );

    // Open popup window with current buffer
    let enter_into_window = true;
    let open_win_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .width(width as u32)
        .height(height as u32)
        .row(rows)
        .col(cols)
        .border(WindowBorder::Rounded)
        .build();

    match open_win(&Buffer::current(), enter_into_window, &open_win_config) {
        Ok(win) => Some(win.handle()),
        Err(_) => None,
    }
}

#[cfg(feature = "enable_picker_debug_print")]
use nvim_oxi as nvim;

use nvim::api::{
    Buffer, Window, cmd as vim_cmd, get_option_value, list_wins, open_win,
    opts::{CmdOpts, OptionOpts, OptionScope},
    set_option_value,
    types::{CmdInfos, WindowBorder, WindowConfig, WindowRelativeTo},
};
use nvim_oxi::{self as nvim};
