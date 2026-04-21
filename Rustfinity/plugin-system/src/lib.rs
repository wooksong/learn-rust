pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        if self.plugins.iter().any(|p| p.name() == plugin.name()) {
            panic!("Plugin with name '{}' already exists", plugin.name());
        }

        self.plugins.push(plugin);
    }

    pub fn remove_plugin(&mut self, plugin_name: &str) -> Option<Box<dyn Plugin>> {
        let idx = self.plugins.iter().position(|p| p.name() == plugin_name)?;
        Some(self.plugins.remove(idx))
    }

    pub fn execute_all(&self) {
        for plugin in &self.plugins {
            plugin.execute();
        }
    }
}

// Example usage
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "MyPlugin"
    }
    fn execute(&self) {
        println!("Executing MyPlugin");
    }
}

impl MyPlugin {
    fn new() -> Self {
        Self
    }
}

pub fn main() {
    let mut manager = PluginManager::new();

    manager.add_plugin(Box::new(MyPlugin::new()));
    manager.execute_all();
}
