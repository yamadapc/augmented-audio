import Foundation

/**
 * Protocol to be implemented by audio-engine.
 * These are the methods the UI calls when pushing data/events into the engine.
 */
public protocol SequencerEngine {
    func onClickPlayheadStop()
    func onClickPlayheadPlay()

    func setVolume(track: Int, volume: Float)
    func setTempo(tempo: Float)

    func onClickRecord(track: Int)
    func onClickPlay(track: Int)
    func onClickClear(track: Int)

    func toggleStep(track: Int, step: Int)
    func addParameterLock(track: Int, step: Int, parameterId: ObjectId, value: Float)
    func addSceneParameterLock(sceneId: Int, track: Int, parameterId: ObjectId, value: Float)
    func addMidiMapping(controller: Int, parameterId: ObjectId)
}
