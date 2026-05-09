use crate::files_screen::FilesScreen;
use crate::packages_screen::PackagesScreen;

pub enum CurrentScreen {
    Packages(PackagesScreen),
    Files(FilesScreen),
    AddPackageForm,
    AppendFilesForm,
}
