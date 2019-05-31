#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;

include!(concat!(env!("OUT_DIR"), "/zstd_bindings.rs"));

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(msg: &str);
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    if let Ok(layout) =
        std::alloc::Layout::from_size_align(size + std::mem::size_of::<usize>(), align)
    {
        let ptr = std::alloc::alloc(layout);
        *(ptr as *mut usize) = size;
        ptr.add(std::mem::size_of::<usize>())
    } else {
        std::ptr::null_mut::<u8>()
    }
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    let actual = ptr.sub(std::mem::size_of::<usize>());
    let size = *(actual as *mut usize);
    let align = std::mem::align_of::<usize>();
    let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
    std::alloc::dealloc(actual, layout);
}

#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        console_error(&info.to_string());
    }));
}

#[wasm_bindgen]
pub struct DecompressStream {
    stream: *mut ZSTD_DStream,
    buffer: Vec<u8>,
}

unsafe fn handle(r: usize) -> Result<(), JsValue> {
    if ZSTD_isError(r) == 1 {
        let message = std::ffi::CStr::from_ptr(ZSTD_getErrorName(r));
        let message = message.to_str().unwrap();
        let message = &format!("ZSTD error: {}", message);
        Err(JsValue::from_str(message))
    } else {
        Ok(())
    }
}

#[wasm_bindgen]
impl DecompressStream {
    #[wasm_bindgen(constructor)]
    pub fn default() -> Result<DecompressStream, JsValue> {
        let (stream, buffer_size) = unsafe {
            let stream = ZSTD_createDStream();
            let r = ZSTD_initDStream(stream);
            handle(r)?;
            (stream, ZSTD_DStreamOutSize())
        };
        let buffer = vec![0; buffer_size];
        Ok(DecompressStream { stream, buffer })
    }

    pub fn decompress(&mut self, data: &mut [u8]) -> Result<Vec<u8>, JsValue> {
        let mut input = ZSTD_inBuffer {
            src: data.as_mut_ptr() as *mut std::ffi::c_void,
            size: data.len(),
            pos: 0,
        };

        let mut result = Vec::with_capacity(4096);
        loop {
            let out_pos = {
                let mut buf_out = ZSTD_outBuffer {
                    dst: self.buffer.as_mut_ptr() as *mut std::ffi::c_void,
                    size: self.buffer.len(),
                    pos: 0,
                };

                unsafe {
                    let r = ZSTD_decompressStream(self.stream, &mut buf_out, &mut input);
                    handle(r)?;
                    buf_out.pos
                }
            };

            result.extend_from_slice(&self.buffer[0..out_pos]);

            if out_pos < self.buffer.len() {
                break;
            }
        }

        Ok(result)
    }
}
