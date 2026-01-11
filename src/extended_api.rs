#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
unsafe extern "C" {

    ///
    /// Add the missing API: https://neovim.io/doc/user/api.html#nvim_exec_lua()
    /// source: https://github.com/neovim/neovim/blob/ba7e17160df2088251217d059ad3441253b74993/src/nvim/api/vim.c#L524
    ///
    pub(crate) unsafe fn nvim_exec_lua(
        code: NvimStr,
        args: Array,
        arena: *mut Arena,
        err: *mut ErrorStruct,
    ) -> Object;
}

///
/// Copy from `nvim_oxi::lib.rs`
///
macro_rules! choose {
    ($err:expr, ()) => {
        if $err.is_err() {
            Err($err.into())
        } else {
            Ok(())
        }
    };

    ($err:expr, $other:expr) => {
        if $err.is_err() {
            Err($err.into())
        } else {
            $other
        }
    };
}

///
/// Add the missing API: https://neovim.io/doc/user/api.html#nvim_exec_lua()
///
pub fn exec_lua<ExecResult>(code: &str, args: Vec<String>) -> Result<ExecResult>
where
    ExecResult: FromObject,
{
    // let arr = vec!["aa"];
    let mut err = ErrorStruct::new();
    let code_string = nvim_oxi::String::from(code);
    let obj = unsafe {
        let args_array = Array::from_iter(args.into_iter());
        nvim_exec_lua(
            code_string.as_nvim_str(),
            args_array,
            // Array::from_iter([
            //     Object::from("111"),
            //     Object::from("222"),
            // ]),
            arena(),
            &mut err,
        )
    };
    choose!(err, Ok(ExecResult::from_object(obj)?))
}

use nvim_oxi::{Arena, Array, ErrorStruct, NvimStr, Object, Result, arena, conversion::FromObject};
