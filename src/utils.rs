use nvim::api::{
    Buffer, Window, cmd as vim_cmd, get_option_value, list_wins, open_win,
    opts::{CmdOpts, OptionOpts, OptionScope},
    set_option_value,
    types::{CmdInfos, WindowBorder, WindowConfig, WindowRelativeTo},
};
use nvim_oxi::{self as nvim};

///
///
///
pub fn open_centred_floating_terminal_window() {
    // Get terminal width and height
    let opts = OptionOpts::builder().scope(OptionScope::Local).build();
    let terminal_width = get_option_value::<u32>("columns", &opts).unwrap();
    let terminal_height = get_option_value::<u32>("lines", &opts).unwrap();

    // Calculate popup window width and height by ratio
    let default_width_ratio = 0.8f32;
    let default_height_ratio = 0.7f32;
    let width = ((terminal_width as f32) * default_width_ratio).floor();
    let height = ((terminal_height as f32) * default_height_ratio).floor();

    // Center window in `editor` area by calculating the (left, top)
    let cols = (((terminal_width as f32 - width) / 2f32).floor()) as u32;
    let rows = (((terminal_height as f32 - height) / 2f32).floor()) as u32;

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
    let _ = open_win(&Buffer::current(), enter_into_window, &open_win_config);

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
