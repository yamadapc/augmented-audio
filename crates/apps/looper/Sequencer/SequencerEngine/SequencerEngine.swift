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
    func onClickRecord(track: Int) {
        print(engine.debugDescription)
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
            self.flushTimeInfo()
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

    func flushTimeInfo() {
        let playhead = looper_engine__get_playhead_position(engine.engine)

        store.timeInfo.positionSamples = playhead.position_samples
        store.timeInfo.positionBeats = playhead.position_beats == -1 ? nil : playhead.position_beats
        store.timeInfo.tempo = playhead.tempo == -1 ? nil : playhead.tempo

        for (i, trackState) in store.trackStates.enumerated() {
            trackState.numSamples = looper_engine__get_looper_num_samples(engine.engine, UInt(i))
            trackState.positionPercent = looper_engine__get_looper_position(engine.engine, UInt(i))
        }

        DispatchQueue.main.asyncAfter(deadline: .now().advanced(by: .milliseconds(16))) {
            self.flushTimeInfo()
        }
    }
}
