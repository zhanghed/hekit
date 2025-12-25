use winres::WindowsResource;

fn main() {
    if cfg!(target_os = "windows") {
        WindowsResource::new()
            .set_icon("src/assets/hekit.ico")
            .compile()
            .unwrap();
    }
}
