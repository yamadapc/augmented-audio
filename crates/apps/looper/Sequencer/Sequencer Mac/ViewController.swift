//
//  ViewController.swift
//  Sequencer Mac
//
//  Created by Pedro Tacla Yamada on 9/3/2022.
//

import Cocoa
import SequencerUI
import SwiftUI

class EngineImpl {
    var engine: OpaquePointer

    init() {
        engine = looper_engine__new()
    }
}

extension EngineImpl: SequencerEngine {
    func onClickRecord(track: Int) {
        looper_engine__record(engine, UInt(track - 1))
    }

    func onClickPlay(track: Int) {
        looper_engine__play(engine, UInt(track - 1))
    }

    func onClickClear(track: Int) {
        looper_engine__clear(engine, UInt(track - 1))
    }
}

class ViewController: NSViewController {
    var engine = EngineImpl()

    override func viewDidLoad() {
        super.viewDidLoad()

        let contentView = ContentView()
            .environmentObject(Store(engine: engine))
        let hostingView = NSHostingView(rootView: contentView)
        view = hostingView
    }
}
