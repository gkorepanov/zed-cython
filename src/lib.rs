use zed_extension_api::{self as zed, settings::LspSettings};

const LANGUAGE_SERVER_ID: &str = "cyright";

struct CythonExtension;

impl zed::Extension for CythonExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        if language_server_id.as_ref() != LANGUAGE_SERVER_ID {
            return Err(format!("unsupported language server: {language_server_id}"));
        }

        let lsp_settings = LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)
            .map_err(|_| cyright_configuration_error())?;
        let binary = lsp_settings
            .binary
            .ok_or_else(cyright_configuration_error)?;
        let path = binary.path.ok_or_else(cyright_configuration_error)?;
        let args = binary.arguments.unwrap_or_else(|| vec!["--stdio".into()]);
        let env = worktree.shell_env();

        if path.ends_with(".js") {
            let mut node_args = vec![path];
            node_args.extend(args);
            return Ok(zed::Command {
                command: zed::node_binary_path()?,
                args: node_args,
                env,
            });
        }

        Ok(zed::Command {
            command: path,
            args,
            env,
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

fn cyright_configuration_error() -> String {
    "Cyright is not configured. Build Cyright and set `lsp.cyright.binary.path` to its `packages/pyright/langserver.index.js`."
        .into()
}

zed::register_extension!(CythonExtension);
