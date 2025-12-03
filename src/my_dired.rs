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
        let buffer_file_type = get_option_value::<NvimString>("filetype", &opts);
        let buffer_is_hidden = get_option_value::<NvimString>("bufhidden", &opts);
        let buffer_has_swapfile = get_option_value::<bool>("swapfile", &opts);

        #[cfg(feature = "enable_my_dired_debug_print")]
        {
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

        // let _ = set_keymap(
        //     Mode::Normal,
        //     "Y",
        //     "y$",
        //     &SetKeymapOpts::builder().desc("Dired buffer: Open directory or file").build(),
        // );

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
}

///
/// Open the dired buffer
///
fn open() {
    let dired_buffer_handle = get_dired_buffer(true);

    //
    // Get current buffer info
    //
    let current_buffer = Buffer::current();
    let buffer_filename = current_buffer.get_name();
    if let Err(error) = buffer_filename {
        nvim::print!(
            "\n>>> [ my_dired - open ] - Failed to get current buffer filename: {error:?}"
        );
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

use nvim_oxi as nvim;

use nvim::{
    String as NvimString,
    api::{
        Buffer, create_buf, get_option_value, list_bufs,
        opts::{OptionOpts, OptionScope, SetKeymapOpts},
        set_keymap, set_option_value,
        types::Mode,
    },
};
use std::sync::LazyLock;
use std::sync::Mutex;
