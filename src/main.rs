use plugin_core::{Host, MyTextEditorHost, MyTextEditorPluginDynMut, Plugin};

#[stabby::stabby]
struct MyHost;

impl MyTextEditorHost for MyHost {
    extern "C" fn move_cursor(
        &self,
        _path: stabby::slice::Slice<'_, u8>,
        _line: u32,
        _column: u32,
    ) {
        todo!()
    }
}

async fn init_plugin() -> core::result::Result<Plugin, stabby::string::String> {
    unsafe {
        let path = if cfg!(target_os = "linux") {
            "./target/debug/libplugin_a.so"
        } else if cfg!(target_os = "windows") {
            "./target/debug/plugin_a.dll"
        } else if cfg!(target_os = "macos") {
            "./target/debug/libplugin_a.dylib"
        } else {
            ""
        };
        let lib = libloading::Library::new(path).unwrap_or_else(|e| {
            panic!(
                "{e}\n\nreaddir(./target/debug)={:?}",
                std::fs::read_dir("./target/debug")
                    .map(|d| d.map(|f| f.unwrap().file_name()).collect::<Vec<_>>())
            )
        });

        use stabby::libloading::StabbyLibrary;
        let init_plugin: stabby::libloading::Symbol<
            extern "C" fn(Host) -> stabby::result::Result<Plugin, stabby::string::String>,
        > = lib
            .get_stabbied::<extern "C" fn(Host) -> stabby::result::Result<Plugin, stabby::string::String>>(
                b"my_text_editor_init_plugin",
            )
            .unwrap();

        let result = init_plugin(stabby::sync::Arc::new(MyHost).into());

        result.into()
    }
}

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    match init_plugin().await {
        Ok(mut plugin) => {
            plugin.on_editor_opened("./hello".as_bytes().into());
        }
        Err(err) => {
            log::error!("error occured: err: {:?}", err);
        }
    }
}
