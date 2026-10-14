//! Phosphor Light icons used on the recording setup bar (compile-time subset).

egui_phosphor::subset! {
    pub mod icons {
        use light::{
            APP_WINDOW,
            CARET_DOWN,
            DEVICE_MOBILE,
            GEAR_SIX,
            MICROPHONE,
            MICROPHONE_SLASH,
            MONITOR,
            SELECTION,
            SPEAKER_HIGH,
            SPEAKER_SLASH,
            VIDEO_CAMERA,
            VIDEO_CAMERA_SLASH,
            X,
        };
    }
}

pub fn install(fonts: &mut egui::FontDefinitions) {
    icons::light::add_to_fonts(fonts);
}
