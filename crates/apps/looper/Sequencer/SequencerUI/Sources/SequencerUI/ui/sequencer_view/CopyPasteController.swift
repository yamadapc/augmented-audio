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

private let logger = Logger(label: "com.beijaflor.sequencer.CopyPasteController")

/**
 * Currently empty.
 *
 * The idea is to support normal system copy-paste, so we'll push a JSON string representing the copied object onto
 * the clipboard (or a file-path if a file-path is sensible) and on paste we'll parse the string & find the object.
 */
private func onPaste(_: Store, _ maybeValue: String?, _ maybeErr: Error?) {
    if let err = maybeErr {
        logger.warning("Failed to parse clipboard contents as String", metadata: [
            "error": .stringConvertible(err.localizedDescription),
        ])
        return
    }
    guard let value = maybeValue else {
        logger.warning("Empty value pasted")
        return
    }

    logger.info("Received pasteboard event", metadata: [
        "value": .string(value),
    ])
}

extension View {
    func setupCopyPasteController(store: Store) -> some View {
        return onCopyCommand(perform: {
            [
                NSItemProvider(
                    object: NSString(
                        "continuous_looper://parameter/looper:1/source_start"
                    )
                ),
            ]
        })
        .onPasteCommand(of: [
            "public.plain-text",
        ], perform: { items in
            items.forEach { item in
                _ = item.loadObject(
                    ofClass: String.self,
                    completionHandler: { value, err in
                        onPaste(store, value, err)
                    }
                )
            }
        })
    }
}
