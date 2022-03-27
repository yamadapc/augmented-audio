import SwiftUI
import Logging

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
    func performDrop(info: DropInfo) -> Bool {
        let audioContent = info.itemProviders(for: [.fileURL])
        audioContent.forEach { file in
          let _ = file.loadObject(ofClass: NSURL.self, completionHandler: { item, err in
            let url: NSURL?? = item as? NSURL?
            let path: String? = url??.filePathURL?.path
            self.logger.info("Received drop event", metadata: [
              "filepath": .string(path ?? "<unknown>")
            ])
          })
        }
        return true
    }
}

