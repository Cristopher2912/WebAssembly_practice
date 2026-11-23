// Módulo WebAssembly mínimo: expone una lista de elementos como texto.
static COSAS: &str = "Docker,Contenedores,Imágenes";

#[no_mangle]
pub extern "C" fn cosas_ptr() -> *const u8 {
    COSAS.as_ptr()
}

#[no_mangle]
pub extern "C" fn cosas_len() -> usize {
    COSAS.len()
}
