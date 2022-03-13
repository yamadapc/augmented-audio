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
        DispatchQueue.global(qos: .background).async {
            print(self.engine.debugDescription)
            self.engine = looper_engine__new()
        }
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
}

public class EngineController {
    let engine: EngineImpl
    public let store: Store

    public init() {
        engine = EngineImpl()
        store = Store(engine: engine)
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
}
