#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;

include!(concat!(env!("OUT_DIR"), "/zstd_bindings.rs"));

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(msg: &str);
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    if let Ok(layout) = std::alloc::Layout::from_size_align(size + std::mem::size_of::<usize>(), align) {
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

#[wasm_bindgen]
pub fn create_stream() -> *mut ZSTD_DStream {
    unsafe {
        let stream = ZSTD_createDStream();
        let r = ZSTD_initDStream(stream);
        if ZSTD_isError(r) == 1 {
            panic!("ZSTD error {}", ZSTD_getErrorCode(r));
        }
        stream
    }
}

#[wasm_bindgen]
pub fn decompress(stream: *mut ZSTD_DStream, data: &mut [u8]) -> Vec<u8> {
    let mut input = ZSTD_inBuffer {
        src: data.as_mut_ptr() as *mut std::ffi::c_void,
        size: data.len(),
        pos: 0,
    };
    let buf_out_size = std::cmp::max(unsafe { ZSTD_DStreamOutSize() }, 65536);
    let mut buf_out: Vec<u8> = Vec::with_capacity(buf_out_size);
    unsafe {
        buf_out.set_len(buf_out_size);
    }

    let mut output = ZSTD_outBuffer {
        dst: buf_out.as_mut_ptr() as *mut std::ffi::c_void,
        size: buf_out_size,
        pos: 0,
    };

    while input.pos < input.size {
        let r = unsafe { ZSTD_decompressStream(stream, &mut output, &mut input) };
        if unsafe { ZSTD_isError(r) } == 1 {
            match unsafe { ZSTD_getErrorCode(r) } {
                ZSTD_ErrorCode_ZSTD_error_dstSize_tooSmall => {
                    let new_size = output.size * 2;
                    unsafe {
                        (*(output.dst as *mut Vec<u8>)).set_len(new_size);
                    }
                    output.size = new_size;
                }
                v => {
                    panic!("ZSTD error {}", v);
                }
            }
        }
    }

    buf_out.truncate(output.pos);
    buf_out
}

#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        console_error(&info.to_string());
    }));
}
