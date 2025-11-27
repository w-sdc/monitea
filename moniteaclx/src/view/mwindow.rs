slint::include_modules!();

pub trait MainWindowAgent {}

// Main window for standalone application
struct mainAppWindow {
    mwinst: MainWindow,
}
