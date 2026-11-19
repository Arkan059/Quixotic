fn main() {
    println!("cargo:rerun-if-changed=icon.ico");
    #[cfg(windows)]
    {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("icon.ico");
        resource.set("ProductName", "Quixotic");
        resource.set("FileDescription", "Quixotic");
        resource.set("LegalCopyright", "created by arkanquixote");
        if let Err(error) = resource.compile() {
            println!("cargo:warning=could not embed the Windows icon: {error}");
        }
    }
}
