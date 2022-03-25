import Combine
import CoreGraphics

struct SceneDragState {
    let scene: Int
    let position: CGPoint
}

enum DragMode {
    case lock, copy
}

/**
 * All UI elements should be identifiable.
 *
 * This tracks what element is hovered, selected or dragged to support operations such as parameter locking and copying.
 */
class FocusState: ObservableObject {
    @Published var mouseOverObject: ObjectId?
    @Published var selectedObject: ObjectId?
    @Published var draggingSource: ParameterLockSource?
    @Published var dragMode: DragMode? = nil

    @Published var sceneDragState: SceneDragState?

    init() {}
}
