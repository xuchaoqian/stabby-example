use plugin_core::{Host, MyTextEditorPlugin, Plugin};
use stabby::{boxed::Box, result::Result, string::String};
struct MyPlugin;
impl MyTextEditorPlugin for MyPlugin {
    extern "C" fn on_editor_opened(&mut self, _path: stabby::slice::Slice<'_, u8>) {
        println!("on_editor_opened");
    }

    extern "C" fn on_editor_closing(
        &mut self,
        _path: stabby::slice::Slice<'_, u8>,
    ) -> plugin_core::CloseResponse {
        todo!()
    }
}

#[stabby::export]
pub extern "C" fn my_text_editor_init_plugin(_host: Host) -> Result<Plugin, String> {
    Result::Ok(Box::new(MyPlugin).into())
}
