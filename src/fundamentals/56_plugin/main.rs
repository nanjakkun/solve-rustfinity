/*
Design and implement a plugin system using trait objects. You will:

    Define a Plugin trait that includes methods for initialization and execution.
    Create a PluginManager struct to manage plugins. It should:
        Dynamically load plugins implementing the Plugin trait.
        Allow adding and removing plugins.
        Execute all registered plugins in sequence.

    The Plugin trait should include the following methods:
        fn name(&self) -> &str; - Returns the name of the plugin.
        fn execute(&self); - Executes the plugin's functionality.
    The PluginManager should:
        Have the following methods and associated functions:
            new() -> Self - Creates a new PluginManager instance.
            add_plugin - Adds a plugin to the list.
            remove_plugin - Removes a plugin from the list and returns the removed plugin if found.
            execute_all - Executes all registered plugins.
    If a duplicate plugin is added (with the same name), it should panic with the message "Plugin with name [name] already exists".
*/

pub trait Plugin {
    // 1. Finish the trait
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct MyPlugin {
    name: String,
}

impl MyPlugin {
    pub fn new() -> Self {
        MyPlugin {
            name: String::from(""),
        }
    }
}

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    fn execute(&self) {
        println!("Executing!");
    }
}

pub struct PluginManager {
    // 2. Finish the struct
    // Make fields public
    pub plugins: Vec<Box<dyn Plugin>>,
}

// 3. Implement the PluginManager
impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        if self.plugins.iter().any(|p| p.name() == plugin.name()) {
            panic!("Plugin with name {} already exists", plugin.name());
        }
        self.plugins.push(plugin);
    }
    pub fn remove_plugin(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        self.plugins
            .iter()
            .position(|p| p.name() == name)
            .map(|pos| self.plugins.remove(pos))
    }
    pub fn execute_all(&self) {
        for plugin in &self.plugins {
            plugin.execute();
        }
    }
}

// Example usage
pub fn main() {
    let mut manager = PluginManager::new();

    manager.add_plugin(Box::new(MyPlugin::new()));
    manager.execute_all();
}
