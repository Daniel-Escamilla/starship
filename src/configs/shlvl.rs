use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ShLvlConfig<'a> {
    pub threshold: i64,
    pub format: &'a str,
    pub symbol: &'a str,
    pub repeat: bool,
    pub repeat_offset: u64,
    pub style: &'a str,
    pub disabled: bool,
}

impl Default for ShLvlConfig<'_> {
    fn default() -> Self {
        Self {
            threshold: 2,
            format: "[$symbol$shlvl]($style) ",
            symbol: "↕️  ", // extra space for emoji
            repeat: false,
            repeat_offset: 0,
            style: "bold yellow",
            disabled: true,
        }
    }
}
