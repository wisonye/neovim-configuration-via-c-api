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
    const LOGGER_PREFIX: &'static str = "[ my_dired - get_dired_buffer ]";

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
                    "\n>>> {} buffer info: {{",
                    "\n\tbuffer_no: {:?}",
                    "\n\tis_dired_buffer: {:?}",
                    "\n\tbuffer_type: {:?}",
                    "\n\tbuffer_file_type: {:?}",
                    "\n\tbuffer_is_hidden: {:?}",
                    "\n\tbuffer_has_swapfile: {:?}",
                    "\n}}"
                ),
                LOGGER_PREFIX,
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
                    nvim::print!(
                        "\n>>> {LOGGER_PREFIX} Found existing dired buffer: {dired_buffer_handle}"
                    );

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
        nvim::print!(
            "\n>>> {LOGGER_PREFIX} Created new dired buffer: {}",
            dired_buffer.handle()
        );

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
                    ()
                })
                .build(),
        );
        let _ = dired_buffer.set_keymap(
            Mode::Normal,
            "l",
            "",
            &SetKeymapOpts::builder()
                .desc("Dired buffer: Open directory or file")
                .callback(|_| {
                    open_directory_or_file();
                    ()
                })
                .build(),
        );
        let _ = dired_buffer.set_keymap(
            Mode::Normal,
            "<CR>",
            "",
            &SetKeymapOpts::builder()
                .desc("Dired buffer: Open directory or file")
                .callback(|_| {
                    open_directory_or_file();
                    ()
                })
                .build(),
        );
        let _ = dired_buffer.set_keymap(
            Mode::Normal,
            "A",
            "",
            &SetKeymapOpts::builder()
                .desc("Dired buffer: Create file or directory")
                .callback(|_| {
                    create();
                    ()
                })
                .build(),
        );
        let _ = dired_buffer.set_keymap(
            Mode::Normal,
            "C",
            "",
            &SetKeymapOpts::builder()
                .desc("Dired buffer: Copy file or directory")
                .callback(|_| {
                    copy();
                    ()
                })
                .build(),
        );
        let _ = dired_buffer.set_keymap(
            Mode::Normal,
            "R",
            "",
            &SetKeymapOpts::builder()
                .desc("Dired buffer: Rename file or directory")
                .callback(|_| {
                    rename();
                    ()
                })
                .build(),
        );
        let _ = dired_buffer.set_keymap(
            Mode::Normal,
            "D",
            "",
            &SetKeymapOpts::builder()
                .desc("Dired buffer: Delete file or directory")
                .callback(|_| {
                    delete();
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
    const LOGGER_PREFIX: &'static str = "[ my_dired - list_directories_into_dired_buffer ]";

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
            nvim::print!("\n>>> {LOGGER_PREFIX}  ls output: {}", output);

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
            nvim::print!("\n>>> {LOGGER_PREFIX} lcd_cmd_result: {:?}", lcd_cmd_result);

            //
            // Update internal state
            //
            MY_DIRED_STATE.lock().unwrap().last_dired_buffer_dir = dir.to_owned();
        }
        cmd_utils::ExecuteCommandResult::Fail { error_message } => {
            let _ = &error_message;
            #[cfg(feature = "enable_my_dired_debug_print")]
            nvim::print!("\n>>> {LOGGER_PREFIX} error: {}", error_message);
        }
    }
}

///
/// Open the dired buffer based on the current buffer filename
///
fn open() {
    const LOGGER_PREFIX: &'static str = "[ my_dired - open ]";

    let dired_buffer_handle = get_dired_buffer(true);

    //
    // Get current buffer info
    //
    let current_buffer = Buffer::current();
    let buffer_filename = current_buffer.get_name();
    if let Err(error) = buffer_filename {
        nvim::print!("\n>>> {LOGGER_PREFIX} Failed to get current buffer filename: {error:?}");
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
            "\n>>> {} {{",
            "\n\tcurrent_buffer: {:?}",
            "\n\tunwrapped_path: {:?}",
            "\n\tdir: {:?}",
            "\n\tcurrent_dir (use this if 'dir' is empty): {:?}",
            "\n}}"
        ),
        LOGGER_PREFIX,
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
    const LOGGER_PREFIX: &'static str = "[ my_dired - go_parent_directory ]";

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
            "\n>>> {LOGGER_PREFIX} state.last_dired_buffer_dir: {:?}",
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
                    "\n>>> {} {{",
                    "\n\tstate.last_dired_buffer_dir: {:?}",
                    "\n\tparent_dir: {:?}",
                    "\n}}"
                ),
                LOGGER_PREFIX,
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
        nvim::print!("\n>>> {LOGGER_PREFIX} dir: {dir}",);

        list_directories_into_dired_buffer(dired_buffer_handle, &dir);
    }
}

#[derive(Debug)]
struct CurrentDiredBufferItem {
    dired_buffer_handle: i32,
    name: String,
    is_diretory: bool,
}

///
/// Get back the current item in the dired_buffer, it returns:
/// - `NONE` if not found
/// - `Some(CurrentDiredBufferItem) if found
///
fn get_current_dired_buffer_item() -> Option<CurrentDiredBufferItem> {
    const LOGGER_PREFIX: &'static str = "[ my_dired - get_current_dired_buffer_item ]";

    let dired_buffer_handle = get_dired_buffer(false);
    if dired_buffer_handle == -1 {
        #[cfg(feature = "enable_my_dired_debug_print")]
        nvim::print!("\n>>> {LOGGER_PREFIX} dired_buffer not found.");

        return None;
    }

    //
    // Check whether the dired_buffer is the current buffer or not
    //
    let current_buffer = Buffer::current();
    if current_buffer.handle() != dired_buffer_handle {
        #[cfg(feature = "enable_my_dired_debug_print")]
        nvim::print!("\n>>> {LOGGER_PREFIX} dired_buffer is NOT the current buffer, abort.");

        return None;
    }

    //
    // Get the current cursor line from the dired_buffer and get the last column
    //
    // Sample:
    // drwxr-xr-x   6 wison wison   13B Dec  4 12:24 .
    // -rw-r--r--   1 wison wison    0B Dec  4 12:24 filename that has space.txt
    // drwxr-xr-x   3 wison wison   14B Dec  4 12:10 lua
    // -rw-r--r--   1 wison wison  1.6K Dec  3 15:34 init.lua
    // drwxr-xr-x   3 wison wison    3B Dec  3 15:34 ~
    // drwxr-xr-x   2 wison wison   43B Dec  3 15:34 undo
    // -rw-r--r--   1 wison wison   35B Dec  3 10:24 my-init.lua
    // drwx------  28 wison wison   29B Dec  1 21:23 ..
    //
    let current_line_result = get_current_line();
    if current_line_result.is_err() {
        #[cfg(feature = "enable_my_dired_debug_print")]
        nvim::print!("\n>>> {LOGGER_PREFIX} Failed to get current line.");

        return None;
    }

    let current_line = current_line_result.unwrap();
    let columns = current_line.split(" ").collect::<Vec<&str>>();
    if columns.len() < 9 {
        #[cfg(feature = "enable_my_dired_debug_print")]
        nvim::print!("\n>>> {LOGGER_PREFIX} dir_cols_len < 9.");

        return None;
    }

    //
    // Get dir/filename: loop backward to find the `time` col and then join the
    // rest colums as `item_name`.
    //
    //Why? That's because sometimes filename has
    // space charactor. e.g.:
    //
    // -rw-r--r--   1 wison wison    0B Dec  4 12:24 filename that has space.txt
    //
    let mut time_col_index = -1;
    for index in (0..columns.len()).rev() {
        if columns[index].find(":").is_some() {
            time_col_index = index as i32;
            break;
        }
    }

    #[cfg(feature = "enable_my_dired_debug_print")]
    nvim::print!("\n>>> {LOGGER_PREFIX} time_col_index: {time_col_index}");

    if time_col_index == -1 {
        #[cfg(feature = "enable_my_dired_debug_print")]
        nvim::print!("\n>>> {LOGGER_PREFIX} Failed to find time column.");

        return None;
    }

    //
    //  -rw-r--r--   1 wison wison    0B Dec  4 12:24 filename that has space.txt
    //
    // For this case: `current_line` filename has a or more space.
    //
    // You CANNOT get the rest string content just by slicing on it:
    // `columns[time_col_index as usize + 1..]`
    //
    // As it gets back like this: `[filename,that,has,space.txt]` (no spaces
    // included)!!!
    //
    // You have to find the `time column` and search its string content to get back
    // the index. Then, slice the rest part of the string to get back the entire
    // filename before you can escape the special characters!!!
    //
    let time_str = columns[time_col_index as usize];
    let time_str_start_index = current_line.find(time_str).unwrap();
    let slice_start_index = time_str_start_index + time_str.len() + 1;
    let rest_part = &current_line[slice_start_index..];

    Some(CurrentDiredBufferItem {
        dired_buffer_handle,

        // You need to escape the `#`, `%`, `$` and ` `
        name: rest_part
            .replace(" ", "\\ ")
            .replace("%", "\\%")
            .replace("#", "\\#")
            .replace("$", "\\$"),

        is_diretory: columns[0].find("d").is_some(),
    })
}

///
/// Go into the current directory or open file
///
fn open_directory_or_file() {
    const LOGGER_PREFIX: &'static str = "[ my_dired - open_directory_or_file ]";

    let current_item = get_current_dired_buffer_item();
    if current_item.is_none() {
        return;
    }

    let item = current_item.unwrap();

    #[cfg(feature = "enable_my_dired_debug_print")]
    nvim::print!("\n>>> {LOGGER_PREFIX} item: {:?}", &item);

    //
    // Don't handle the following cases
    //
    if item.is_diretory && item.name == "." {
        #[cfg(feature = "enable_my_dired_debug_print")]
        nvim::print!("\n>>> {LOGGER_PREFIX} Don't handle '.' dierctory");

        return;
    }

    //
    // `go_parent_directory()`
    //
    if item.is_diretory && item.name == ".." {
        #[allow(unused_assignments)]
        let mut latest_dir: Option<String> = None;
        {
            let locked_state = MY_DIRED_STATE.lock();
            let state = locked_state.as_ref().unwrap();
            latest_dir = Some(state.last_dired_buffer_dir.to_owned());
        }

        if let Some(d) = latest_dir {
            let latest_path = std::path::PathBuf::from(&d);
            if let Some(parent_dir) = latest_path.parent() {
                #[cfg(feature = "enable_my_dired_debug_print")]
                nvim::print!(
                    concat!(
                        "\n>>> {} {{",
                        "\n\tlatest_dir: {:?}",
                        "\n\tparent_dir: {:?}",
                        "\n}}"
                    ),
                    LOGGER_PREFIX,
                    d,
                    parent_dir
                );

                if let Some(dir) = parent_dir.to_str() {
                    list_directories_into_dired_buffer(item.dired_buffer_handle, &dir);
                }
            }
        }
    }
    //
    // Go to the given directory
    //
    else if item.is_diretory && item.name != ".." {
        #[allow(unused_assignments)]
        let mut latest_dir: Option<String> = None;
        {
            let locked_state = MY_DIRED_STATE.lock();
            let state = locked_state.as_ref().unwrap();
            latest_dir = Some(state.last_dired_buffer_dir.to_owned());
        }

        if let Some(d) = latest_dir {
            let mut new_path = std::path::PathBuf::from(&d);
            new_path.push(&item.name);

            if let Some(dir) = new_path.to_str() {
                #[cfg(feature = "enable_my_dired_debug_print")]
                nvim::print!(
                    concat!(
                        "\n>>> {} {{",
                        "\n\tlatest_dir: {:?}",
                        "\n\titem_name: {:?}",
                        "\n\tdir_to_open: {:?}",
                        "\n}}"
                    ),
                    LOGGER_PREFIX,
                    d,
                    &item.name,
                    dir
                );

                list_directories_into_dired_buffer(item.dired_buffer_handle, &dir);
            }
        }
    }
    //
    // Open file into a new buffer
    //
    else if !item.is_diretory && item.name != "" {
        #[allow(unused_assignments)]
        let mut latest_dir: Option<String> = None;
        {
            let locked_state = MY_DIRED_STATE.lock();
            let state = locked_state.as_ref().unwrap();
            latest_dir = Some(state.last_dired_buffer_dir.to_owned());
        }

        if let Some(d) = latest_dir {
            let mut file_path = std::path::PathBuf::from(&d);
            file_path.push(&item.name);

            if let Some(filename) = file_path.to_str() {
                #[cfg(feature = "enable_my_dired_debug_print")]
                nvim::print!(
                    concat!(
                        "\n>>> {} {{",
                        "\n\tlatest_dir: {:?}",
                        "\n\titem_name: {:?}",
                        "\n\tfile_to_open: {:?}",
                        "\n}}"
                    ),
                    LOGGER_PREFIX,
                    d,
                    &item.name,
                    filename
                );

                if let Ok(new_buffer) = create_buf(true, false) {
                    let _ = set_current_buf(&new_buffer);

                    let edit_command = "edit";
                    let edit_cmd_info = CmdInfos::builder()
                        .cmd(edit_command)
                        .args([filename])
                        .build();
                    let edit_command_opts = CmdOpts::builder().output(false).build();
                    let edit_cmd_result = vim_cmd(&edit_cmd_info, &edit_command_opts);
                    let _ = &edit_cmd_result;
                    #[cfg(feature = "enable_my_dired_debug_print")]
                    nvim::print!(
                        "\n>>> {LOGGER_PREFIX} edit_cmd_result: {:?}",
                        edit_cmd_result
                    );
                }
            }
        }
    }
}

///
///
///
#[derive(Debug)]
enum MyDiredItemAction {
    Copy,
    Create,
    Delete,
    Rename,
}

///
///
///
fn run_action_on_dired_buffer_item(action: MyDiredItemAction) {
    const LOGGER_PREFIX: &'static str = "run_action_on_dired_buffer_item";

    #[cfg(feature = "enable_my_dired_debug_print")]
    nvim::print!("\n>>> {LOGGER_PREFIX} action: {action:?}");

    #[allow(unused_assignments)]
    let mut current_item: Option<CurrentDiredBufferItem> = None;
    let mut dired_buffer_handle = -1;

    match action {
        MyDiredItemAction::Copy | MyDiredItemAction::Delete | MyDiredItemAction::Rename => {
            current_item = get_current_dired_buffer_item();
            if current_item.is_none() {
                return;
            }

            //
            // Don't handle the following cases
            //
            let item = current_item.as_ref().unwrap();
            if item.is_diretory && (item.name == "." || item.name == "..") {
                #[cfg(feature = "enable_my_dired_debug_print")]
                nvim::print!("\n>>> {LOGGER_PREFIX} doesn't handle '.' or '..' directory");

                return;
            }

            dired_buffer_handle = item.dired_buffer_handle;
        }
        MyDiredItemAction::Create => {
            dired_buffer_handle = get_dired_buffer(false);
            if dired_buffer_handle == -1 {
                #[cfg(feature = "enable_my_dired_debug_print")]
                nvim::print!("\n>>> {LOGGER_PREFIX} 'get_dired_buffer(false)' return '-1'.");

                return;
            }

            if dired_buffer_handle != Buffer::current().handle() {
                #[cfg(feature = "enable_my_dired_debug_print")]
                nvim::print!("\n>>> {LOGGER_PREFIX} dired_buffer is NOT the current buffer, abort");

                return;
            }
        }
        _ => {}
    }

    //
    // Save the internal state and release the mutex lock immediately.
    //
    #[allow(unused_assignments)]
    let mut latest_dir = String::from("");
    {
        latest_dir = MY_DIRED_STATE.lock().unwrap().last_dired_buffer_dir.clone();
    }

    //
    // Show action prompt and create action command
    //
    let mut cmd_vec = Vec::<String>::with_capacity(5);

    match action {
        MyDiredItemAction::Create => {
            //
            // You can call `nvim_oxi::api::call_function()` to call Vimscript function (NOT lua
            // function!!!).
            //
            // You need to provide the generic type when call this function.
            //
            // call_function::<ArgumentsGenericType, ReturnGenericType>(FunctionNameHere, ArgsHere);
            //
            // `ArgsHere` need to match the following syntax:
            // - One param: (A,)
            // - Twe param: (A, B,)
            // - Three param: (A, B, C,)
            // ...
            //
            let prompt = "Create file or directory (end with '/')";
            let eval_result = call_function::<_, String>(
                "luaeval",
                //
                // The magic global "_A" is the placeholder, it use the second argument!!!
                //
                // Example:
                //     :luaeval('_A[1] + _A[2]', [40, 2])
                //   ->: luaeval('40 + 2')
                //
                //     :luaeval('string.match(_A, "[a-z]+")', 'XYXfoo123')
                //   ->:luaeval('string.match( 'XYXfoo123', "[a-z]+")')
                //
                (r#"vim.fn.input({ prompt =  _A })"#, prompt),
            );

            #[cfg(feature = "enable_my_dired_debug_print")]
            nvim::print!("\n>>> {LOGGER_PREFIX} eval_result: {eval_result:?}");

            if let Ok(new_item) = eval_result {
                if new_item == "" {
                    return;
                }

                let item_bytes = new_item.as_bytes();
                let is_dir_char = item_bytes[item_bytes.len() - 1usize] == '/' as u8;

                #[cfg(feature = "enable_my_dired_debug_print")]
                nvim::print!(
                    "\n>>> {LOGGER_PREFIX} action: 'create', new_item: {}, is_dir: {}",
                    new_item,
                    is_dir_char
                );

                if is_dir_char {
                    cmd_vec.push("mkdir".to_string());
                    cmd_vec.push((&new_item[..new_item.len() - 1]).to_owned());
                } else {
                    cmd_vec.push("touch".to_string());
                    cmd_vec.push(new_item);
                }
            }
        }
        MyDiredItemAction::Copy => {
            if current_item.is_none() {
                return;
            }

            let item = current_item.unwrap();
            let action_prompt = if item.is_diretory {
                format!("Copy '{}' and all its contents to: ", item.name)
            } else {
                format!("Copy '{}' to: ", item.name)
            };

            let eval_result = call_function::<_, String>(
                "luaeval",
                (r#"vim.fn.input({ prompt =  _A })"#, action_prompt),
            );

            if let Ok(copied_to_item) = eval_result {
                if copied_to_item != "" {
                    #[cfg(feature = "enable_my_dired_debug_print")]
                    nvim::print!(
                        "\n>>> {LOGGER_PREFIX} action: 'copy', copy '{}' to '{}'",
                        item.name,
                        copied_to_item
                    );

                    cmd_vec.push("cp".to_string());
                    cmd_vec.push("-rf".to_string());
                    cmd_vec.push(item.name);
                    cmd_vec.push(copied_to_item);
                }
            }
        }
        MyDiredItemAction::Rename => {
            if current_item.is_none() {
                return;
            }

            let item = current_item.unwrap();
            let action_prompt = if item.is_diretory {
                format!("Rename '{}' and all its contents to: ", item.name)
            } else {
                format!("Rename '{}' to: ", item.name)
            };

            let eval_result = call_function::<_, String>(
                "luaeval",
                (r#"vim.fn.input({ prompt =  _A })"#, action_prompt),
            );

            if let Ok(rename_to_item) = eval_result {
                if rename_to_item != "" {
                    #[cfg(feature = "enable_my_dired_debug_print")]
                    nvim::print!(
                        "\n>>> {LOGGER_PREFIX} action: 'rename', rename '{}' to '{}'",
                        item.name,
                        rename_to_item
                    );

                    cmd_vec.push("mv".to_string());
                    cmd_vec.push(item.name);
                    cmd_vec.push(rename_to_item);
                }
            }
        }
        MyDiredItemAction::Delete => {
            if current_item.is_none() {
                return;
            }

            let item = current_item.unwrap();
            let action_prompt = if item.is_diretory {
                format!("Are you sure to delete '{}' and all its contents? (y/n)", item.name)
            } else {
                format!("Are you sure to delete '{}'? (y/n)", item.name)
            };

            let eval_result = call_function::<_, String>(
                "luaeval",
                (r#"vim.fn.input({ prompt =  _A })"#, action_prompt),
            );

            if let Ok(delete_confirm) = eval_result {
                if delete_confirm == "y" || delete_confirm == "Y"  {
                    #[cfg(feature = "enable_my_dired_debug_print")]
                    nvim::print!(
                        "\n>>> {LOGGER_PREFIX} action: 'delete', delete '{}'",
                        item.name,
                    );

                    cmd_vec.push("rm".to_string());
 cmd_vec.push("-rf".to_string());
                    cmd_vec.push(item.name);
                }
            }
        }
        _ => {
            nvim::print!("\n>>> {LOGGER_PREFIX} unsupported action: {action:?}");
        }
    }

    //
    // Run command
    //
    #[cfg(feature = "enable_my_dired_debug_print")]
    nvim::print!("\n>>> {LOGGER_PREFIX} cmd_vec: {cmd_vec:?}");

    let temp_cmd_list = cmd_vec.iter().map(|v| v.as_str()).collect();
    match cmd_utils::execute_command(temp_cmd_list) {
        cmd_utils::ExecuteCommandResult::Success {
            cmd_desc,
            exit_code,
            output,
        } => {
            let _ = cmd_desc;
            let _ = exit_code;
            let _ = output;

            if dired_buffer_handle != -1 {
                list_directories_into_dired_buffer(dired_buffer_handle, &latest_dir);
            }
        }
        cmd_utils::ExecuteCommandResult::Fail { error_message } => {
            let _ = &error_message;
            #[cfg(feature = "enable_my_dired_debug_print")]
            nvim::print!("\n>>> {LOGGER_PREFIX} error: {}", error_message);
        }
    }
}

///
/// Delete
///
fn delete() {
    run_action_on_dired_buffer_item(MyDiredItemAction::Delete);
}

//
// Create file or directory
//
fn create() {
    run_action_on_dired_buffer_item(MyDiredItemAction::Create);
}

///
/// Copy
///
fn copy() {
    run_action_on_dired_buffer_item(MyDiredItemAction::Copy);
}

///
/// Rename
///
fn rename() {
    run_action_on_dired_buffer_item(MyDiredItemAction::Rename);
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
        Buffer, call_function, cmd as vim_cmd, create_buf, get_current_line, get_option_value,
        list_bufs,
        opts::{CmdOpts, OptionOpts, SetKeymapOpts},
        set_current_buf, set_keymap, set_option_value,
        types::{CmdInfos, Mode},
    },
};
use nvim_oxi::{self as nvim};
use rust_utils::cmd as cmd_utils;
use std::sync::LazyLock;
use std::sync::Mutex;
