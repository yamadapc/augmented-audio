//
//  Sequencer_AudioUnitAudioUnit.m
//  Sequencer AudioUnit
//
//  Created by Pedro Tacla Yamada on 23/3/2022.
//

#import "Sequencer_AudioUnitAudioUnit.h"

#import <AVFoundation/AVFoundation.h>

// Define parameter addresses.
const AudioUnitParameterID myParam1 = 0;

@interface Sequencer_AudioUnitAudioUnit ()

@property (nonatomic, readwrite) AUParameterTree *parameterTree;
@property AUAudioUnitBusArray *inputBusArray;
@property AUAudioUnitBusArray *outputBusArray;
@end


@implementation Sequencer_AudioUnitAudioUnit
@synthesize parameterTree = _parameterTree;

- (instancetype)initWithComponentDescription:(AudioComponentDescription)componentDescription options:(AudioComponentInstantiationOptions)options error:(NSError **)outError {
    self = [super initWithComponentDescription:componentDescription options:options error:outError];
    
    if (self == nil) { return nil; }

	_kernelAdapter = [[Sequencer_AudioUnitDSPKernelAdapter alloc] init];

	[self setupAudioBuses];
	[self setupParameterTree];
	[self setupParameterCallbacks];
    return self;
}

#pragma mark - AUAudioUnit Setup

- (void)setupAudioBuses {
	// Create the input and output bus arrays.
	_inputBusArray  = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self
															 busType:AUAudioUnitBusTypeInput
															  busses: @[_kernelAdapter.inputBus]];
	_outputBusArray = [[AUAudioUnitBusArray alloc] initWithAudioUnit:self
															 busType:AUAudioUnitBusTypeOutput
															  busses: @[_kernelAdapter.outputBus]];
}

- (void)setupParameterTree {
    // Create parameter objects.
    AUParameter *param1 = [AUParameterTree createParameterWithIdentifier:@"param1"
																	name:@"Parameter 1"
																 address:myParam1
																	 min:0
																	 max:100
																	unit:kAudioUnitParameterUnit_Percent
																unitName:nil
																   flags:kAudioUnitParameterFlag_IsWritable | kAudioUnitParameterFlag_IsReadable
															valueStrings:nil
													 dependentParameters:nil];

    // Initialize the parameter values.
    param1.value = 0.5;

    // Create the parameter tree.
    _parameterTree = [AUParameterTree createTreeWithChildren:@[ param1 ]];
}

- (void)setupParameterCallbacks {
	// Make a local pointer to the kernel to avoid capturing self.
	__block Sequencer_AudioUnitDSPKernelAdapter * kernelAdapter = _kernelAdapter;

	// implementorValueObserver is called when a parameter changes value.
	_parameterTree.implementorValueObserver = ^(AUParameter *param, AUValue value) {
		[kernelAdapter setParameter:param value:value];
	};

	// implementorValueProvider is called when the value needs to be refreshed.
	_parameterTree.implementorValueProvider = ^(AUParameter *param) {
		return [kernelAdapter valueForParameter:param];
	};

	// A function to provide string representations of parameter values.
	_parameterTree.implementorStringFromValueCallback = ^(AUParameter *param, const AUValue *__nullable valuePtr) {
		AUValue value = valuePtr == nil ? param.value : *valuePtr;

		return [NSString stringWithFormat:@"%.f", value];
	};
}

#pragma mark - AUAudioUnit Overrides

- (AUAudioFrameCount)maximumFramesToRender {
	return [_kernelAdapter maximumFramesToRender];
}

- (void)setMaximumFramesToRender:(AUAudioFrameCount)maximumFramesToRender {
	[_kernelAdapter setMaximumFramesToRender:maximumFramesToRender];
}

// If an audio unit has input, an audio unit's audio input connection points.
// Subclassers must override this property getter and should return the same object every time.
// See sample code.
- (AUAudioUnitBusArray *)inputBusses {
	return _inputBusArray;
}

// An audio unit's audio output connection points.
// Subclassers must override this property getter and should return the same object every time.
// See sample code.
- (AUAudioUnitBusArray *)outputBusses {
	return _outputBusArray;
}

// Allocate resources required to render.
// Subclassers should call the superclass implementation.
- (BOOL)allocateRenderResourcesAndReturnError:(NSError **)outError {
	if (_kernelAdapter.outputBus.format.channelCount != _kernelAdapter.inputBus.format.channelCount) {
		if (outError) {
			*outError = [NSError errorWithDomain:NSOSStatusErrorDomain code:kAudioUnitErr_FailedInitialization userInfo:nil];
		}
		// Notify superclass that initialization was not successful
		self.renderResourcesAllocated = NO;

		return NO;
	}

	[super allocateRenderResourcesAndReturnError:outError];
	[_kernelAdapter allocateRenderResources];
	return YES;
}

// Deallocate resources allocated in allocateRenderResourcesAndReturnError:
// Subclassers should call the superclass implementation.
- (void)deallocateRenderResources {
	[_kernelAdapter deallocateRenderResources];

    // Deallocate your resources.
    [super deallocateRenderResources];
}

#pragma mark - AUAudioUnit (AUAudioUnitImplementation)

// Block which subclassers must provide to implement rendering.
- (AUInternalRenderBlock)internalRenderBlock {
	return _kernelAdapter.internalRenderBlock;
}

@end

