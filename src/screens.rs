use crate::add_package_form::AddPackageForm;
use crate::files_screen::FilesScreen;
use crate::packages_screen::PackagesScreen;

pub enum CurrentScreen {
    Packages,
    Files,
    AddPackageForm,
    AppendFilesForm,
}

pub struct Screens {
    pub packages: Option<PackagesScreen>,
    pub files: Option<FilesScreen>,
    pub add_package_form: Option<AddPackageForm>,
}