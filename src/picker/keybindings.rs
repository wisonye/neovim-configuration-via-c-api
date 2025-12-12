///
/// <c-j>/<c-k>: Move the cursor up and down in the list buffer and set the input buffer text
///
fn ctrl_jk_callback(list_win_ref: &mut Window, is_ctrl_j: bool, input_buffer_ref: &mut Buffer) {
    if let Ok(cursor_pos) = &list_win_ref.get_cursor() {
        let mut row = cursor_pos.0;
        let col = cursor_pos.1;

        if !is_ctrl_j && row == 1 {
            return;
        }

        // Set the input window text to current line from list window
        if let Ok(list_buffer) = list_win_ref.get_buf() {
            let mut read_line_range: std::ops::Range<usize> = row..row + 1;
            if !is_ctrl_j && row >= 2 {
                read_line_range = row - 2..row - 1;
            }
            if let Ok(mut lines) = list_buffer.get_lines(read_line_range.clone(), true) {
                if let Some(first_line) = lines.next() {
                    let _ =
                        input_buffer_ref.set_lines(.., true, vec![first_line.to_str().unwrap()]);
                }
            }

            // Update list window cursor
            if let Ok(line_count) = list_buffer.line_count() {
                match is_ctrl_j {
                    true if row < line_count => row += 1,
                    false if row > 1 => row -= 1,
                    _ => {}
                }
            }
        }

        let _ = list_win_ref.set_cursor(row, col);
    }
}

///
/// <CR>: Add input into the list buffer IF it doesn't exists, and then trigger callback.
///
fn enter_callback<F>(
    title_window_handle: i32,
    input_window_handle: i32,
    list_window_handle: i32,
    mut selected_callback: F,
) where
    F: FnMut(String) + Clone + 'static,
{
    #[cfg(feature = "enable_picker_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ picker keybindings - enter_callback ]";

    let title_window = Window::from(title_window_handle);
    let input_window = Window::from(input_window_handle);
    let list_window = Window::from(list_window_handle);

    let mut selected_text = String::from("");

    if let Ok(input_buffer) = input_window.get_buf() {
        #[cfg(feature = "enable_picker_debug_print")]
        match input_buffer.get_lines(0..1, true) {
            Ok(mut value) => {
                while let Some(v) = value.next() {
                    nvim::print!("\n>>> {LOGGER_PREFIX} line: {}", v);
                }
            }
            Err(e) => {
                nvim::print!("\n>>> {LOGGER_PREFIX} get_lines_result failed: {e:?}");
            }
        }

        if let Ok(mut lines) = input_buffer.get_lines(0..1, true) {
            if let Some(first_line) = lines.next() {
                selected_text = first_line.to_str().unwrap().to_owned();
            }
        }
    }

    // Back to normal mode
    let command = "stopinsert";
    let infos = CmdInfos::builder().cmd(command).build();
    let opts = CmdOpts::builder().output(false).build();
    let _ = vim_cmd(&infos, &opts);

    // Close all windows
    let _ = title_window.close(true);
    let _ = input_window.close(true);
    let _ = list_window.close(true);

    // Call the callback
    selected_callback(selected_text);
}

///
/// <c-e>: Quit the picker without trigger the `selected_callback`.
///
fn ctrl_e_to_close_the_picker(
    title_window_handle: i32,
    input_window_handle: i32,
    list_window_handle: i32,
) {

    // Back to normal mode
    let command = "stopinsert";
    let infos = CmdInfos::builder().cmd(command).build();
    let opts = CmdOpts::builder().output(false).build();
    let _ = vim_cmd(&infos, &opts);

    // Close all windows
    let _ = Window::from(title_window_handle).close(true);
    let _ = Window::from(input_window_handle).close(true);
    let _ = Window::from(list_window_handle).close(true);
}

///
/// Set the following keybindings for the input buffer:
///
/// - <c-j>/<c-k>: Move the cursor up and down in the list buffer.
/// - <CR>: Add input into the list buffer IF it doesn't exists, and then trigger callback.
/// - <c-e>: Quit the picker without trigger the `selected_callback`.
///
pub fn set_input_buffer_keybindings<F>(
    title_window_handle: i32,
    input_window_handle: i32,
    list_window_handle: i32,
    selected_callback: F,
) where
    F: FnMut(String) + Clone + 'static,
{
    if title_window_handle == -1 || input_window_handle == -1 || list_window_handle == -1 {
        return;
    }

    let input_window = Window::from(input_window_handle);

    let mut input_buffer = input_window.get_buf().unwrap();
    let input_buffer_handle = input_buffer.handle();

    let selected_callback_cloned = selected_callback.clone();
    let my_keybindings_with_callback: Vec<(Mode, &str, &str, Box<dyn Fn()>)> = vec![
        (
            Mode::Insert,
            "<CR>",
            "Press ENTER to select",
            Box::new(move || {
                enter_callback(
                    title_window_handle,
                    input_window_handle,
                    list_window_handle,
                    selected_callback_cloned.clone(),
                )
            }),
        ),
        (
            Mode::Normal,
            "<CR>",
            "Press ENTER to select",
            Box::new(move || {
                enter_callback(
                    title_window_handle,
                    input_window_handle,
                    list_window_handle,
                    selected_callback.clone(),
                )
            }),
        ),
        (
            Mode::Insert,
            "<c-j>",
            "'<c-j>' to move down",
            Box::new(move || {
                ctrl_jk_callback.clone()(
                    &mut Window::from(list_window_handle),
                    true,
                    &mut Buffer::from(input_buffer_handle),
                );
            }),
        ),
        (
            Mode::Normal,
            "<c-j>",
            "'<c-j>' to move down",
            Box::new(move || {
                ctrl_jk_callback.clone()(
                    &mut Window::from(list_window_handle),
                    true,
                    &mut Buffer::from(input_buffer_handle),
                );
            }),
        ),
        (
            Mode::Insert,
            "<c-k>",
            "'<c-k>' to move up",
            Box::new(move || {
                ctrl_jk_callback(
                    &mut Window::from(list_window_handle),
                    false,
                    &mut Buffer::from(input_buffer_handle),
                );
            }),
        ),
        (
            Mode::Normal,
            "<c-k>",
            "'<c-k>' to move up",
            Box::new(move || {
                ctrl_jk_callback(
                    &mut Window::from(list_window_handle),
                    false,
                    &mut Buffer::from(input_buffer_handle),
                );
            }),
        ),
        (
            Mode::Normal,
            "<c-e>",
            "'<c-e>' to close the picker",
            Box::new(move || {
                ctrl_e_to_close_the_picker(
                    title_window_handle,
                    input_window_handle,
                    list_window_handle,
                );
            }),
        ),
        (
            Mode::Insert,
            "<c-e>",
            "'<c-e>' to close the picker",
            Box::new(move || {
                ctrl_e_to_close_the_picker(
                    title_window_handle,
                    input_window_handle,
                    list_window_handle,
                );
            }),
        ),
    ];

    for bindings in my_keybindings_with_callback {
        let _ = input_buffer.set_keymap(
            bindings.0,
            bindings.1,
            "",
            &SetKeymapOpts::builder()
                .desc(bindings.2)
                .callback(move |_| {
                    bindings.3();
                    ()
                })
                .silent(true)
                .build(),
        );
    }
}

use nvim_oxi::api::{
    Buffer, Window, cmd as vim_cmd,
    opts::{CmdOpts, SetKeymapOpts},
    types::{CmdInfos, Mode},
};

#[cfg(feature = "enable_picker_debug_print")]
use nvim_oxi as nvim;
