use std::any::Any;

use rust_raknet::Reliability;

// 定义插件trait
pub trait PluginTrait: Any + Send + Sync {
    // 插件加载
    fn load(&self) -> PluginConfig;
    // 处理数据包并返回新的数据包和确认方式
    fn receive_packet(&self, data: &Vec<u8>) -> Option<(Vec<u8>, Reliability)>;
    // TODO：尝试接受事件
}

// 定义插件配置
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub name: String,
    pub version: [u8; 4],
    pub description: String,
}
