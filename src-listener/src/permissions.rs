//! Platform permission prompts for the listener process.

pub fn request_required_permissions() {
    platform::request_required_permissions();
}

#[cfg(target_os = "macos")]
mod platform {
    use core_foundation::base::TCFType;
    use core_foundation::boolean::CFBoolean;
    use core_foundation::dictionary::CFDictionary;
    use core_foundation::string::CFString;
    use core_foundation_sys::base::Boolean;
    use core_foundation_sys::dictionary::CFDictionaryRef;
    use core_foundation_sys::string::CFStringRef;
    use log::{info, warn};
    use std::fs;
    use std::path::PathBuf;

    const INPUT_MONITORING_PROMPT_MARKER: &str = "macos-input-monitoring.prompted";
    const ACCESSIBILITY_PROMPT_MARKER: &str = "macos-accessibility.prompted";

    pub fn request_required_permissions() {
        let input_monitoring_ready = request_input_monitoring_access();
        if input_monitoring_ready {
            info!("macOS Input Monitoring permission is granted");
        } else {
            warn!(
                "macOS Input Monitoring permission is not granted yet. Grant it in System Settings, then restart the listener."
            );
        }

        let accessibility_ready = request_accessibility_access();
        if accessibility_ready {
            info!("macOS Accessibility permission is granted");
        } else {
            warn!(
                "macOS Accessibility permission is not granted yet. Grant it in System Settings, then restart the listener."
            );
        }
    }

    fn request_input_monitoring_access() -> bool {
        if unsafe { CGPreflightListenEventAccess() } {
            return true;
        }

        if has_prompt_marker(INPUT_MONITORING_PROMPT_MARKER) {
            return false;
        }

        let granted = unsafe { CGRequestListenEventAccess() };
        mark_prompt_shown(INPUT_MONITORING_PROMPT_MARKER);
        granted
    }

    fn request_accessibility_access() -> bool {
        if is_accessibility_trusted(false) {
            return true;
        }

        if has_prompt_marker(ACCESSIBILITY_PROMPT_MARKER) {
            return false;
        }

        let granted = is_accessibility_trusted(true);
        mark_prompt_shown(ACCESSIBILITY_PROMPT_MARKER);
        granted
    }

    fn is_accessibility_trusted(prompt: bool) -> bool {
        unsafe {
            let prompt_key = CFString::wrap_under_get_rule(kAXTrustedCheckOptionPrompt);
            let prompt_value = if prompt {
                CFBoolean::true_value()
            } else {
                CFBoolean::false_value()
            };
            let options = CFDictionary::from_CFType_pairs(&[(prompt_key, prompt_value)]);

            AXIsProcessTrustedWithOptions(options.as_concrete_TypeRef()) != 0
        }
    }

    fn has_prompt_marker(name: &str) -> bool {
        prompt_marker_path(name).is_some_and(|path| path.exists())
    }

    fn mark_prompt_shown(name: &str) {
        let Some(path) = prompt_marker_path(name) else {
            warn!("Could not resolve permission prompt marker path");
            return;
        };

        if let Some(parent) = path.parent() {
            if let Err(error) = fs::create_dir_all(parent) {
                warn!(
                    "Failed to create permission prompt marker directory: {}",
                    error
                );
                return;
            }
        }

        if let Err(error) = fs::write(&path, b"prompted\n") {
            warn!("Failed to write permission prompt marker: {}", error);
        }
    }

    fn prompt_marker_path(name: &str) -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("horologion").join("permissions").join(name))
    }

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGPreflightListenEventAccess() -> bool;
        fn CGRequestListenEventAccess() -> bool;
    }

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        static kAXTrustedCheckOptionPrompt: CFStringRef;
        fn AXIsProcessTrustedWithOptions(options: CFDictionaryRef) -> Boolean;
    }
}

#[cfg(not(target_os = "macos"))]
mod platform {
    pub fn request_required_permissions() {}
}
