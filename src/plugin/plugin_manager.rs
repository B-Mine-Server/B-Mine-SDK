use std::{collections::HashMap, ffi::OsStr, fs, sync::Arc};

use libloading::{Library, Symbol};
use tracing::{debug, error, info};

use super::plugin::PluginTrait;

// 插件管理器
pub struct PluginManager {
    extends: HashMap<String, Arc<Box<dyn PluginTrait>>>,
    loaded_libraries: Vec<Library>,
}
impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            extends: HashMap::new(),
            loaded_libraries: Vec::new(),
        }
    }
    pub fn load_all(&mut self) {
        let r = fs::read_dir("plugins").unwrap();
        for i in r {
            let entity = i
                .map_err(|err| error!("Error reading plugin file: {:?}", err))
                .unwrap();
            let path = entity.path();
            let match_ext = {
                if cfg!(target_os = "windows") {
                    path.extension()
                        .map(|v| v.to_str().unwrap())
                        .unwrap_or("")
                        .eq("dll")
                } else {
                    path.extension()
                        .map(|v| v.to_str().unwrap())
                        .unwrap_or("")
                        .eq("so")
                }
            };
            if path.is_file() && match_ext {
                match unsafe { self.load_extend(path.clone()) } {
                    Ok(_) => {
                        debug!("Load plugin success: {}", path.to_string_lossy());
                    }
                    Err(_err) => {
                        error!("{}", _err);
                    }
                }
            }
        }
    }

    pub unsafe fn load_extend<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<(), String> {
        type SymbolPluginTrait = unsafe fn() -> *mut dyn PluginTrait;
        let lib = match Library::new(filename.as_ref()) {
            Ok(lib) => lib,
            Err(_err) => return Err(_err.to_string()),
        };
        self.loaded_libraries.push(lib);
        let lib = self.loaded_libraries.last().unwrap();
        let constructor: Symbol<SymbolPluginTrait> = match lib.get(b"_app_extend_create") {
            Ok(lib) => lib,
            Err(_err) => {
                return Err(_err.to_string());
            }
        };
        let boxed_raw: *mut dyn PluginTrait = constructor();
        let extend = Box::from_raw(boxed_raw);
        let plugin_config = extend.load();
        info!("加载插件：{}", plugin_config.name);
        self.extends
            .insert(plugin_config.name.clone(), Arc::new(extend));
        Ok(())
    }

    pub async fn select_all(&self, socket: rust_raknet::RaknetSocket, buf: &Vec<u8>) {
        // plugin.receive_packet(buf).clone()
        // 以下代码实现了接收并转发数据
        for plugin_name in self.extends.keys() {
            let plugin = self.select(plugin_name).unwrap();
            let processed_buf = plugin.receive_packet(buf);
            if let Some(processed_buf) = processed_buf {
                match socket.send(&processed_buf.0, processed_buf.1).await {
                    Ok(_) => {
                        debug!("Send to {}: {:?}", plugin_name, processed_buf.0);
                    }
                    Err(_err) => {
                        error!("From to {} error: {:?}", plugin_name, _err);
                    }
                }
            }
        }
    }
    // 指定一个插件进行加载
    fn select<T: Into<String>>(&self, target: T) -> Result<Arc<Box<dyn PluginTrait>>, String> {
        let key: String = target.into();
        self.extends
            .get(&key)
            .map(|v| v.clone())
            .ok_or("Plugin not found.".to_string())
    }
}
