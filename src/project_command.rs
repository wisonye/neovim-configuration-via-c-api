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

#[derive(Debug, Default)]
struct MyProjectCommandState {
    //
    // project_dir <--> command list
    //
    cmd_map: HashMap<String, Vec<String>>,
}

impl MyProjectCommandState {
    fn init() -> Self {
        Self {
            cmd_map: HashMap::with_capacity(10),
        }
    }
}

///
/// Private module-scope state
///
static MY_PROJECT_COMMAND_STATE: LazyLock<Mutex<MyProjectCommandState>> =
    LazyLock::new(|| Mutex::new(MyProjectCommandState::init()));

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
        let state = locked_state.as_mut().unwrap();
        //
        // Only init the cmd list when it doesn't exists.
        //
        if state.cmd_map.get(&project_dir).is_none() {
            if options.enable_script_files {
                if let Ok(script_file_list) = get_project_script_files(&project_dir) {
                    let mut cmd_list = Vec::with_capacity(script_file_list.len());
                    for script_file in script_file_list {
                        cmd_list.push(script_file);
                    }

                    state.cmd_map.insert(project_dir.to_string(), cmd_list);
                }
            } else {
                state
                    .cmd_map
                    .insert(project_dir.to_string(), Vec::with_capacity(5));
            }

            #[cfg(feature = "enable_project_command_debug_print")]
            nvim::print!("{LOGGER_PREFIX} state: {state:#?}");
        }
    }

    //
    // Lock the state
    //
    let mut locked_state = MY_PROJECT_COMMAND_STATE.lock();
    let state = locked_state.as_mut().unwrap();

    //
    // Get back the `project_dir` cmd list
    //
    if let Some(cmd_list) = state.cmd_map.get(&project_dir) {
        let result = create_editable_picker_with_options(
            &mut EditablePickerOptions {
                title: "Project Command ('Ctrl+d' to delete item)".to_string(),
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
                list: &cmd_list,
            },
            |selected_text: String| {
                #[cfg(feature = "enable_picker_debug_print")]
                nvim::print!("\n>>> {LOGGER_PREFIX} Pressed ENTER, selected_text: {selected_text}",);
            },
        );
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
