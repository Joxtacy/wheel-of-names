// this file is generated — do not edit it

/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://kit.svelte.dev/docs/configuration#env) (if configured).
 *
 * _Unlike_ [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 *
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 *
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 *
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 *
 * You can override `.env` values from the command line like so:
 *
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module "$env/static/private" {
  export const POSH_CURSOR_COLUMN: string;
  export const PERLBREW_SHELLRC_VERSION: string;
  export const NVM_INC: string;
  export const MANPATH: string;
  export const npm_package_devDependencies_prettier: string;
  export const PERLBREW_VERSION: string;
  export const ZLE_RPROMPT_INDENT: string;
  export const TERM_PROGRAM: string;
  export const npm_package_devDependencies_eslint_plugin_svelte: string;
  export const NODE: string;
  export const PERLBREW_PERL: string;
  export const NVM_CD_FLAGS: string;
  export const DISABLE_AUTO_TITLE: string;
  export const npm_package_devDependencies_prettier_plugin_svelte: string;
  export const npm_package_devDependencies_typescript: string;
  export const INIT_CWD: string;
  export const TERM: string;
  export const SHELL: string;
  export const FIREFOXBIN: string;
  export const npm_package_devDependencies_vite: string;
  export const TMPDIR: string;
  export const HOMEBREW_REPOSITORY: string;
  export const PERL5LIB: string;
  export const npm_package_scripts_lint: string;
  export const TERM_PROGRAM_VERSION: string;
  export const CONDA_PROMPT_MODIFIER: string;
  export const npm_package_scripts_dev: string;
  export const PERL_MB_OPT: string;
  export const JENV_FORCEJAVAHOME: string;
  export const npm_package_private: string;
  export const npm_package_devDependencies__sveltejs_kit: string;
  export const npm_config_registry: string;
  export const ZSH: string;
  export const POWERLINE_COMMAND: string;
  export const PNPM_HOME: string;
  export const npm_package_devDependencies_globals: string;
  export const ZSH_TMUX_TERM: string;
  export const USER: string;
  export const NVM_DIR: string;
  export const LS_COLORS: string;
  export const npm_package_scripts_check_watch: string;
  export const COMMAND_MODE: string;
  export const PNPM_SCRIPT_SRC_DIR: string;
  export const _ZSH_TMUX_FIXED_CONFIG: string;
  export const SSH_AUTH_SOCK: string;
  export const __CF_USER_TEXT_ENCODING: string;
  export const npm_package_devDependencies_eslint: string;
  export const DENO_INSTALL: string;
  export const npm_execpath: string;
  export const ZSH_TMUX_CONFIG: string;
  export const PAGER: string;
  export const npm_package_devDependencies_svelte: string;
  export const TMUX: string;
  export const PERLBREW_ROOT: string;
  export const WEZTERM_EXECUTABLE_DIR: string;
  export const LSCOLORS: string;
  export const npm_config_frozen_lockfile: string;
  export const PATH: string;
  export const LaunchInstanceID: string;
  export const __CFBundleIdentifier: string;
  export const POSH_CURSOR_LINE: string;
  export const LC_COLLATE: string;
  export const PWD: string;
  export const JENV_LOADED: string;
  export const npm_command: string;
  export const JAVA_HOME: string;
  export const npm_package_scripts_preview: string;
  export const EDITOR: string;
  export const npm_lifecycle_event: string;
  export const POSH_SHELL_VERSION: string;
  export const LANG: string;
  export const npm_package_name: string;
  export const npm_package_devDependencies__sveltejs_vite_plugin_svelte: string;
  export const WEZTERM_PANE: string;
  export const NODE_PATH: string;
  export const npm_package_scripts_build: string;
  export const XPC_FLAGS: string;
  export const TMUX_PANE: string;
  export const PERLBREW_HOME: string;
  export const JENV_FORCEJDKHOME: string;
  export const npm_package_devDependencies_eslint_config_prettier: string;
  export const npm_config_node_gyp: string;
  export const XPC_SERVICE_NAME: string;
  export const WEZTERM_UNIX_SOCKET: string;
  export const npm_package_version: string;
  export const npm_package_devDependencies__sveltejs_adapter_auto: string;
  export const npm_package_devDependencies_svelte_check: string;
  export const JDK_HOME: string;
  export const SHLVL: string;
  export const HOME: string;
  export const npm_package_type: string;
  export const PERLBREW_MANPATH: string;
  export const HOMEBREW_PREFIX: string;
  export const WEZTERM_CONFIG_DIR: string;
  export const PERL_LOCAL_LIB_ROOT: string;
  export const POSH_PROMPT_COUNT: string;
  export const PERLBREW_PATH: string;
  export const LOGNAME: string;
  export const LESS: string;
  export const npm_package_scripts_format: string;
  export const JENV_SHELL: string;
  export const npm_lifecycle_script: string;
  export const TMUX_PLUGIN_MANAGER_PATH: string;
  export const NVM_BIN: string;
  export const BUN_INSTALL: string;
  export const npm_config_user_agent: string;
  export const INFOPATH: string;
  export const HOMEBREW_CELLAR: string;
  export const npm_package_devDependencies__types_eslint: string;
  export const WEZTERM_EXECUTABLE: string;
  export const POSH_THEME: string;
  export const POSH_PID: string;
  export const git_status: string;
  export const SECURITYSESSIONID: string;
  export const WEZTERM_CONFIG_FILE: string;
  export const PERL_MM_OPT: string;
  export const NODE_EXTRA_CA_CERTS: string;
  export const npm_package_scripts_check: string;
  export const COLORTERM: string;
  export const npm_node_execpath: string;
}

/**
 * Similar to [`$env/static/private`](https://kit.svelte.dev/docs/modules#$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 *
 * Values are replaced statically at build time.
 *
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module "$env/static/public" {}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://kit.svelte.dev/docs/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://kit.svelte.dev/docs/configuration#env) (if configured).
 *
 * This module cannot be imported into client-side code.
 *
 * Dynamic environment variables cannot be used during prerendering.
 *
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 *
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module "$env/dynamic/private" {
  export const env: {
    POSH_CURSOR_COLUMN: string;
    PERLBREW_SHELLRC_VERSION: string;
    NVM_INC: string;
    MANPATH: string;
    npm_package_devDependencies_prettier: string;
    PERLBREW_VERSION: string;
    ZLE_RPROMPT_INDENT: string;
    TERM_PROGRAM: string;
    npm_package_devDependencies_eslint_plugin_svelte: string;
    NODE: string;
    PERLBREW_PERL: string;
    NVM_CD_FLAGS: string;
    DISABLE_AUTO_TITLE: string;
    npm_package_devDependencies_prettier_plugin_svelte: string;
    npm_package_devDependencies_typescript: string;
    INIT_CWD: string;
    TERM: string;
    SHELL: string;
    FIREFOXBIN: string;
    npm_package_devDependencies_vite: string;
    TMPDIR: string;
    HOMEBREW_REPOSITORY: string;
    PERL5LIB: string;
    npm_package_scripts_lint: string;
    TERM_PROGRAM_VERSION: string;
    CONDA_PROMPT_MODIFIER: string;
    npm_package_scripts_dev: string;
    PERL_MB_OPT: string;
    JENV_FORCEJAVAHOME: string;
    npm_package_private: string;
    npm_package_devDependencies__sveltejs_kit: string;
    npm_config_registry: string;
    ZSH: string;
    POWERLINE_COMMAND: string;
    PNPM_HOME: string;
    npm_package_devDependencies_globals: string;
    ZSH_TMUX_TERM: string;
    USER: string;
    NVM_DIR: string;
    LS_COLORS: string;
    npm_package_scripts_check_watch: string;
    COMMAND_MODE: string;
    PNPM_SCRIPT_SRC_DIR: string;
    _ZSH_TMUX_FIXED_CONFIG: string;
    SSH_AUTH_SOCK: string;
    __CF_USER_TEXT_ENCODING: string;
    npm_package_devDependencies_eslint: string;
    DENO_INSTALL: string;
    npm_execpath: string;
    ZSH_TMUX_CONFIG: string;
    PAGER: string;
    npm_package_devDependencies_svelte: string;
    TMUX: string;
    PERLBREW_ROOT: string;
    WEZTERM_EXECUTABLE_DIR: string;
    LSCOLORS: string;
    npm_config_frozen_lockfile: string;
    PATH: string;
    LaunchInstanceID: string;
    __CFBundleIdentifier: string;
    POSH_CURSOR_LINE: string;
    LC_COLLATE: string;
    PWD: string;
    JENV_LOADED: string;
    npm_command: string;
    JAVA_HOME: string;
    npm_package_scripts_preview: string;
    EDITOR: string;
    npm_lifecycle_event: string;
    POSH_SHELL_VERSION: string;
    LANG: string;
    npm_package_name: string;
    npm_package_devDependencies__sveltejs_vite_plugin_svelte: string;
    WEZTERM_PANE: string;
    NODE_PATH: string;
    npm_package_scripts_build: string;
    XPC_FLAGS: string;
    TMUX_PANE: string;
    PERLBREW_HOME: string;
    JENV_FORCEJDKHOME: string;
    npm_package_devDependencies_eslint_config_prettier: string;
    npm_config_node_gyp: string;
    XPC_SERVICE_NAME: string;
    WEZTERM_UNIX_SOCKET: string;
    npm_package_version: string;
    npm_package_devDependencies__sveltejs_adapter_auto: string;
    npm_package_devDependencies_svelte_check: string;
    JDK_HOME: string;
    SHLVL: string;
    HOME: string;
    npm_package_type: string;
    PERLBREW_MANPATH: string;
    HOMEBREW_PREFIX: string;
    WEZTERM_CONFIG_DIR: string;
    PERL_LOCAL_LIB_ROOT: string;
    POSH_PROMPT_COUNT: string;
    PERLBREW_PATH: string;
    LOGNAME: string;
    LESS: string;
    npm_package_scripts_format: string;
    JENV_SHELL: string;
    npm_lifecycle_script: string;
    TMUX_PLUGIN_MANAGER_PATH: string;
    NVM_BIN: string;
    BUN_INSTALL: string;
    npm_config_user_agent: string;
    INFOPATH: string;
    HOMEBREW_CELLAR: string;
    npm_package_devDependencies__types_eslint: string;
    WEZTERM_EXECUTABLE: string;
    POSH_THEME: string;
    POSH_PID: string;
    git_status: string;
    SECURITYSESSIONID: string;
    WEZTERM_CONFIG_FILE: string;
    PERL_MM_OPT: string;
    NODE_EXTRA_CA_CERTS: string;
    npm_package_scripts_check: string;
    COLORTERM: string;
    npm_node_execpath: string;
    [key: `PUBLIC_${string}`]: undefined;
    [key: `${string}`]: string | undefined;
  };
}

/**
 * Similar to [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 *
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 *
 * Dynamic environment variables cannot be used during prerendering.
 *
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module "$env/dynamic/public" {
  export const env: {
    [key: `PUBLIC_${string}`]: string | undefined;
  };
}
