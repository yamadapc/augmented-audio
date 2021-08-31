//
//  File.swift
//  
//
//  Created by Pedro Tacla Yamada on 31/8/21.
//

import Foundation
import SwiftUI

public protocol AppContext {
    func chartHandler() throws -> ChartHandler
}

enum EmptyAppContextError: Error {
    case notImplemented
}

class EmptyAppContext: AppContext {
    func chartHandler() throws -> ChartHandler {
        throw EmptyAppContextError.notImplemented
    }
}

public struct AppContextKey: EnvironmentKey {
    public static let defaultValue: AppContext = EmptyAppContext()
}

extension EnvironmentValues {
    public var appContext: AppContext {
        get { self[AppContextKey.self] }
        set { self[AppContextKey.self] = newValue }
    }
}
