//
//  SequencerEngine.swift
//  SequencerEngine
//
//  Created by Pedro Tacla Yamada on 13/3/2022.
//

import Foundation
import SequencerEngine_private
import SequencerUI

class EngineImpl {
    var engine: OpaquePointer!

    init() {
        engine = looper_engine__new()
    }
}

extension EngineImpl: SequencerEngine {
    func setVolume(track: Int, volume: Float) {
        looper_engine__set_volume(engine, UInt(track - 1), volume)
    }

    func onClickRecord(track: Int) {
        looper_engine__record(engine, UInt(track - 1))
    }

    func onClickPlay(track: Int) {
        looper_engine__play(engine, UInt(track - 1))
    }

    func onClickClear(track: Int) {
        looper_engine__clear(engine, UInt(track - 1))
    }

    func onClickPlayheadStop() {
        looper_engine__playhead_stop(engine)
    }

    func onClickPlayheadPlay() {
        looper_engine__playhead_play(engine)
    }
}

public class EngineController {
    let engine: EngineImpl
    public let store: Store

    public init() {
        engine = EngineImpl()
        store = Store(engine: engine)

        DispatchQueue.main.async {
            self.flushPollInfo()
        }
    }

    public func loadExampleFileBuffer() {
        DispatchQueue.global(qos: .background).async {
            let exampleBuffer = looper__get_example_buffer()
            let bufferPtr = UnsafeBufferPointer<Float32>(
                start: exampleBuffer.ptr,
                count: Int(exampleBuffer.count)
            )
            DispatchQueue.main.async {
                self.store.setTrackBuffer(trackId: 1, buffer: bufferPtr)
            }
        }
    }

    func flushPollInfo() {
        let playhead = looper_engine__get_playhead_position(engine.engine)

        // Updating ObservableObject at 60fps causes high CPU usage
        if store.timeInfo.positionSamples != playhead.position_samples {
            store.timeInfo.positionSamples = playhead.position_samples
        }
        let positionBeats = playhead.position_beats == -1 ? nil : playhead.position_beats
        if store.timeInfo.positionBeats != positionBeats {
            store.timeInfo.positionBeats = positionBeats
        }
        let tempo = playhead.tempo == -1 ? nil : playhead.tempo
        if store.timeInfo.tempo != tempo {
            store.timeInfo.tempo = tempo
        }

        for (i, trackState) in store.trackStates.enumerated() {
            // trackState.numSamples = looper_engine__get_looper_num_samples(engine.engine, UInt(i))
            let positionPercent = looper_engine__get_looper_position(engine.engine, UInt(i))
            if trackState.positionPercent != positionPercent {
                trackState.positionPercent = positionPercent
            }
            let looperState = convertState(looperState: looper_engine__get_looper_state(engine.engine, UInt(i)))
            if trackState.looperState != looperState {
                trackState.looperState = looperState
            }
        }

        DispatchQueue.main.asyncAfter(deadline: .now().advanced(by: .milliseconds(16))) {
            self.flushPollInfo()
        }
    }
}

func convertState(looperState: SequencerEngine_private.LooperState) -> SequencerUI.LooperState {
    switch looperState {
    case SequencerEngine_private.Recording:
        return SequencerUI.LooperState.recording
    case SequencerEngine_private.Playing:
        return SequencerUI.LooperState.playing
    case SequencerEngine_private.Paused:
        return SequencerUI.LooperState.paused
    case SequencerEngine_private.Overdubbing:
        return SequencerUI.LooperState.overdubbing
    case SequencerEngine_private.RecordingScheduled:
        return SequencerUI.LooperState.recordingScheduled
    case SequencerEngine_private.PlayingScheduled:
        return SequencerUI.LooperState.playingScheduled
    default:
        return SequencerUI.LooperState.empty
    }
}
