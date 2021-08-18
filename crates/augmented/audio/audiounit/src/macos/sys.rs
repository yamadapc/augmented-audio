#[cfg(test)]
mod test {
    use cocoa::base::YES;

    use avfaudio_sys::{
        AVAudioUnitComponentManager, IAVAudioUnitComponentManager, INSPredicate, NSPredicate,
    };

    #[test]
    fn list_all_audio_units() {
        let manager = AVAudioUnitComponentManager::sharedAudioUnitComponentManager();
        let components =
            manager.componentsMatchingPredicate_(NSPredicate::predicateWithValue_(YES));
    }
}
