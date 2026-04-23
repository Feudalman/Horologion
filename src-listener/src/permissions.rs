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

        unsafe { CGRequestListenEventAccess() }
    }

    fn request_accessibility_access() -> bool {
        if is_accessibility_trusted(false) {
            return true;
        }

        is_accessibility_trusted(true)
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
