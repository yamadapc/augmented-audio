mod sys;

use cacao::core_foundation::base::OSStatus;
use cacao::foundation::NSNumber;
use cocoa::base::{id, BOOL, YES};
use objc::msg_send;

// /// An audio component. - <https://developer.apple.com/documentation/audiotoolbox/audiocomponent?language=objc>
// #[repr(C)]
// pub struct AudioComponent {
//     private: [u8; 0],
// }
//
// /// A component instance, or object, is an audio unit or audio codec. - <https://developer.apple.com/documentation/audiotoolbox/audiocomponentinstance?language=objc>
// #[repr(C)]
// pub struct AudioComponentInstance {
//     private: [u8; 0],
// }
//
// #[link(name = "AVFAudio", kind = "framework")]
// extern "C" {
//     fn AudioComponentInstanceNew(
//         in_component: *mut AudioComponent,
//         out_instance: *mut AudioComponentInstance,
//     ) -> OSStatus;
// }

pub struct AVAudioUnit {
    reference: id,
}

impl AVAudioUnit {
    pub fn instantiate(description: AudioComponentDescription) {}
}

/// Wraps `AVAudioUnitComponent` - <https://developer.apple.com/documentation/avfaudio/avaudiounitcomponent?language=objc>
///
/// A class that provides details about an audio unit such as: type, subtype, manufacturer, and
/// location.
pub struct AVAudioUnitComponent {
    reference: id,
}

impl AVAudioUnitComponent {
    pub fn new(reference: id) -> Self {
        Self { reference }
    }

    // /// The AudioComponent of the audio unit component.
    // pub fn audio_component(&self) -> *mut AudioComponent {
    //     unsafe { msg_send![self.reference, audioComponent] }
    // }

    /// The [`AudioComponentDescription`] of the audio unit component.
    pub fn audio_component_description(&self) -> AudioComponentDescription {
        unsafe { msg_send![self.reference, audioComponentDescription] }
    }

    /// The name of the audio unit component.
    pub fn name(&self) -> String {
        unsafe {
            // NSString*
            let ns_string: id = msg_send![self.reference, name];
            AVAudioUnitComponent::build_string(ns_string)
        }
    }

    /// The name of the manufacturer of the audio unit component.
    pub fn manufacturer_name(&self) -> String {
        unsafe {
            // NSString*
            let ns_string: id = msg_send![self.reference, manufacturerName];
            AVAudioUnitComponent::build_string(ns_string)
        }
    }

    /// The audio unit component type.
    pub fn component_type_name(&self) -> String {
        unsafe {
            // NSString*
            let ns_string: id = msg_send![self.reference, typeName];
            AVAudioUnitComponent::build_string(ns_string)
        }
    }

    /// A string representing the audio unit component version number
    pub fn version_string(&self) -> String {
        unsafe {
            // NSString*
            let ns_string: id = msg_send![self.reference, versionString];
            AVAudioUnitComponent::build_string(ns_string)
        }
    }

    fn build_string(ns_string: id) -> String {
        let ns_string = cacao::foundation::NSString::retain(ns_string);
        ns_string.to_string()
    }

    /// An array of supported architectures.
    pub fn available_architectures(&self) -> Vec<i64> {
        let mut result = vec![];
        unsafe {
            // NSArray*
            let ns_array: id = msg_send![self.reference, availableArchitectures];
            let count = msg_send![ns_array, count];

            for i in 0..count {
                // NSNumber*
                let item: id = msg_send![ns_array, objectAtIndex: i];
                result.push(NSNumber::wrap(item).as_i64())
            }
        }
        result
    }

    /// Whether the audio unit component has a custom view.
    pub fn has_custom_view(&self) -> bool {
        unsafe {
            let result: BOOL = msg_send![self.reference, hasCustomView];
            result == YES
        }
    }

    /// Whether the audio unit component has midi input.
    pub fn has_midi_input(&self) -> bool {
        unsafe {
            let result: BOOL = msg_send![self.reference, hasMIDIInput];
            result == YES
        }
    }

    /// Whether the audio unit component has midi output.
    pub fn has_midi_output(&self) -> bool {
        unsafe {
            let result: BOOL = msg_send![self.reference, hasMIDIOutput];
            result == YES
        }
    }
}

/// Wraps `AudioComponentDescription` - <https://developer.apple.com/documentation/audiotoolbox/audiocomponentdescription?language=objc>
///
/// Identifying information for an audio component.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct AudioComponentDescription {
    /// A unique 4-byte code identifying the interface for the component.
    pub componentType: u32,
    /// A 4-byte code that you can use to indicate the purpose of a component. For example, you
    /// could use lpas or lowp as a mnemonic indication that an audio unit is a low-pass filter.
    pub componentSubType: u32,
    /// The unique vendor identifier, registered with Apple, for the audio component.
    pub componentManufacturer: u32,
    /// Set this value to zero.
    pub componentFlags: u32,
    /// Set this value to zero.
    pub componentFlagsMask: u32,
}

/// Wraps `AVAudioUnitComponentManager` - <https://developer.apple.com/documentation/avfaudio/avaudiounitcomponentmanager?language=objc>
///
/// An object that provides a way to search and query audio components that are registered with the
/// system.
///
/// ## Overview
/// > The component manager has methods to find various information about the audio components without
/// > opening them. Currently, only audio components that are audio units can be searched.
/// >
/// > The class also supports predefined system tags and arbitrary user tags. Each audio unit can be
/// > tagged as part of its definition. AudioUnit Hosts such as Logic or GarageBand can present
/// > groupings of audio units based on the tags.
///
/// See the link above for more information.
pub struct AVAudioUnitComponentManager {
    reference: id,
}

impl AVAudioUnitComponentManager {
    /// Returns the shared component manager.
    pub fn shared() -> AVAudioUnitComponentManager {
        AVAudioUnitComponentManager {
            reference: unsafe {
                msg_send![
                    class!(AVAudioUnitComponentManager),
                    sharedAudioUnitComponentManager
                ]
            },
        }
    }

    /// Return all [`AudioUnit`] component information
    pub fn all_components(&mut self) -> Vec<AVAudioUnitComponent> {
        let mut result = vec![];
        unsafe {
            // NSPredicate
            let predicate: id = msg_send![class!(NSPredicate), predicateWithValue: YES];

            // NSArray
            let components_array: id =
                msg_send![self.reference, componentsMatchingPredicate: predicate];
            let count = msg_send![components_array, count];

            for i in 0..count {
                let item: id = msg_send![components_array, objectAtIndex: i];
                let unit = AVAudioUnitComponent::new(item);
                result.push(unit);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_all_components() {
        let mut manager = AVAudioUnitComponentManager::shared();
        let components = manager.all_components();
        for component in components {
            println!("Name: {}", component.name());
        }
    }

    #[test]
    fn test_list_all_components_and_archs() {
        let mut manager = AVAudioUnitComponentManager::shared();
        let components = manager.all_components();
        let archs = components[0].available_architectures();
        println!("{:?}", archs);
    }

    #[test]
    fn test_name_and_desc_strings() {
        let mut manager = AVAudioUnitComponentManager::shared();
        let components = manager.all_components();
        let component = &components[0];
        println!("Name: {:?}", component.name());
        println!("Manufacturer: {:?}", component.manufacturer_name());
        println!("Version: {:?}", component.version_string());
        println!("Type: {:?}", component.component_type_name());
    }

    #[test]
    fn test_query_description() {
        let mut manager = AVAudioUnitComponentManager::shared();
        let components = manager.all_components();
        let component = &components[0];
        println!("Description: {:?}", component.audio_component_description());
    }

    #[test]
    fn test_query_capabilities() {
        let mut manager = AVAudioUnitComponentManager::shared();
        let components = manager.all_components();
        let component = &components[0];
        println!("{}", component.has_custom_view());
        println!("{}", component.has_midi_input());
        println!("{}", component.has_midi_output());
    }
}
