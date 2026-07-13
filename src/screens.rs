use crossterm::event::KeyEvent;

use enum_dispatch::enum_dispatch;

use crate::{
    add_package_form::AddPackageForm,
    app_action::AppAction,
    append_files_form::AppendFilesForm,
    downloads_screen::DownloadsScreen,
    files_screen::FilesScreen,
    packages_screen::PackagesScreen,
};

#[enum_dispatch]
#[allow(async_fn_in_trait)]
pub trait ScreenHandler {
    async fn handle_keys(&mut self, key: KeyEvent) -> Option<AppAction>;
    fn handle_paste(&mut self, _content: &str) {}
}

#[allow(clippy::large_enum_variant)]
#[enum_dispatch(ScreenHandler)]
#[derive(Clone)]
pub enum Screen {
    Packages(PackagesScreen),
    Files(FilesScreen),
    Downloads(DownloadsScreen),
    AddPackageForm(AddPackageForm),
    AppendFilesForm(AppendFilesForm),
}
