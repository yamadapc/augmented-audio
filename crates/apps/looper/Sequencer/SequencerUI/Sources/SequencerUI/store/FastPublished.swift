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
import Combine
import SwiftUI

protocol IFastPublished {
    var objectWillChange: ObservableObjectPublisher? { get set }
}

/**
 * This is same as `@Published`, but reading values is faster due to skipping some getters.
 *
 * To use this, annotate fields with `@FastPublished` then call `setupFastPublished` in the initializer.
 */
@propertyWrapper public class FastPublished<T>: IFastPublished {
    public var wrappedValue: T {
        didSet {
            projectedValue.send(wrappedValue)
            objectWillChange?.send()
        }
    }

    public var projectedValue: Publisher
    public var objectWillChange: ObservableObjectPublisher?

    public init(wrappedValue: T) {
        self.wrappedValue = wrappedValue
        projectedValue = CurrentValueSubject<T, Never>(wrappedValue)
    }

    public typealias Publisher = CurrentValueSubject<T, Never>
}

/**
 * Binds all `FastPublished` fields to the parent observable object. This is a recursive function to support
 * sub-classing.
 */
func setupFastPublished<O: ObservableObject>(_ target: O) {
    let mirror = Mirror(reflecting: target)
    setupFastPublishedMirror(target, mirror)
}

func setupFastPublishedMirror<O: ObservableObject>(_ target: O, _ mirror: Mirror) {
    mirror.children.forEach { child in
        if var observedProperty = child.value as? IFastPublished {
            observedProperty.objectWillChange = (target.objectWillChange as! ObservableObjectPublisher)
        }
    }

    if let parent = mirror.superclassMirror {
        setupFastPublishedMirror(target, parent)
    }
}
