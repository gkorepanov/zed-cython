use std::{env, fs};

use zed_extension_api::{self as zed, settings::LspSettings};

const LANGUAGE_SERVER_ID: &str = "cyright";
const BUNDLED_CYRIGHT_ENTRYPOINT: &[&str] =
    &["cyright", "packages", "pyright", "langserver.index.js"];

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

        let lsp_settings =
            LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree).unwrap_or_default();
        let env = command_env(worktree, &lsp_settings);

        if let Some(binary) = lsp_settings.binary {
            if let Some(path) = binary.path {
                let args = binary.arguments.unwrap_or_else(|| vec!["--stdio".into()]);
                return cyright_command(path, args, env);
            }
        }

        let path = bundled_cyright_entrypoint()?;
        cyright_command(path, vec!["--stdio".into()], env)
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

fn command_env(worktree: &zed::Worktree, lsp_settings: &LspSettings) -> zed::EnvVars {
    let mut env = worktree.shell_env();
    if let Some(binary_env) = lsp_settings
        .binary
        .as_ref()
        .and_then(|binary| binary.env.as_ref())
    {
        env.extend(
            binary_env
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        );
    }
    env
}

fn cyright_command(
    path: String,
    args: Vec<String>,
    env: zed::EnvVars,
) -> zed::Result<zed::Command> {
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

fn bundled_cyright_entrypoint() -> zed::Result<String> {
    let path = BUNDLED_CYRIGHT_ENTRYPOINT.iter().fold(
        env::current_dir().map_err(|err| err.to_string())?,
        |path, segment| path.join(segment),
    );

    if fs::metadata(&path).is_ok_and(|metadata| metadata.is_file()) {
        return Ok(path.to_string_lossy().to_string());
    }

    Err(format!(
        "Cyright is not bundled at `{}`. Build a distributable bundle or set `lsp.cyright.binary.path`.",
        path.display()
    ))
}

zed::register_extension!(CythonExtension);
