#[stabby::stabby]
#[repr(u8)]
pub enum CloseResponse {
    /// Your plugin accepts that the file will be closed
    Acknowledge,
    /// Your plugin requests that the file be kept open
    Refuse,
}
use stabby::slice::Slice;
#[stabby::stabby(checked)]
pub trait MyTextEditorPlugin {
    extern "C" fn on_editor_opened(&mut self, path: Slice<'_, u8>);
    extern "C" fn on_editor_closing(&mut self, path: Slice<'_, u8>) -> CloseResponse;
}
#[stabby::stabby(checked)]
pub trait MyTextEditorHost {
    extern "C" fn move_cursor(&self, path: Slice<'_, u8>, line: u32, column: u32);
}
pub type Host = stabby::dynptr!(stabby::sync::Arc<dyn MyTextEditorHost>);
pub type Plugin = stabby::dynptr!(stabby::boxed::Box<dyn MyTextEditorPlugin>);
