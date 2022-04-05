// = copyright ====================================================================
// Continuous: Live-looper and performance sampler
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
import Logging
import SwiftUI

/**
 * Listens to drop events on the top-level view. If an audio-file is dropped, calls into the LooperEngine to load it.
 */
class DropController {
    private let store: Store
    private let logger = Logger(label: "com.beijaflor.sequencer.ui.DropController")
    init(store: Store) {
        self.store = store
    }
}

@available(macOS 11.0, *)
extension DropController: DropDelegate {
    func dropEntered(info _: DropInfo) {}

    func dropExited(info _: DropInfo) {}

    func performDrop(info: DropInfo) -> Bool {
        let audioContent = info.itemProviders(for: [.fileURL])
        audioContent.forEach { file in
            let _ = file.loadObject(ofClass: NSURL.self, completionHandler: { item, _ in
                let url: NSURL?? = item as? NSURL?
                let path: String? = url??.filePathURL?.path
                self.logger.info("Received drop event", metadata: [
                    "filepath": .string(path ?? "<unknown>"),
                ])
                if let pathStr = path {
                    self.store.engine?.loadFile(atPath: pathStr)
                }
            })
        }
        return true
    }
}
