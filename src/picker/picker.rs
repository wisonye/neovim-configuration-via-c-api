///
/// Picker options
///
#[derive(Debug)]
pub struct PickerOptions {
    window_opts: PopupWindowOptions,
    list: Vec<String>,
}

///
///
///
fn create_popup_buffer() -> Result<Buffer, NvimError> {
    #[cfg(feature = "enable_picker_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ picker - create_popup_buffer ]";

    //
    // Create internal buffer with the following options:
    //
    // - No numbers, no sign column, no swapfile,
    // - Not related to any file
    // - Wipe the buffer from the buffer list
    // - Not allow to edit
    //
    let picker_buffer = create_buf(false, false)?;

    #[cfg(feature = "enable_picker_debug_print")]
    nvim::print!(
        "\n>>> {LOGGER_PREFIX} picker_buffer_id: {:?}",
        picker_buffer.handle()
    );

    let buffer_opts = OptionOpts::builder().buffer(picker_buffer.clone()).build();
    let _ = set_option_value("number", false, &buffer_opts);
    let _ = set_option_value("signcolumn", "no", &buffer_opts);
    let _ = set_option_value("swapfile", "no", &buffer_opts);
    let _ = set_option_value("buftype", "nofile", &buffer_opts);
    let _ = set_option_value("bufhidden", "wipe", &buffer_opts);

    Ok(picker_buffer)
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
    let mut picker_buffer = create_popup_buffer()?;
    let picker_buffer_id = picker_buffer.handle();

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
/// Editable picker options
///
#[derive(Debug)]
pub struct EditablePickerOptions<'epo> {
    pub title: String,
    pub window_opts: PopupWindowOptions,
    pub list: &'epo Vec<String>,
}

///
/// Editable picker open result
///
#[derive(Debug)]
pub struct EditablePickerOpenResult {
    pub title_window_handle: i32,
    input_window_handle: i32,
    list_window_handle: i32,
}

///
/// Create an editor picker from the given list, split across three windows with their own buffers
/// like this:
///
/// /-------------------------------------\
/// | Title                               | <-- Title window and buffer.
/// |-------------------------------------|
/// | User input                          | <-- Input window and buffer.
/// |-------------------------------------|
/// | List line 0                         |
/// | List line 1                         | <-- List window and buffer.
/// | List line 2                         |
/// | List line ...                       |
/// \-------------------------------------/
///
/// After creating three buffers and three windows, set the second window as the current window
/// (i.e., give it focus and input). Also, set the following keybindings for the input buffer:
///
/// - <c-j>/<c-k>: Move the cursor up and down in the list buffer.
/// - <CR>: Add input into the list buffer IF it doesn't exists, and then trigger callback.
///
pub fn create_editable_picker_with_options<F>(
    opts: &mut EditablePickerOptions,
    selected_callback: F,
) -> Result<EditablePickerOpenResult, NvimError>
where
    F: FnMut(String) + Clone + 'static,
{
    #[cfg(feature = "enable_picker_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ picker - create_editable_picker_with_options<F> ]";

    const POPUP_WINDOW_AUTO_WIDTH_PADDING_EACH_SIDE: u32 = 2;

    // #[cfg(feature = "enable_picker_debug_print")]
    // nvim::print!("\n>>> {LOGGER_PREFIX} opts: {opts:#?}");

    //
    // Create buffers
    //
    let mut title_buffer = create_popup_buffer()?;
    let input_buffer = create_popup_buffer()?;
    let mut list_buffer = create_popup_buffer()?;

    // Fill list buffer
    let list_content = opts.list.iter().map(|v| v.as_str()).collect::<Vec<&str>>();
    let _ = list_buffer.set_lines(.., true, list_content)?;

    // Fill title buffer and set highlight colors
    let _ = title_buffer.set_lines(.., true, vec![opts.title.clone()])?;

    //
    // Not allow to modify after adding content
    //
    let title_buffer_opts = OptionOpts::builder().buffer(title_buffer.clone()).build();
    let list_buffer_opts = OptionOpts::builder().buffer(list_buffer.clone()).build();
    let _ = set_option_value("modifiable", false, &title_buffer_opts);
    let _ = set_option_value("modifiable", false, &list_buffer_opts);

    //
    // Calculate the outter virtual window size to hold all 3 inner windows
    //
    let screen_size = get_screen_size();

    let default_width_ratio = match opts.window_opts.window_width_ratio {
        Some(w_ratio) => w_ratio,
        None => 0.5f32,
    };
    let default_height_ratio = match opts.window_opts.window_height_ratio {
        Some(h_ratio) => h_ratio,
        None => 0.5f32,
    };
    let mut width = ((screen_size.width as f32) * default_width_ratio).floor();
    let mut height = ((screen_size.height as f32) * default_height_ratio).floor();

    // Auto width logic
    if opts.window_opts.auto_width && opts.window_opts.window_width_ratio.is_none() {
        // Loop through all lines in all buffers to find the longest one
        let mut max_cols = opts.title.len();

        for line in opts.list.iter() {
            if line.len() > max_cols {
                max_cols = line.len();
            }

            // #[cfg(feature = "enable_picker_debug_print")]
            // nvim::print!("\n>>> {LOGGER_PREFIX} max_cols (without paddings): {max_cols}");

            if max_cols > 0 {
                let both_padding = (POPUP_WINDOW_AUTO_WIDTH_PADDING_EACH_SIDE * 2) as f32;
                width = (max_cols as f32 + both_padding) as f32;
            }
        }
    }

    // Auto height logic
    if opts.window_opts.auto_height && opts.window_opts.window_height_ratio.is_none() {
        height = opts.list.len() as f32 + 2.0f32; // 1 line title, 1 line empty input

        // #[cfg(feature = "enable_picker_debug_print")]
        // nvim::print!("\n>>> {LOGGER_PREFIX} max_rows: {height}");
    }

    // #[cfg(feature = "enable_picker_debug_print")]
    // nvim::print!("\n>>> {LOGGER_PREFIX} width: {width}, height: {height}");

    // Center window in `editor` area by calculating the (left, top)
    let cal_width = if opts.window_opts.border == WindowBorder::None {
        width
    } else {
        width + 2.0f32
    };
    let cal_height = if opts.window_opts.border == WindowBorder::None {
        height
    } else {
        height + 4.0f32 // 4 borders!!!
    };
    let left = (((screen_size.width as f32 - cal_width) / 2f32).floor()) as u32;
    let mut top = (((screen_size.height as f32 - cal_height) / 2f32).floor()) as u32;

    // #[cfg(feature = "enable_picker_debug_print")]
    // nvim::print!("\n>>> {LOGGER_PREFIX} cal_width: {cal_width}, cal_height: {cal_height}");

    //
    // Title window
    //
    let mut title_window_handle = -1;

    let title_win_popup_border = WindowBorder::Anal(
        WindowBorderChar::Char(Some('╭')), // Left-top corner
        WindowBorderChar::Char(Some('─')), // Top
        WindowBorderChar::Char(Some('╮')), // Right-top corner
        WindowBorderChar::Char(Some('│')), // Right-vertical
        WindowBorderChar::Char(None),      // Right-bottom corner
        WindowBorderChar::Char(None),      // bottom
        WindowBorderChar::Char(None),      // Left-bottom corner
        WindowBorderChar::Char(Some('│')), // Left-vertical
    );

    let title_window_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .width(width as u32)
        .height(1)
        .row(top)
        .col(left)
        .border(title_win_popup_border)
        .build();

    // #[cfg(feature = "enable_picker_debug_print")]
    // nvim::print!("\n>>> {LOGGER_PREFIX} title_window_config: {title_window_config:#?}");

    if let Ok(title_window) = open_win(&title_buffer, false, &title_window_config) {
        title_window_handle = title_window.handle();

        // Add window left padding
        let _ = set_option_value(
            "foldcolumn",
            POPUP_WINDOW_AUTO_WIDTH_PADDING_EACH_SIDE.to_string(),
            &OptionOpts::builder().win(title_window.clone()).build(),
        );
    }

    //
    // Input window
    //
    let mut input_window_handle = -1;

    let input_win_popup_border = WindowBorder::Anal(
        WindowBorderChar::Char(Some('│')), // Left-top corner
        WindowBorderChar::Char(Some('─')), // Top
        WindowBorderChar::Char(Some('│')), // Right-top corner
        WindowBorderChar::Char(Some('│')), // Right-vertical
        WindowBorderChar::Char(Some('│')), // Right-bottom corner
        WindowBorderChar::Char(Some('─')), // bottom
        WindowBorderChar::Char(Some('│')), // Left-bottom corner
        WindowBorderChar::Char(Some('│')), // Left-vertical
    );

    top += 2;
    let input_window_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .width(width as u32)
        .height(1)
        .row(top)
        .col(left)
        .border(input_win_popup_border)
        .build();

    // #[cfg(feature = "enable_picker_debug_print")]
    // nvim::print!("\n>>> {LOGGER_PREFIX} input_window_config: {input_window_config:#?}");

    if let Ok(input_window) = open_win(&input_buffer, false, &input_window_config) {
        input_window_handle = input_window.handle();

        // Add window left padding
        let _ = set_option_value(
            "foldcolumn",
            POPUP_WINDOW_AUTO_WIDTH_PADDING_EACH_SIDE.to_string(),
            &OptionOpts::builder().win(input_window.clone()).build(),
        );
    }

    //
    // List window
    //
    let mut list_window_handle = -1;

    let list_win_popup_border = WindowBorder::Anal(
        WindowBorderChar::Char(None),      // Left-top corner
        WindowBorderChar::Char(None),      // Top
        WindowBorderChar::Char(None),      // Right-top corner
        WindowBorderChar::Char(Some('│')), // Right-vertical
        WindowBorderChar::Char(Some('╯')), // Right-bottom corner
        WindowBorderChar::Char(Some('─')), // bottom
        WindowBorderChar::Char(Some('╰')), // Left-bottom corner
        WindowBorderChar::Char(Some('│')), // Left-vertical
    );

    top += 3; // title_win height: 1, input_win height: 1
    let list_len = opts.list.len() as u32;
    let list_window_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .width(width as u32)
        .height(if list_len > 0 { list_len } else { 1 })
        .row(top)
        .col(left)
        .border(list_win_popup_border)
        .build();

    // #[cfg(feature = "enable_picker_debug_print")]
    // nvim::print!("\n>>> {LOGGER_PREFIX} list_window_config: {list_window_config:#?}");

    if let Ok(list_window) = open_win(&list_buffer, false, &list_window_config) {
        list_window_handle = list_window.handle();

        //
        // Enable list window cursor line
        //
        let _ = set_option_value(
            "cursorline",
            true,
            &OptionOpts::builder().win(list_window.clone()).build(),
        );

        // Add window left padding
        let _ = set_option_value(
            "foldcolumn",
            POPUP_WINDOW_AUTO_WIDTH_PADDING_EACH_SIDE.to_string(),
            &OptionOpts::builder().win(list_window.clone()).build(),
        );
    }

    //
    // Add left padding to all windows
    //

    //
    // Inupt buffer keybindings:
    //
    let _ = set_input_buffer_keybindings(
        title_window_handle,
        input_window_handle,
        list_window_handle,
        selected_callback,
    );

    //
    // Reset the input window as current window to get focus and input, and go into `INSERT` mode.
    //
    let _ = set_current_win(&Window::from(input_window_handle));
    let command = "startinsert";
    let infos = CmdInfos::builder().cmd(command).build();
    let opts = CmdOpts::builder().output(false).build();
    let _ = vim_cmd(&infos, &opts);

    Ok(EditablePickerOpenResult {
        title_window_handle,
        input_window_handle,
        list_window_handle,
    })
}

///
///
///
fn run_test_picker() {
    #[cfg(feature = "enable_picker_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ picker - run_test_picker ]";

    let _ = create_picker_with_options(
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
                let _ = selected_line;

                #[cfg(feature = "enable_picker_debug_print")]
                nvim::print!(
                    "\n>>> {LOGGER_PREFIX} Pressed ENTER, selected line: {}",
                    selected_line
                );

                let _ = Window::from(picker_window_id).close(false);
            }
        },
    );
}

///
///
///
fn run_test_picker_2() {
    #[cfg(feature = "enable_picker_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ picker - run_test_picker_2 ]";

    let result = create_editable_picker_with_options(
        &mut EditablePickerOptions {
            title: "Project Command ('Ctrl+d' to delete item, 'tab' to fill input)".to_string(),
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
            list: &vec![
                String::from("11111"),
                String::from("22222"),
                String::from("33333"),
                String::from("44444"),
                String::from("./build.sh"),
                String::from("./build_release.sh"),
            ],
        },
        |selected_text: String| {
            #[cfg(feature = "enable_picker_debug_print")]
            nvim::print!("\n>>> {LOGGER_PREFIX} Pressed ENTER, selected_text: {selected_text}",);
        },
    );

    let _ = result;

    #[cfg(feature = "enable_picker_debug_print")]
    nvim::print!("\n>>> {LOGGER_PREFIX} result: {:?}", result);
}

///
///
///
pub fn setup() {
    let picker_keybindings_with_callback: Vec<(Mode, &str, &str, Box<dyn Fn()>)> = vec![(
        Mode::Normal,
        "<leader>tp",
        "'<leader>tp': Test picker.",
        Box::new(|| {
            // run_test_picker();
            run_test_picker_2();
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

use crate::picker::{
    PopupWindowOptions, create_popup_window, get_screen_size,
    keybindings::set_input_buffer_keybindings,
};

use nvim_oxi::{
    BufHandle, WinHandle,
    api::{
        Buffer, Error as NvimError, Window, cmd as vim_cmd, create_buf, get_current_line, open_win,
        opts::{CmdOpts, OptionOpts, SetKeymapOpts},
        set_current_win, set_keymap, set_option_value,
        types::{CmdInfos, Mode, WindowBorder, WindowBorderChar, WindowConfig, WindowRelativeTo},
    },
};

#[cfg(feature = "enable_picker_debug_print")]
use nvim_oxi as nvim;
