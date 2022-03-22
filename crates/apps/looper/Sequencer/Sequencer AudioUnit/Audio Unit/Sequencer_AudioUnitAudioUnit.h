//
//  Sequencer_AudioUnitAudioUnit.h
//  Sequencer AudioUnit
//
//  Created by Pedro Tacla Yamada on 23/3/2022.
//

#import <AudioToolbox/AudioToolbox.h>
#import "Sequencer_AudioUnitDSPKernelAdapter.h"

// Define parameter addresses.
extern const AudioUnitParameterID myParam1;

@interface Sequencer_AudioUnitAudioUnit : AUAudioUnit

@property (nonatomic, readonly) Sequencer_AudioUnitDSPKernelAdapter *kernelAdapter;
- (void)setupAudioBuses;
- (void)setupParameterTree;
- (void)setupParameterCallbacks;
@end
