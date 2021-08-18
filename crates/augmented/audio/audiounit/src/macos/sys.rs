#[cfg(test)]
mod test {
    use cocoa::base::YES;

    use avfaudio_sys::{
        AVAudioUnitComponent, AVAudioUnitComponentManager, IAVAudioUnitComponent,
        IAVAudioUnitComponentManager, INSArray, INSPredicate, NSPredicate,
    };

    #[test]
    fn list_all_audio_units() {
        unsafe {
            let manager: AVAudioUnitComponentManager = AVAudioUnitComponentManager(
                AVAudioUnitComponentManager::sharedAudioUnitComponentManager(),
            );
            let components =
                manager.componentsMatchingPredicate_(NSPredicate::predicateWithValue_(YES));
            for i in 0..INSArray::<AVAudioUnitComponent>::count(components) {
                let component: AVAudioUnitComponent = components.objectAtIndex_(i);
            }
        }
    }
}
