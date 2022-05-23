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

#if os(macOS)
    import Cocoa
    import KeyboardShortcuts

    /**
     * Dispatches keyboard-shortcuts as application actions.
     */
    class KeyboardShortcutsController {
        private let store: Store

        init(store: Store) {
            self.store = store
        }

        func onKeyDown(key: KeyboardShortcuts.Key, modifiers: NSEvent.ModifierFlags) {
            if handleTrackSwitch(key: key, modifiers: modifiers) {
                return
            }

            switch key {
            case .space:
                if store.isPlaying {
                    store.onClickPlayheadStop()
                } else {
                    store.onClickPlayheadPlay()
                }
            case .delete:
                if case let .parameterLock(source: sourceId, parameterId: parameterId) = store.focusState.selectedObject {
                    store.removeParameterLock(ParameterLockId(parameterId: parameterId, source: sourceId))
                }
            default:
                return
            }
        }

        func onKeyUp(key _: KeyboardShortcuts.Key, modifiers _: NSEvent.ModifierFlags) {}

        func handleTrackSwitch(key: KeyboardShortcuts.Key, modifiers: NSEvent.ModifierFlags) -> Bool {
            if !modifiers.contains(.command) {
                return false
            }

            let numberKeys = [
                KeyboardShortcuts.Key.one,
                KeyboardShortcuts.Key.two,
                KeyboardShortcuts.Key.three,
                KeyboardShortcuts.Key.four,
                KeyboardShortcuts.Key.five,
                KeyboardShortcuts.Key.six,
                KeyboardShortcuts.Key.seven,
                KeyboardShortcuts.Key.eight,
            ]
            var trackKeys: [KeyboardShortcuts.Key: () -> Void] = [:]
            store.trackStates.enumerated().forEach { i, track in
                let key = numberKeys[i]
                let trackId = track.id
                trackKeys[key] = {
                    self.store.onClickTrack(trackId)
                }
            }

            guard let action = trackKeys[key] else {
                return false
            }

            action()
            return true
        }
    }

#endif
