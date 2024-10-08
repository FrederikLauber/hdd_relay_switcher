extern crate windows_exe_info;
use camino::Utf8PathBuf;

fn main(){
    windows_exe_info::icon::icon_ico(Utf8PathBuf::from("../assets/on.ico"));
}