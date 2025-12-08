///
/// Picker options
///
#[derive(Debug)]
pub struct PickerOptions {
    window_opts: PopupWindowOptions,
    list: Vec<String>,
}

///
/// Create picker with the given list
///
fn create_picker_with_options<F>(
    opts: &mut PickerOptions,
    mut selected_callback: F,
) -> Result<(), NvimError>
where
    F: FnMut(BufHandle, WinHandle) + Clone + 'static,
{
    #[cfg(feature = "enable_picker_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ picker - create_picker_with_options<F> ]";

    //
    // Create internal buffer with the following options:
    //
    // - No numbers, no sign column, no swapfile,
    // - Not related to any file
    // - Wipe the buffer from the buffer list
    // - Not allow to edit
    //
    let mut picker_buffer = create_buf(false, false)?;
    let picker_buffer_id = picker_buffer.handle();

    #[cfg(feature = "enable_picker_debug_print")]
    nvim::print!("\n>>> {LOGGER_PREFIX} picker_buffer_id: {picker_buffer_id:#?}");

    let buffer_opts = OptionOpts::builder().buffer(picker_buffer.clone()).build();
    let _ = set_option_value("number", false, &buffer_opts);
    let _ = set_option_value("signcolumn", "no", &buffer_opts);
    let _ = set_option_value("swapfile", "no", &buffer_opts);
    let _ = set_option_value("buftype", "nofile", &buffer_opts);
    let _ = set_option_value("bufhidden", "wipe", &buffer_opts);

    //
    // The first param `line_range: core::ops::RangeBounds<usize>` represents the
    // ranage of `start line index` and `end line index` in the given neomvim buffer.
    //
    // In Rust, you can use slice syntax like below (zero-based)
    //
    // 0..0 - The first line
    // 1..1 - The sencond line
    // 2..4 - The range of third line to fifth line
    // ..4  - The range of first line to fifth line
    // 2..  - The range of third line to the last line
    // 0..  - The range of first line to the last line
    // ..   - The range of all lines
    //
    let content = opts.list.iter().map(|v| v.as_str()).collect::<Vec<&str>>();
    let _ = picker_buffer.set_lines(.., true, content)?;

    //
    // Not allow to modify after adding content
    //
    let _ = set_option_value("modifiable", false, &buffer_opts);

    //
    // Keybindings
    //
    let _ = picker_buffer.set_keymap(
        Mode::Normal,
        "<CR>",
        "",
        &SetKeymapOpts::builder()
            .desc("Press ENTER to select")
            .callback(move |_| {
                let current_win = Window::current();
                selected_callback(picker_buffer_id, current_win.handle());
                ()
            })
            .silent(true)
            .build(),
    );

    let _ = picker_buffer.set_keymap(
        Mode::Normal,
        "<c-j>",
        "j",
        &SetKeymapOpts::builder()
            .desc("'<c-j>' to move down")
            .noremap(true)
            .silent(false)
            .build(),
    );
    let _ = picker_buffer.set_keymap(
        Mode::Normal,
        "<c-k>",
        "k",
        &SetKeymapOpts::builder()
            .desc("'<c-k>' to move up")
            .noremap(true)
            .silent(false)
            .build(),
    );

    //
    // Open the picker window
    //
    opts.window_opts.buffer = Some(picker_buffer_id);
    if let Some(win_handle) = create_popup_window(&opts.window_opts) {
        let current_window = Window::from(win_handle);
        //
        // Disable default window option:
        //
        // 'fillchars' 'fcs'	string	(default "")
        // 			global or local to window |global-local|
        // 	Characters to fill the statuslines, vertical separators and special
        // 	lines in the window.
        // 	It is a comma-separated list of items.  Each item has a name, a colon
        // 	and the value of that item: |E1511|
        //
        // 	  eob		'~'		empty lines at the end of a buffer
        //
        let window_opts = OptionOpts::builder().win(current_window).build();
        let _ = set_option_value("fillchars", "eob: ", &window_opts);

        //
        // Enable window cursor line
        //
        let _ = set_option_value("cursorline", true, &window_opts);
    }

    Ok(())
}

///
///
///
fn run_test_picker() {
    #[cfg(feature = "enable_picker_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ picker - run_test_picker ]";

    let result = create_picker_with_options(
        &mut PickerOptions {
            window_opts: PopupWindowOptions {
                border: WindowBorder::Rounded,
                // window_width_ratio: Some(0.7),
                // window_height_ratio: Some(0.7),
                window_width_ratio: None,
                window_height_ratio: None,
                auto_width: true,
                auto_height: true,
                buffer: None,
            },
            list: vec![
                String::from("./build.sh"),
                String::from("./build_release.sh"),
            ],
        },
        |picker_buffer_id: BufHandle, picker_window_id: WinHandle| {
            if let Ok(selected_line) = get_current_line() {
                #[cfg(feature = "enable_picker_debug_print")]
                nvim::print!(
                    "\n>>> {LOGGER_PREFIX} Pressed ENTER, selected line: {}",
                    selected_line
                );

                let _ = Window::from(picker_window_id).close(false);
            }
        },
    );

    #[cfg(feature = "enable_picker_debug_print")]
    nvim::print!("\n>>> {LOGGER_PREFIX} result: {:?}", result);
}

///
/// Setup a test picker bindings
///
pub fn setup_picker_bindings() {
    let picker_keybindings_with_callback: Vec<(Mode, &str, &str, Box<dyn Fn()>)> = vec![(
        Mode::Normal,
        "<leader>tp",
        "'<leader>tp': Test picker.",
        Box::new(|| {
            run_test_picker();
        }),
    )];

    for bindings in picker_keybindings_with_callback {
        let _ = set_keymap(
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

use crate::picker::{PopupWindowOptions, create_popup_window};

use nvim::{
    BufHandle, WinHandle,
    api::{
        Error as NvimError, Window, create_buf, get_current_line,
        opts::{OptionOpts, SetKeymapOpts},
        set_keymap, set_option_value,
        types::{Mode, WindowBorder},
    },
};

#[cfg(feature = "enable_picker_debug_print")]
use nvim_oxi as nvim;
