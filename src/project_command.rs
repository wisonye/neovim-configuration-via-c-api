//! A module to show a command list picker based on the current project scope.
//! You can add or select a pre-filled command and run it to compile your project
//! and run the binary; the result appears in a right-split terminal buffer window.
//! If compilation fails, pressing ENTER on the error line will bring you to the
//! source code.
//!
//! ```rust
//!  let _ = set_keymap(
//!      Mode::Normal,
//!      "<leader>pc",
//!      "",
//!      &SetKeymapOpts::builder()
//!          .desc("Project command")
//!          .silent(true)
//!          .callback(|_| {
//!              project_command();
//!              ()
//!          })
//!          .build(),
//!  );
//! ```

#[derive(Debug)]
struct ProjectCommandState {
    cmd_list: Vec<String>,
    //
    // The default selection index from the `cmd_list`
    //
    default_cmd_index: Option<usize>,
}

#[derive(Debug, Default)]
struct ModuleState {
    //
    // project_dir <--> project command state
    //
    cmd_map: HashMap<String, ProjectCommandState>,

    //
    // custom highlight namespace, you don't need to destroy it manually, it's cheap, just an integer!!!
    //
    custom_highlight: Option<u32>,
}

impl ModuleState {
    fn init() -> Self {
        Self {
            cmd_map: HashMap::with_capacity(10),
            custom_highlight: Some(create_namespace("project_command_highlight")),
        }
    }
}

// impl Drop for ModuleState {
//     fn drop(&mut self) {
//         // const LOGGER_PREFIX: &'static str = "[ project_command - ModuleState.drop ]";
//         // nvim_oxi::print!(
//         //     "{LOGGER_PREFIX} delete the 'custom_highlight': {:?}",
//         //     self.custom_highlight
//         // );
//     }
// }

///
/// Private module-scope state
///
static MY_PROJECT_COMMAND_STATE: LazyLock<Mutex<ModuleState>> =
    LazyLock::new(|| Mutex::new(ModuleState::init()));

///
/// Get back all `*.sh` files in the current project directory
///
fn get_project_script_files(project_dir: &str) -> std::io::Result<Vec<String>> {
    let mut script_file_list = std::fs::read_dir(project_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "sh")
                .unwrap_or(false)
        })
        .map(|d| format!("./{}", d.file_name().into_string().unwrap()))
        .collect::<Vec<String>>();

    // Sort by filename
    script_file_list.sort_by(|a, b| a.cmp(b));

    Ok(script_file_list)
}

///
/// Get command buffer, create it if it's not exists yet
///
/// If `open_on_most_left_win` is `true`, open source code on most left window.
///
fn get_command_buffer(open_on_most_left_win: bool) -> Option<Buffer> {
    #[cfg(feature = "enable_project_command_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ project_command - get_command_buffer ]";

    const COMMAND_BUFFER_NAME: &'static str = "Command result";

    let mut command_buffer: Option<Buffer> = None;

    //
    // Find the existing command buffer
    //
    let buffer_list = list_bufs().collect::<Vec<Buffer>>();
    for buffer in buffer_list.iter() {
        let opts = OptionOpts::builder().buffer(buffer.clone()).build();
        let buffer_name = buffer.get_name();
        let buffer_type = get_option_value::<NvimString>("buftype", &opts);
        let buffer_file_type = get_option_value::<NvimString>("filetype", &opts);
        let buffer_is_hidden = get_option_value::<NvimString>("bufhidden", &opts);
        let buffer_has_swapfile = get_option_value::<bool>("swapfile", &opts);

        match (
            buffer_name,
            buffer_type,
            buffer_file_type,
            buffer_is_hidden,
            buffer_has_swapfile,
        ) {
            (Ok(name), Ok(b_type), Ok(file_type), Ok(is_hidden), Ok(has_swapfile)) => {
                #[cfg(feature = "enable_project_command_debug_print")]
                nvim::print!(
                    concat!(
                        "{} {{",
                        "\n\tbuffer_name: {:?}",
                        "\n\tbuffer_type: {:?}",
                        "\n\tbuffer_file_type: {:?}",
                        "\n\tbuffer_has_swapfile: {:?}",
                        "\n\tbuffer_name: {:?}",
                        "\n}}"
                    ),
                    LOGGER_PREFIX,
                    &name,
                    &b_type,
                    &file_type,
                    &is_hidden,
                    &has_swapfile,
                );

                if name.to_str().unwrap().ends_with(COMMAND_BUFFER_NAME)
                    && b_type == "nowrite"
                    && file_type == "fish"
                    && is_hidden == "hide"
                    && !has_swapfile
                {
                    command_buffer = Some(Buffer::from(buffer.handle()));

                    #[cfg(feature = "enable_project_command_debug_print")]
                    if let Some(temp_buffer) = &command_buffer {
                        nvim::print!(
                            "{LOGGER_PREFIX} Found command buffer: {}",
                            temp_buffer.handle()
                        );
                    }

                    return command_buffer;
                }
            }
            _ => {}
        }
    }

    //
    // Create the new one if not exists yet
    //
    if command_buffer.is_none()
        && let Ok(mut new_buffer) = create_buf(true, false)
    {
        #[cfg(feature = "enable_project_command_debug_print")]
        nvim::print!(
            "\n>>> {LOGGER_PREFIX} Created new command buffer: {}",
            new_buffer.handle()
        );

        //
        // Set related options
        //
        let opts = OptionOpts::builder().buffer(new_buffer.clone()).build();

        let _ = set_option_value("buftype", "nowrite", &opts);
        let _ = set_option_value("bufhidden", "hide", &opts);
        let _ = set_option_value("swapfile", false, &opts);

        // Allow to modify before finishing the command
        let _ = set_option_value("modifiable", true, &opts);

        // This enables the shell syntax color
        let _ = set_option_value("filetype", "fish", &opts);

        // Set the name
        let _ = new_buffer.set_name(COMMAND_BUFFER_NAME);

        //
        // Setup local buffer keybindings
        //
        let _ = new_buffer.set_keymap(
            Mode::Normal,
            "<CR>",
            "",
            &SetKeymapOpts::builder()
                .desc("Command result: open error/warning under cursor")
                .callback(move |_| {
                    // go_to_error_or_warning_under_cursor();
                    ()
                })
                .build(),
        );

        command_buffer = Some(new_buffer);
    }

    command_buffer
}

///
/// Execute the command and write the result back to the `command buffer`
///
fn execute_command(project_dir: &str, cmd: &str) {
    #[cfg(feature = "enable_project_command_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ project_command - execute_command ]";

    let mut command_buffer = get_command_buffer(true).unwrap();

    let command_window = match get_split_window(true) {
        Some(mut split_win) => {
            let _ = split_win.set_buf(&command_buffer);
            split_win
        }
        None => {
            //
            // Otherwise, create the new split window with the `command_buffer`
            //
            let command_window_config =
                WindowConfig::builder().split(SplitDirection::Right).build();

            open_win(&command_buffer, true, &command_window_config).unwrap()
        }
    };

    //
    // Set related options
    //
    let buffer_opts = OptionOpts::builder().buffer(command_buffer.clone()).build();

    // Allow to modify before finishing the command
    let _ = set_option_value("modifiable", true, &buffer_opts);

    //
    // Replace the command buffer content to running command and force to redraw
    // to see the buffer change
    //
    let _ = command_buffer.set_lines(.., true, vec![format!("Running command: {cmd}")]);
    let _ = command_window.call(|_| {
        let redraw_command = "redraw";
        let redraw_cmd_info = CmdInfos::builder().cmd(redraw_command).build();
        let redraw_command_opts = CmdOpts::builder().output(false).build();
        let _ = vim_cmd(&redraw_cmd_info, &redraw_command_opts);
    });

    //
    // Create `cmd_list`: the first element is the biniary name, and then all args follow
    //
    let cmd_list = cmd.split(" ").collect::<Vec<&str>>();
    match cmd_utils::execute_command(cmd_list) {
        cmd_utils::ExecuteCommandResult::Success {
            cmd_desc,
            exit_code,
            output,
        } => {
            let _ = cmd_desc;
            let _ = exit_code;
            let _ = output;

            #[cfg(feature = "enable_project_command_debug_print")]
            nvim::print!("\n>>> {LOGGER_PREFIX} cmd output: {output}");

            //
            // You have to split on `\n` before inserting to the command buffer!!!
            //
            let output_lines = output.split("\n").collect::<Vec<&str>>();
            let mut result_list = Vec::with_capacity(output_lines.len() + 3);
            let first_line = format!("Command: {cmd}");
            result_list.push(first_line.as_str());
            result_list.push("-------------------------------------------------------");
            result_list.push("");
            result_list.extend(output_lines);

            let set_lines_result = command_buffer.set_lines(.., true, result_list);
            let _ = set_lines_result;

            // #[cfg(feature = "enable_project_command_debug_print")]
            // nvim::print!("\n>>> {LOGGER_PREFIX} set_lines_result: {set_lines_result:?}");
        }
        cmd_utils::ExecuteCommandResult::Fail { error_message } => {
            let _ = command_buffer.set_lines(.., true, vec![error_message]);
        }
    }

    // Not allow to modify after finishing the command
    let _ = set_option_value("modifiable", false, &buffer_opts);
}

///
///
///
fn picker_selected_callback(project_dir: &str, selected_cmd: String) {
    #[cfg(feature = "enable_project_command_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ project_command - picker_selected_callback ]";

    //
    // Lock the state and get back the cmd list
    //
    let mut locked_state = MY_PROJECT_COMMAND_STATE.lock();
    let module_state = locked_state.as_mut().unwrap();

    let get_state_result = module_state.cmd_map.get_mut(project_dir);
    if get_state_result.is_none() {
        return;
    }

    let state = get_state_result.unwrap();
    let mut cmd = selected_cmd;

    //
    // Pick the first line from the `state.cmd_list` if `cmd` is empty, otherwise, exit
    //
    if cmd.len() == 0 {
        if state.cmd_list.len() == 0 {
            return;
        }

        if let Some(cmd_index) = state.default_cmd_index {
            cmd = state.cmd_list[cmd_index].clone();
        } else {
            cmd = state.cmd_list[0].clone();
        }
    }

    #[cfg(feature = "enable_project_command_debug_print")]
    nvim::print!("{LOGGER_PREFIX} called with '{cmd}'.");

    //
    // Update the cmd list if the cmd doesn't exists
    //
    if state
        .cmd_list
        .iter()
        .find(|&item| item.cmp(&cmd) == core::cmp::Ordering::Equal)
        .iter()
        .count()
        == 0
    {
        state.cmd_list.push(cmd.clone());

        //
        // Remove the empty placeholder line (used for rendering the empty list window) if exists.
        //
        state.cmd_list.retain(|line| !line.is_empty());
    }

    //
    // Update the `default_cmd_index`
    //
    for index in 0..state.cmd_list.len() {
        if state.cmd_list[index].cmp(&cmd) == core::cmp::Ordering::Equal {
            state.default_cmd_index = Some(index);

            #[cfg(feature = "enable_project_command_debug_print")]
            nvim::print!("{LOGGER_PREFIX} update 'default_cmd_index' to: {index}");

            break;
        }
    }

    //
    //
    //
    execute_command(project_dir, &cmd);
}

///
/// Options
///
struct ProjectCommandOptions {
    enable_script_files: bool,
    open_source_on_left_split_win: bool,
}

///
/// Open the project command picker
///
fn open(options: ProjectCommandOptions) {
    #[cfg(feature = "enable_project_command_debug_print")]
    const LOGGER_PREFIX: &'static str = "[ project_command - open ]";

    //
    // TODO:
    //
    // 'project_dir' should be the '.git' folder searching start from the current opened file!!!
    // 'project_dir' should be the '.git' folder searching start from the current opened file!!!
    // 'project_dir' should be the '.git' folder searching start from the current opened file!!!
    //
    let project_dir = match std::env::var("PWD") {
        Ok(current_pwd) => current_pwd,
        Err(_) => "".to_string(),
    };

    #[cfg(feature = "enable_project_command_debug_print")]
    nvim::print!("{LOGGER_PREFIX} project_dir: {project_dir}");

    //
    // Use the temporary scope to create a small mutex lock range!!!
    //
    {
        let mut locked_state = MY_PROJECT_COMMAND_STATE.lock();
        let module_state = locked_state.as_mut().unwrap();
        //
        // Only init the cmd list when it doesn't exists.
        //
        if module_state.cmd_map.get(&project_dir).is_none() {
            if options.enable_script_files {
                if let Ok(script_file_list) = get_project_script_files(&project_dir) {
                    let mut cmd_list = Vec::with_capacity(script_file_list.len());
                    for script_file in script_file_list {
                        cmd_list.push(script_file);
                    }

                    module_state.cmd_map.insert(
                        project_dir.to_string(),
                        ProjectCommandState {
                            cmd_list,
                            default_cmd_index: None,
                        },
                    );
                }
            } else {
                module_state.cmd_map.insert(
                    project_dir.to_string(),
                    ProjectCommandState {
                        cmd_list: Vec::with_capacity(5),
                        default_cmd_index: None,
                    },
                );
            }

            #[cfg(feature = "enable_project_command_debug_print")]
            nvim::print!("{LOGGER_PREFIX} state: {module_state:#?}");
        }
    }

    //
    // Lock the state
    //
    let mut locked_state = MY_PROJECT_COMMAND_STATE.lock();
    let module_state = locked_state.as_mut().unwrap();

    //
    // Get back the `project_dir` cmd list
    //
    if let Some(state) = module_state.cmd_map.get(&project_dir) {
        let cmd_list_len = state.cmd_list.len();
        let mut temp_cmd_list =
            Vec::<String>::with_capacity(if cmd_list_len > 0 { cmd_list_len } else { 1 });

        //
        // If `state.default_cmd_index != None` then put the `default_cmd` on top then follow by the rest
        //
        let display_cmd_list: &Vec<String> = if state.default_cmd_index.is_none() {
            &state.cmd_list
        } else {
            // Put the default cmd on top
            let top_line = state.cmd_list[state.default_cmd_index.unwrap()].clone();
            temp_cmd_list.push(top_line.clone());

            // Copy the reset
            for line in &state.cmd_list {
                if line != &top_line {
                    temp_cmd_list.push(line.clone());
                }
            }

            // return the re-ordered cmd list
            &temp_cmd_list
        };

        //
        // Open the picker
        //
        if let Ok(open_result) = create_editable_picker_with_options(
            &mut EditablePickerOptions {
                title: "Project Command ('Ctrl+e' to close picker)".to_string(),
                window_opts: PopupWindowOptions {
                    border: WindowBorder::Rounded,
                    window_width_ratio: None,
                    window_height_ratio: None,
                    auto_width: true,
                    auto_height: true,
                    buffer: None,
                },
                list: &display_cmd_list,
            },
            move |selected_text: String| {
                picker_selected_callback(&project_dir, selected_text);
            },
        ) {
            let custom_highlight_id = module_state.custom_highlight.unwrap();
            if let Ok(mut title_buffer) = Window::from(open_result.title_window_handle).get_buf() {
                let _ = title_buffer.set_extmark(
                    custom_highlight_id, // namespace ID
                    0,                   // start line/row
                    18,                  // start col
                    &SetExtmarkOpts::builder()
                        .end_line(0)
                        .end_col(24)
                        //
                        // You can run `:h highlight-groups` in Neovim to show all supported
                        // highlight group values.
                        //
                        // .hl_group("TermCursor")
                        .hl_group("Question")
                        .build(),
                );
            }
        };
    };
}

///
///
///
pub fn setup() {
    let _ = set_keymap(
        Mode::Normal,
        "<leader>pc",
        "",
        &SetKeymapOpts::builder()
            .desc("Project command")
            .silent(true)
            .callback(|_| {
                open(ProjectCommandOptions {
                    enable_script_files: true,
                    open_source_on_left_split_win: false,
                });
                ()
            })
            .build(),
    );
}

use crate::{
    picker::{EditablePickerOptions, PopupWindowOptions, create_editable_picker_with_options},
    utils::get_split_window,
};

use rust_utils::cmd as cmd_utils;

use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use nvim_oxi::{
    String as NvimString,
    api::{
        Buffer, Window, cmd as vim_cmd, create_buf, create_namespace, get_option_value, list_bufs,
        open_win,
        opts::{CmdOpts, OptionOpts, SetExtmarkOpts, SetKeymapOpts},
        set_keymap, set_option_value,
        types::{CmdInfos, Mode, SplitDirection, WindowBorder, WindowConfig},
    },
};

#[cfg(feature = "enable_project_command_debug_print")]
use nvim_oxi as nvim;
