use zed_extension_api::{self as zed, Result};

struct FtlExtension;

impl zed::Extension for FtlExtension {
    fn new() -> Self {
        Self
    }

    // Если у вас есть языковой сервер, добавьте его настройку здесь.
    // fn language_server_command(
    //     &mut self,
    //     language_server_id: &zed::LanguageServerId,
    //     worktree: &zed::Worktree,
    // ) -> Result<zed::Command> {
    //     Ok(zed::Command {
    //         command: "path/to/language-server".into(),
    //         args: vec![],
    //         env: Default::default(),
    //     })
    // }
}

zed::register_extension!(FtlExtension);
