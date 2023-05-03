use js_sys::{Array, JsString};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioContext, Response, Window};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

async fn fetch_and_decode(audio_context: &AudioContext, url: &str) -> Result<AudioBuffer, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let response: Response = JsFuture::from(window.fetch_with_str(url))
        .await?
        .dyn_into()?;
    let array_buffer: js_sys::ArrayBuffer =
        JsFuture::from(response.array_buffer()?).await?.dyn_into()?;
    let audio_buffer: AudioBuffer = JsFuture::from(audio_context.decode_audio_data(&array_buffer)?)
        .await?
        .dyn_into()?;
    Ok(audio_buffer)
}

#[wasm_bindgen]
pub async fn play_ogg_files(audio_file_urls: JsValue) -> Result<(), JsValue> {
    let audio_file_urls: Array = audio_file_urls.into();
    log(&format!("Play {} files", audio_file_urls.length()));

    let audio_context = AudioContext::new()?;
    let audio_context_rc = Rc::new(RefCell::new(audio_context));
    let base_time = audio_context_rc.borrow().current_time() + 1.0;

    for (index, url) in audio_file_urls.iter().enumerate() {
        let url = url.dyn_into::<JsString>()?.as_string().unwrap();
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

        let buffer = fetch_and_decode(&audio_context_rc.borrow(), &url).await?;
        let func: &js_sys::Function = closure.as_ref().unchecked_ref();
        func.call1(&JsValue::NULL, &buffer)?;
        closure.forget();
    }

    Ok(())
}
