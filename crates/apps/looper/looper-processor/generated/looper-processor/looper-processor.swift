
public class LooperEngine: LooperEngineRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$LooperEngine$_free(ptr)
        }
    }
}
extension LooperEngine {
    public convenience init() {
        self.init(ptr: __swift_bridge__$LooperEngine$new())
    }
}
public class LooperEngineRefMut: LooperEngineRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class LooperEngineRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension LooperEngineRef {
    public func handle() -> MultiTrackLooperHandleRef {
        MultiTrackLooperHandleRef(ptr: __swift_bridge__$LooperEngine$handle(ptr))
    }
}
extension LooperEngine: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_LooperEngine$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_LooperEngine$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: LooperEngine) {
        __swift_bridge__$Vec_LooperEngine$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_LooperEngine$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (LooperEngine(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LooperEngineRef> {
        let pointer = __swift_bridge__$Vec_LooperEngine$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LooperEngineRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LooperEngineRefMut> {
        let pointer = __swift_bridge__$Vec_LooperEngine$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return LooperEngineRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_LooperEngine$len(vecPtr)
    }
}


public class MultiTrackLooperHandleRef: MultiTrackLooperHandleRefRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$MultiTrackLooperHandleRef$_free(ptr)
        }
    }
}
public class MultiTrackLooperHandleRefRefMut: MultiTrackLooperHandleRefRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class MultiTrackLooperHandleRefRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension MultiTrackLooperHandleRef: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_MultiTrackLooperHandleRef$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_MultiTrackLooperHandleRef$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: MultiTrackLooperHandleRef) {
        __swift_bridge__$Vec_MultiTrackLooperHandleRef$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_MultiTrackLooperHandleRef$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (MultiTrackLooperHandleRef(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<MultiTrackLooperHandleRefRef> {
        let pointer = __swift_bridge__$Vec_MultiTrackLooperHandleRef$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return MultiTrackLooperHandleRefRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<MultiTrackLooperHandleRefRefMut> {
        let pointer = __swift_bridge__$Vec_MultiTrackLooperHandleRef$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return MultiTrackLooperHandleRefRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_MultiTrackLooperHandleRef$len(vecPtr)
    }
}



