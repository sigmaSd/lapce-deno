use lapce_plugin::{register_plugin,  start_lsp, LapcePlugin};
use serde::{Deserialize, Serialize};
use serde_json:: Value;

#[derive(Default)]
struct State {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    arch: String,
    os: String,
    configuration: Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    language_id: String,
    options: Option<Value>,
}

register_plugin!(State);

impl LapcePlugin for State {
    fn initialize(&mut self, info: serde_json::Value) {
        let info = serde_json::from_value::<PluginInfo>(info).unwrap();
        dbg!(4);
        start_lsp(&"deno", "typescript", info.configuration.options, true);
    }
}
