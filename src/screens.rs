use crate::add_package_form::AddPackageForm;
use crate::append_files_form::AppendFilesForm;
use crate::files_screen::FilesScreen;
use crate::packages_screen::PackagesScreen;

#[derive(Clone)]
pub enum Screen {
    Packages(PackagesScreen),
    Files(FilesScreen),
    AddPackageForm(AddPackageForm),
    AppendFilesForm(AppendFilesForm),
}
