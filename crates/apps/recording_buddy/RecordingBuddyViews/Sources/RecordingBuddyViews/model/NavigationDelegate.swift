//
//  File.swift
//  
//
//  Created by Pedro Tacla Yamada on 1/9/21.
//

import Foundation

public enum NavigationEvent {
    case openSettings
}

public protocol NavigationDelegate {
    func navigate(_ event: NavigationEvent)
}
