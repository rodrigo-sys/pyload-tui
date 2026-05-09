use crate::packages_screen::PackagesScreen;

pub enum CurrentScreen {
    Packages(PackagesScreen),
    Files,
    AddPackageForm,
    AppendFilesForm,
}
