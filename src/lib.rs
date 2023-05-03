use js_sys::ArrayBuffer;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use js_sys::Array;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn play_ogg_files(audio_files: Array) -> Result<(), JsValue> {
    let audio_context = AudioContext::new()?;
    let audio_context_rc = Rc::new(RefCell::new(audio_context));
    let base_time = audio_context_rc.borrow().current_time() + 1.0;

    for (index, file) in audio_files.iter().enumerate() {
        let array_buffer = file.dyn_into::<js_sys::ArrayBuffer>()?;
        let schedule_time = base_time + index as f64 * 0.5;

        let audio_context_clone = audio_context_rc.clone();

        let closure = Closure::wrap(Box::new(move |buffer: AudioBuffer| {
            let source = audio_context_clone.borrow().create_buffer_source().unwrap();
            source.set_buffer(Some(&buffer));
            source
                .connect_with_audio_node(&audio_context_clone.borrow().destination())
                .unwrap();
            source.start_with_when(schedule_time).unwrap();
            log(&format!(
                "Playing file {} at time {}",
                index + 1,
                schedule_time
            ));
        }) as Box<dyn FnMut(_)>);

        let onload = closure.as_ref().unchecked_ref();
        audio_context_rc
            .borrow()
            .decode_audio_data_with_success_callback(&array_buffer, onload)?;
        closure.forget();
    }

    Ok(())
}
