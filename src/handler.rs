// No args
struct EventHandler {
    // This is static lifetime by default, but it can be implemented otherwise
    callback: Option<Box<dyn FnMut()>>,
}

impl EventHandler {
    fn set_callback(&mut self, cb: impl FnMut()) {}
}

