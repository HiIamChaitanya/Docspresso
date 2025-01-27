// build.rs
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .file("src/lib.rs") // Path to your Rust code with Qt integration
        .build();
}
