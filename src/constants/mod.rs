/// Constants: A collection of compile-time constant values used in multiple
/// places throughout the rest of the code.

/// Common options:
pub const PACKAGES: &'static str = "packages";

/// Configuration options:
pub const CONFIG_FILE: &'static str = "config";
pub const SHORT_CONFIG_FILE: &'static str = "c";
pub const NIX_CONF_PATH: &'static str = "config/config.toml";
pub const NIX_LOG_CONF_PATH: &'static str = "config/log4rs.yaml";

/// Operations:
pub const INSTALL: &'static str = "install";
pub const REMOVE: &'static str = "remove";
pub const UPDATE: &'static str = "update";
pub const UPDATE_ALL: &'static str = "update-all";
pub const SEARCH: &'static str = "search";
pub const SHORT_INSTALL: &'static str = "i";
pub const SHORT_REMOVE: &'static str = "r";
pub const SHORT_UPDATE: &'static str = "u";
pub const SHORT_UPDATE_ALL: &'static str = "a";
pub const SHORT_SEARCH: &'static str = "s";
pub const INSTALL_CMD: &'static str = "install";
pub const REMOVE_CMD: &'static str = "remove";
pub const UPDATE_CMD: &'static str = "update";
pub const SEARCH_CMD: &'static str = "search";

/// AUR information:
pub const AUR_RPC_URL: &'static str = "https://aur.archlinux.org/rpc/?v=5";
pub const AUR_RPC_SEARCH_FMT: &'static str = "&type=search";
pub const AUR_RPC_SEARCH_ARG: &'static str = "&arg=";
pub const AUR_RPC_INFO_FMT: &'static str = "&type=info";
pub const AUR_RPC_INFO_ARG: &'static str = "&arg[]=";
