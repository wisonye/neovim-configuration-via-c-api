const POPUP_WINDOW_AUTO_WIDTH_PADDING_EACH_SIDE: u32 = 2;
// const POPUP_WINDOW_AUTO_WIDTH_PADDING_EACH_SIDE: u32 = 4;

///
///
///
#[derive(Debug)]
pub struct PopupWindowOptions {
    pub border: WindowBorder,
    pub window_width_ratio: Option<f32>,  // Default is `0.5`
    pub window_height_ratio: Option<f32>, // Default is `0.5`
    pub auto_width: bool,                 // Only works when `window_width_ratio` is `None`
    pub auto_height: bool,                // Only works when `window_height_ratio` is `None`
    pub buffer: Option<BufHandle>,
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
pub fn create_popup_window(opts: &PopupWindowOptions) -> Option<i32> {
    #[cfg(feature = "enable_picker_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ picker - create_popup_window ]";

    #[cfg(feature = "enable_picker_debug_print")]
    nvim::print!("\n>>> {LOGGER_PREFIX} opts: {opts:#?}");

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
    let mut width = ((screen_size.width as f32) * default_width_ratio).floor();
    let mut height = ((screen_size.height as f32) * default_height_ratio).floor();

    //
    // Auto width logic
    //
    if opts.auto_width && opts.window_width_ratio.is_none() {
        let window_buffer = match opts.buffer {
            Some(handle) => &Buffer::from(handle),
            _ => &Buffer::current(),
        };

        #[cfg(feature = "enable_picker_debug_print")]
        nvim::print!(
            "\n>>> {LOGGER_PREFIX} auto_width case, buffer id: {}",
            window_buffer.handle()
        );

        // Loop through all lines to find the longest one
        let mut max_cols = 0;
        if let Ok(lines) = window_buffer.get_lines(.., true) {
            for line in lines {
                if line.len() > max_cols {
                    max_cols = line.len();
                }
            }

            #[cfg(feature = "enable_picker_debug_print")]
            nvim::print!("\n>>> {LOGGER_PREFIX} max_cols: {max_cols}");

            if max_cols > 0 {
                let both_padding = (POPUP_WINDOW_AUTO_WIDTH_PADDING_EACH_SIDE * 2) as f32;
                width = (max_cols as f32 + both_padding) as f32;
            }
        }
    }

    //
    // Auto height logic
    //
    if opts.auto_height && opts.window_height_ratio.is_none() {
        let window_buffer = match opts.buffer {
            Some(handle) => &Buffer::from(handle),
            None => &Buffer::current(),
        };

        #[cfg(feature = "enable_picker_debug_print")]
        nvim::print!(
            "\n>>> {LOGGER_PREFIX} auto_height case, buffer id: {}",
            window_buffer.handle()
        );

        if let Ok(lines) = window_buffer.get_lines(.., true) {
            if lines.len() > 0 {
                height = lines.len() as f32;

                #[cfg(feature = "enable_picker_debug_print")]
                nvim::print!("\n>>> {LOGGER_PREFIX} max_rows: {height}");
            }
        }
    }

    #[cfg(feature = "enable_picker_debug_print")]
    nvim::print!("\n>>> {LOGGER_PREFIX} width: {width}, height: {height}");

    // Center window in `editor` area by calculating the (left, top)
    let cal_width = if opts.border == WindowBorder::None { width } else { width + 2.0f32 };
    let cal_height = if opts.border == WindowBorder::None { height } else { height + 2.0f32 };
    let cols = (((screen_size.width as f32 - cal_width) / 2f32).floor()) as u32;
    let rows = (((screen_size.height as f32 - cal_height) / 2f32).floor()) as u32;

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
        .border(opts.border.clone())
        .build();

    let window_buffer = match opts.buffer {
        Some(handle) => &Buffer::from(handle),
        None => &Buffer::current(),
    };

    let popup_window_result = open_win(&window_buffer, enter_into_window, &open_win_config);

    match popup_window_result {
        Ok(win) => {
            let popup_win_opts = OptionOpts::builder().win(win.clone()).build();

            // Add window left padding
            let _ = set_option_value(
                "foldcolumn",
                POPUP_WINDOW_AUTO_WIDTH_PADDING_EACH_SIDE.to_string(),
                &popup_win_opts,
            );
            return Some(win.handle());
        }

        Err(_) => return None,
    }
}

#[cfg(feature = "enable_picker_debug_print")]
use nvim_oxi as nvim;

use nvim_oxi::{
    BufHandle,
    api::{
        Buffer, get_option_value, open_win,
        opts::{OptionOpts, OptionScope},
        set_option_value,
        types::{WindowBorder, WindowConfig, WindowRelativeTo},
    },
};
