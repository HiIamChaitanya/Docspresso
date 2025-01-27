// src/main.rs
use my_object::*;
use cxx_qt::CxxQtEngine;
use std::rc::Rc;

fn main() {
    env_logger::init();
    let mut engine = CxxQtEngine::new();
    let my_object = Rc::new(MyObject::default());
    engine.set_context_property("myObject", &my_object);
    engine.load_file("src/main.qml"); // Load your QML file
    engine.exec();
}
