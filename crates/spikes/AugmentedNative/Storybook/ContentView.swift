//
//  ContentView.swift
//  Storybook
//
//  Created by Pedro Tacla Yamada on 22/8/21.
//

import SwiftUI

typealias HostId = String
typealias InputId = String
typealias OutputId = String

class AudioSettingsModel: ObservableObject {
    @Published var hostId: HostId
    @Published var inputId: InputId
    @Published var outputId: OutputId

    init(
        hostId: HostId,
        inputId: InputId,
        outputId: OutputId
    ) {
        self.hostId = hostId
        self.inputId = inputId
        self.outputId = outputId
    }
}

struct SelectInput: View {
    @State var selection: Int? = nil
    var label: String
    var options: [String]

    var body: some View {
        HStack {
            Text("\(label):").frame(width: 200, alignment: .trailing)
            Picker(label, selection: $selection, content: {
                ForEach(options.indices, id: \.self) { index in
                    Text(options[index]).tag(index as Int?)
                }
            }).labelsHidden().frame(maxWidth: 300)
        }
    }
}

class ContentViewController {
    func onInit() -> AudioGuiInitialModel {
        return getAudioInfo()
    }
}

@available(macOS 11.0, *)
struct ContentView: View {
    let controller = ContentViewController()
    var audioInfo: AudioGuiInitialModel!
    @State var selection = 0

    init() {
        self.audioInfo = controller.onInit()
    }

    var body: some View {
        VStack {
            SelectInput(label: "Audio Host", options: self.audioInfo.hostIds)
            SelectInput(label: "Audio Input Device", options: self.audioInfo.inputIds)
            SelectInput(label: "Audio Output Device", options: self.audioInfo.outputIds)
        }.padding()
    }
}


@available(macOS 11.0, *)
struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
