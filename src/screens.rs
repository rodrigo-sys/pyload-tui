use crate::add_package_form::AddPackageForm;
use crate::append_files_form::AppendFilesForm;
use crate::files_screen::FilesScreen;
use crate::packages_screen::PackagesScreen;

#[derive(PartialEq, Clone, Copy)]
pub enum Screen {
    Packages,
    Files,
    AddPackageForm,
    AppendFilesForm,
}

pub struct Screens {
    pub packages: Option<PackagesScreen>,
    pub files: Option<FilesScreen>,
    pub add_package_form: Option<AddPackageForm>,
    pub append_files_form: Option<AppendFilesForm>,
}
