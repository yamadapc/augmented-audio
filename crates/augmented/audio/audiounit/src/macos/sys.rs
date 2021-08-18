// #[cfg(test)]
// mod test {
//     use cocoa::base::YES;
//
//     use avfaudio_sys::{
//         AUAudioUnit, AVAudioUnitComponent, AVAudioUnitComponentManager,
//         AudioComponentInstantiationOptions_kAudioComponentInstantiation_LoadInProcess,
//         IAUAudioUnit, IAVAudioUnitComponent, IAVAudioUnitComponentManager, INSArray, INSPredicate,
//         INSString, NSPredicate, NSString,
//     };
//     use block::ConcreteBlock;
//
//     #[test]
//     fn list_all_audio_units() {
//         unsafe {
//             let manager: AVAudioUnitComponentManager = AVAudioUnitComponentManager(
//                 AVAudioUnitComponentManager::sharedAudioUnitComponentManager(),
//             );
//             let components =
//                 manager.componentsMatchingPredicate_(NSPredicate::predicateWithValue_(YES));
//             for i in 0..INSArray::<AVAudioUnitComponent>::count(&components) {
//                 let component: AVAudioUnitComponent = AVAudioUnitComponent(INSArray::<
//                     AVAudioUnitComponent,
//                 >::objectAtIndex_(
//                     &components, i
//                 ));
//
//                 let (tx, rx) = std::sync::mpsc::channel();
//                 let block = ConcreteBlock::new(move |audio_unit, error| {
//                     println!("\\o/");
//                     tx.send((audio_unit, error)).unwrap();
//                 });
//                 let block = block.copy();
//
//                 AUAudioUnit::instantiateWithComponentDescription_options_completionHandler_(
//                     component.audioComponentDescription(),
//                     AudioComponentInstantiationOptions_kAudioComponentInstantiation_LoadInProcess,
//                     &*block,
//                 );
//
//                 let (au_audio_unit, _error) = rx.recv().unwrap();
//                 println!("Loaded Audio-Unit:");
//                 println!("  Name: {}", to_string(au_audio_unit.name()));
//                 println!("  Version: {}", to_string(au_audio_unit.version()));
//
//                 break;
//             }
//         }
//     }
//
//     fn to_string(ns_string: NSString) -> String {
//         let ns_string = cacao::foundation::NSString::retain(*ns_string);
//         ns_string.to_string()
//     }
// }
