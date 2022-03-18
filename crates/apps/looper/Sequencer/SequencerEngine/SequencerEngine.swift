//
//  SequencerEngine.swift
//  SequencerEngine
//
//  Created by Pedro Tacla Yamada on 13/3/2022.
//

import Combine
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

    func setTempo(tempo: Float) {
        looper_engine__set_tempo(engine, tempo)
    }

    func toggleStep(track: Int, step: Int) {
        looper_engine__toggle_trigger(engine, UInt(track - 1), UInt(step))
        // let voice = looper_engine__get_voice(engine, UInt(step - 1))
    }
}

func getParameterId(_ id: SourceParameterId) -> SequencerEngine_private.SourceParameter {
    switch id {
    case .start:
        return Start
    case .end:
        return End
    case .fadeStart:
        return FadeStart
    case .fadeEnd:
        return FadeEnd
    case .pitch:
        return Pitch
    case .speed:
        return Speed
    }
}

public class EngineController {
    let engine: EngineImpl
    public let store: Store

    var cancellables: Set<AnyCancellable> = Set()

    public init() {
        engine = EngineImpl()
        store = Store(engine: engine)

        store.trackStates.enumerated().forEach { i, trackState in
            trackState.sourceParameters.parameters.forEach { parameter in
                parameter.$value.sink(receiveValue: { value in
                    let rustParameterId = getParameterId(parameter.id)
                    looper_engine__set_source_parameter(self.engine.engine, UInt(i), rustParameterId, value)
                }).store(in: &cancellables)
            }
        }

        store.metronomeVolume.$value.sink(receiveValue: { value in
            looper_engine__set_metronome_volume(self.engine.engine, value)
        }).store(in: &cancellables)

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
                self.store.setTrackBuffer(trackId: 1, fromUnsafePointer: bufferPtr)
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
                if trackState.looperState == .playing {
                    let buffer = looper_engine__get_looper_buffer(engine.engine, UInt(i))
                    let trackBuffer = LooperBufferTrackBuffer(inner: buffer!)
                    // TODO: here we should free the previous buffer if it exists
                    store.setTrackBuffer(trackId: i + 1, fromAbstractBuffer: trackBuffer)
                } else if trackState.looperState == .empty {
                    store.setTrackBuffer(trackId: i + 1, fromAbstractBuffer: nil)
                }
            }
        }

        DispatchQueue.main.asyncAfter(deadline: .now().advanced(by: .milliseconds(16))) {
            self.flushPollInfo()
        }
    }
}

struct LooperBufferTrackBuffer {
    var inner: OpaquePointer
}

extension LooperBufferTrackBuffer: TrackBuffer {
    var count: Int { Int(looper_buffer__num_samples(inner)) }
    subscript(index: Int) -> Float {
        looper_buffer__get(inner, UInt(index))
    }

    func equals(other: TrackBuffer) -> Bool {
        if let otherBuffer = other as? LooperBufferTrackBuffer {
            return inner == otherBuffer.inner
        } else {
            return false
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
