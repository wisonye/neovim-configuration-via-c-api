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
}

impl ModuleState {
    fn init() -> Self {
        Self {
            cmd_map: HashMap::with_capacity(10),
        }
    }
}

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
        .map(|d| d.file_name().into_string().unwrap())
        .collect::<Vec<String>>();

    // Sort by filename
    script_file_list.sort_by(|a, b| a.cmp(b));

    Ok(script_file_list)
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

    if let Some(state) = module_state.cmd_map.get_mut(project_dir) {
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
    }
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
        let open_result = create_editable_picker_with_options(
            &mut EditablePickerOptions {
                title: "Project Command ('Ctrl+d' to delete item, 'Ctrl+e' to close picker)"
                    .to_string(),
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
        );

        #[cfg(feature = "enable_project_command_debug_print")]
        nvim::print!("{LOGGER_PREFIX} open_result: {open_result:#?}");

        let _ = open_result;
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

use crate::picker::{
    EditablePickerOptions, PopupWindowOptions, create_editable_picker_with_options,
};

use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use nvim_oxi::api::{
    opts::SetKeymapOpts,
    set_keymap,
    types::{Mode, WindowBorder},
};

#[cfg(feature = "enable_project_command_debug_print")]
use nvim_oxi as nvim;
