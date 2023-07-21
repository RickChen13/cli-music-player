use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
type EventCallback = Arc<dyn Fn(&str) + Send + Sync>;

/// 修改于
///
/// EventEmitter = "0.0.4"
///
pub struct Mitt {
    _events: Mutex<HashMap<String, HashMap<String, EventCallback>>>,
}

impl Mitt {
    pub fn new() -> Self {
        Mitt {
            _events: Mutex::new(HashMap::new()),
        }
    }

    /// 添加事件监听器
    pub fn on(&self, event: &str, callback: EventCallback) -> String {
        let mut events: std::sync::MutexGuard<
            HashMap<String, HashMap<String, Arc<dyn Fn(&str) + Send + Sync>>>,
        > = self._events.lock().unwrap();
        let callbacks: &mut HashMap<String, Arc<dyn Fn(&str) + Send + Sync>> =
            events.entry(event.to_string()).or_insert(HashMap::new());
        let id = Uuid::new_v4().to_owned();
        callbacks.insert(id.clone().to_string(), callback);
        (&id).to_string()
    }

    /// 移除事件监听器
    pub fn off(&self, event: &str, id: &str) {
        let mut events: std::sync::MutexGuard<
            HashMap<String, HashMap<String, Arc<dyn Fn(&str) + Send + Sync>>>,
        > = self._events.lock().unwrap();
        if let Some(callbacks) = events.get_mut(event) {
            callbacks.remove(id);
        }
    }

    /// 触发事件
    pub fn emit(&self, event: &str, data: String) {
        let events: std::sync::MutexGuard<
            HashMap<String, HashMap<String, Arc<dyn Fn(&str) + Send + Sync>>>,
        > = self._events.lock().unwrap();
        if let Some(callbacks) = events.get(event) {
            for (_id, callback) in callbacks {
                let callback_clone: Arc<dyn Fn(&str) + Send + Sync> = callback.clone();
                let msg: String = data.clone();
                // Spawn a new thread to run each callback asynchronously
                std::thread::spawn(move || {
                    (*callback_clone)(&msg);
                });
            }
        }
    }

    /// 移除所有事件的所有监听器
    pub fn remove_all_listeners(&self, event: &str) {
        let mut events: std::sync::MutexGuard<
            HashMap<String, HashMap<String, Arc<dyn Fn(&str) + Send + Sync>>>,
        > = self._events.lock().unwrap();
        events.remove(event);
    }
}
