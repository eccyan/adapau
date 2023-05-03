use js_sys::Array;
use std::cell::{BorrowError, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn play_ogg_files(audio_files: Array) -> Result<(), JsValue> {
    let audio_context = AudioContext::new()?;
    let audio_context_rc = Rc::new(RefCell::new(audio_context));
    let base_time = audio_context_rc.borrow().current_time() + 1.0;

    for (index, file) in audio_files.iter().enumerate() {
        let array_buffer = file.dyn_into::<js_sys::ArrayBuffer>()?;
        let schedule_time = base_time + index as f64 * 0.5;

        let audio_context_clone = audio_context_rc.clone();

        let closure = Closure::wrap(Box::new(move |buffer: AudioBuffer| {
            let audio_context = match audio_context_clone.try_borrow() {
                Ok(audio_context) => audio_context,
                Err(_err) => {
                    log("Error: Could not borrow the AudioContext");
                    return;
                }
            };

            let source = audio_context.create_buffer_source().unwrap();
            source.set_buffer(Some(&buffer));
            source
                .connect_with_audio_node(&audio_context.destination())
                .unwrap();
            source.start_with_when(schedule_time).unwrap();
            log(&format!(
                "Playing file {} at time {}",
                index + 1,
                schedule_time
            ));
        }) as Box<dyn FnMut(_)>);

        let onload = closure.as_ref().unchecked_ref();

        match audio_context_rc.try_borrow() {
            Ok(audio_context) => {
                let promise =
                    audio_context.decode_audio_data_with_success_callback(&array_buffer, onload)?;
                let _ = JsFuture::from(promise).await?;
            }
            Err(_err) => {
                log("Error: Could not borrow the AudioContext");
                return Err(JsValue::from_str("Could not borrow the AudioContext"));
            }
        }
        closure.forget();
    }

    Ok(())
}
