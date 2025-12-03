//! This module brings my Emacs `dired` experiences into `Neovim`.
//!
//! `dired`: "Directory Edit" with the supporting actions:
//!     - open
//!     - delete
//!     - rename
//!     - copy
//!     - ...
//!
//!
//! You can assign a key binding to lazy load and execute this plugin like this:
//!
//! ```lua
//! vim.keymap.set(
//!     "n",
//!     "<C-c>j",
//!     "<cmd>lua require('my_dired').open()<CR>",
//!     {
//!         silent = true,
//!         desc = "Open file explorer with current buffer directory"
//!     }
//! )
//! ```

#[derive(Debug, Default)]
struct MyDiredState {
    last_dired_buffer_dir: String,
}

///
/// Private module-scope state
///
static MY_DIRED_STATE: LazyLock<Mutex<MyDiredState>> =
    LazyLock::new(|| Mutex::new(MyDiredState::default()));

///
/// Get existing dired buffer, or create new one.
///
fn get_dired_buffer(create_new_one_if_not_exists: bool) -> i32 {
    let mut dired_buffer_handle = -1;

    //
    // The special buffer variable key/name will be set to the dired buffer.
    // Then you can read it back to check whether the given buffer is a
    // `Dired Buffer` or not.
    //
    const UNIQUE_DIRED_BUFFER_FLAG: &'static str = "i_am_dired_buffer";

    //
    // Find the existing dired buffer
    //
    let buffer_list = list_bufs().collect::<Vec<Buffer>>();
    for buffer in buffer_list.iter() {
        let is_dired_buffer = buffer.get_var::<bool>(UNIQUE_DIRED_BUFFER_FLAG);

        let opts = OptionOpts::builder().buffer(buffer.clone()).build();
        let buffer_type = get_option_value::<NvimString>("buftype", &opts);

        #[cfg(feature = "enable_my_dired_debug_print")]
        {
            let buffer_file_type = get_option_value::<NvimString>("filetype", &opts);
            let buffer_is_hidden = get_option_value::<NvimString>("bufhidden", &opts);
            let buffer_has_swapfile = get_option_value::<bool>("swapfile", &opts);

            nvim::print!(
                concat!(
                    "\n>>> [ get_dired_buffer ] - buffer info: {{",
                    "\n\tbuffer_no: {:?}",
                    "\n\tis_dired_buffer: {:?}",
                    "\n\tbuffer_type: {:?}",
                    "\n\tbuffer_file_type: {:?}",
                    "\n\tbuffer_is_hidden: {:?}",
                    "\n\tbuffer_has_swapfile: {:?}",
                    "\n}}"
                ),
                buffer.handle(),
                is_dired_buffer,
                buffer_type,
                buffer_file_type,
                buffer_is_hidden,
                buffer_has_swapfile
            );
        }

        match (buffer_type, is_dired_buffer) {
            (Ok(temp_buffer_type), Ok(yes_it_is)) => {
                if temp_buffer_type == "nowrite" && yes_it_is {
                    dired_buffer_handle = buffer.handle();

                    #[cfg(feature = "enable_my_dired_debug_print")]
                    nvim::print!("\n>>> Found existing dired buffer: {dired_buffer_handle}");

                    return dired_buffer_handle;
                }
            }
            _ => {}
        }
    }

    //
    // Create new dired buffer if it's not exists yet
    //
    if dired_buffer_handle == -1
        && create_new_one_if_not_exists
        && let Ok(mut dired_buffer) = create_buf(true, false)
    {
        #[cfg(feature = "enable_my_dired_debug_print")]
        nvim::print!("\n>>> Created new dired buffer: {}", dired_buffer.handle());

        //
        // Set related options
        //
        let opts = OptionOpts::builder().buffer(dired_buffer.clone()).build();

        let _ = set_option_value("buftype", "nowrite", &opts);
        let _ = set_option_value("bufhidden", "hide", &opts);
        let _ = set_option_value("swapfile", false, &opts);

        // Allow to modify before finishing the command
        let _ = set_option_value("modifiable", true, &opts);

        // This enables the shell syntax color
        let _ = set_option_value("filetype", "fish", &opts);

        // Set unique buffer flag
        let _ = dired_buffer.set_var(UNIQUE_DIRED_BUFFER_FLAG, true);

        //
        // Setup local buffer keybindings
        //
        let _ = dired_buffer.set_keymap(
            Mode::Normal,
            "h",
            "",
            &SetKeymapOpts::builder()
                .desc("Dired buffer: Go to parent directory")
                .callback(|_| {
                    go_parent_directory();
                    // nvim::print!("\n>>> Bind 'h' to 'go_parent_directory()'.");
                    ()
                })
                .build(),
        );

        //
        // Return the newly created dired buffer handle.
        //
        dired_buffer_handle = dired_buffer.handle();
        return dired_buffer_handle;
    }

    dired_buffer_handle
}

///
/// Run ls command and fill the dired buffer and switch it in current window
///
fn list_directories_into_dired_buffer(dired_buffer_handle: i32, dir: &str) {
    let mut dired_buffer = Buffer::from(dired_buffer_handle);

    //
    // CANNOT set buffer name to `dir`, otherwise it will be treated as a
    // built-in directory buffer and be opened into the `netrw` or `nvim-tree`!!!
    //
    // vim.api.nvim_buf_set_name(dired_buffer, buf_info.dir)

    // Allow to modify before finishing the command
    let opts = OptionOpts::builder().buffer(dired_buffer.clone()).build();
    let _ = set_option_value("modifiable", true, &opts);

    match cmd_utils::execute_command(vec!["ls", "-lhta", dir]) {
        cmd_utils::ExecuteCommandResult::Success {
            cmd_desc,
            exit_code,
            output,
        } => {
            let _ = cmd_desc;
            let _ = exit_code;

            #[cfg(feature = "enable_my_dired_debug_print")]
            nvim::print!(
                "\n>>> [ my_dired - list_directories_into_dired_buffer ] ls output: {}",
                output
            );

            //
            // Set dired buffer content
            //
            let dir_title_line = format!("{dir}:");
            let mut dired_buffer_content = vec!["# [ Dired buffer ]", &dir_title_line];
            dired_buffer_content.reserve(100);

            dired_buffer_content.extend(output.split('\n'));

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
            let _ = dired_buffer.set_lines(.., true, dired_buffer_content);

            //
            // Not allow to modify anymore
            //
            let _ = set_option_value("modifiable", false, &opts);

            //
            // Switch to current window and disable spell checking
            //
            let _ = set_current_buf(&dired_buffer);
            let _ = set_option_value("spell", false, &opts);

            //
            // Change working directory to `dir`, so you're able to manipulate files
            // and directories in the current dired_buffer without problem.
            //
            let lcd_command = "lcd";
            let lcd_cmd_info = CmdInfos::builder().cmd(lcd_command).args([dir]).build();
            let lcd_command_opts = CmdOpts::builder().output(false).build();
            let lcd_cmd_result = vim_cmd(&lcd_cmd_info, &lcd_command_opts);
            let _ = &lcd_cmd_result;
            #[cfg(feature = "enable_my_dired_debug_print")]
            nvim::print!(
                "\n>>> [ my_dired - list_directories_into_dired_buffer ] lcd_cmd_result: {:?}",
                lcd_cmd_result
            );

            //
            // Update internal state
            //
            MY_DIRED_STATE.lock().unwrap().last_dired_buffer_dir = dir.to_owned();
        }
        cmd_utils::ExecuteCommandResult::Fail { error_message } => {
            let _ = &error_message;
            #[cfg(feature = "enable_my_dired_debug_print")]
            nvim::print!(
                "\n>>> [ my_dired - list_directories_into_dired_buffer ] error: {}",
                error_message
            );
        }
    }
}

///
/// Open the dired buffer based on the current buffer filename
///
fn open() {
    let dired_buffer_handle = get_dired_buffer(true);

    //
    // Get current buffer info
    //
    let current_buffer = Buffer::current();
    let buffer_filename = current_buffer.get_name();
    if let Err(error) = buffer_filename {
        nvim::print!("\n>>> [ my_dired - open ] Failed to get current buffer filename: {error:?}");
        return;
    }

    let mut dir: &str = "";
    let unwrapped_path = buffer_filename.unwrap();

    if unwrapped_path.is_dir() {
        if let Some(p) = unwrapped_path.to_str() {
            dir = p;
        }
    } else {
        if let Some(parent) = unwrapped_path.parent() {
            if let Some(p) = parent.to_str() {
                dir = p;
            }
        }
    }

    let current_dir = std::env::current_dir().unwrap();

    #[cfg(feature = "enable_my_dired_debug_print")]
    nvim::print!(
        concat!(
            "\n>>> [ my_dired - list_directories_into_dired_buffer ] open: {{",
            "\n\tcurrent_buffer: {:?}",
            "\n\tunwrapped_path: {:?}",
            "\n\tdir: {:?}",
            "\n\tcurrent_dir (use this if 'dir' is empty): {:?}",
            "\n}}"
        ),
        current_buffer.handle(),
        unwrapped_path,
        dir,
        current_dir,
    );

    //
    // List and update `dired_buffer`
    //
    list_directories_into_dired_buffer(
        dired_buffer_handle,
        if dir == "" {
            current_dir.to_str().unwrap()
        } else {
            dir
        },
    );
}

///
/// Go back to the parent directory
///
fn go_parent_directory() {
    let dired_buffer_handle = get_dired_buffer(true);

    let mut dir_to_open: Option<String> = None;

    //
    // Because you're trying to read the value from `MY_DIRED_STATE.last_dired_buffer_dir`,
    // that means you have to hold the mutex lock until you done.
    //
    // The point is that `list_directories_into_dired_buffer` tries to write the value to
    // `MY_DIRED_STATE.last_dired_buffer_dir`, that said it tries to hold the same mutex
    // lock during the call. That's exactly a `DEAD LOCK` happends!!!
    //
    // To solve this, you have to limit the mutex lock in the smaller scope to guarantee
    // to free the lock before calling `list_directories_into_dired_buffer`!!!
    //
    {
        let mut locked_state = MY_DIRED_STATE.lock();
        let state = locked_state.as_mut().unwrap();

        #[cfg(feature = "enable_my_dired_debug_print")]
        nvim::print!(
            "\n>>> [ my_dired - go_parent_directory ] state.last_dired_buffer_dir: {:?}",
            state.last_dired_buffer_dir
        );

        if state.last_dired_buffer_dir == "" {
            return;
        }

        let current_path = std::path::PathBuf::from(&state.last_dired_buffer_dir);
        if let Some(parent_dir) = current_path.parent() {
            #[cfg(feature = "enable_my_dired_debug_print")]
            nvim::print!(
                concat!(
                    "\n>>> [ my_dired - go_parent_directory ] {{",
                    "\n\tstate.last_dired_buffer_dir: {:?}",
                    "\n\tparent_dir: {:?}",
                    "\n}}"
                ),
                state.last_dired_buffer_dir,
                parent_dir
            );

            if let Some(dir) = parent_dir.to_str() {
                dir_to_open = Some(String::from(dir));
            }
        }
    }

    //
    // List and update `dired_buffer`
    //
    if let Some(dir) = dir_to_open {
        #[cfg(feature = "enable_my_dired_debug_print")]
        nvim::print!("\n>>> [ my_dired - go_parent_directory ] dir: {dir}",);

        list_directories_into_dired_buffer(dired_buffer_handle, &dir);
    }
}

///
///
///
pub fn setup() {
    let _ = set_keymap(
        Mode::Normal,
        "<C-c>j",
        "",
        &SetKeymapOpts::builder()
            .desc("Open my dired with current buffer directory")
            .silent(true)
            .callback(|_| {
                open();
                ()
            })
            .build(),
    );
}

use nvim::{
    String as NvimString,
    api::{
        Buffer, cmd as vim_cmd, create_buf, get_option_value, list_bufs,
        opts::{CmdOpts, OptionOpts, SetKeymapOpts},
        set_current_buf, set_keymap, set_option_value,
        types::{CmdInfos, Mode},
    },
};
use nvim_oxi as nvim;
use rust_utils::cmd as cmd_utils;
use std::sync::LazyLock;
use std::sync::Mutex;
