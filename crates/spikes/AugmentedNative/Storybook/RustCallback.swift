//
//  RustCallbac.swift
//  Storybook
//
//  Created by Pedro Tacla Yamada on 24/8/21.
//

import Foundation

private class WrapClosure<T> {
    fileprivate let closure: T
    init(closure: T) {
        self.closure = closure
    }
}
public func asyncOperation(closure: @escaping (Bool) -> Void) {
    let wrappedClosure = WrapClosure(closure: closure)
    let userdata = Unmanaged.passRetained(wrappedClosure).toOpaque()

    let callback: @convention(c) (UnsafeMutableRawPointer?, Bool) -> Void = { (_ userdata: UnsafeMutableRawPointer?, _ success: Bool) in
        if let userdata = userdata {
            let wrappedClosure: WrapClosure<(Bool) -> Void> = Unmanaged.fromOpaque(userdata).takeRetainedValue()
            wrappedClosure.closure(success)
        }
    }

    let completion = CompletedCallback(userdata: userdata, callback: callback)

    async_operation(completion)
}
