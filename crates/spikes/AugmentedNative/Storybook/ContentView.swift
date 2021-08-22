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

class AudioGuiModel: ObservableObject {
    @Published var hostIds: [HostId]
    @Published var inputIds: [InputId]
    @Published var outputIds: [OutputId]
    @Published var settings: AudioSettingsModel

    init(
        hostIds: [HostId],
        inputIds: [InputId],
        outputIds: [OutputId],
        settings: AudioSettingsModel
    ) {
        self.hostIds = hostIds
        self.inputIds = inputIds
        self.outputIds = outputIds
        self.settings = settings
    }
}

struct SelectInput: View {
    @State var selection = 0
    var label: String
    var options: [String]

    var body: some View {
        HStack {
            Text(label).frame(width: 200, alignment: .trailing)
            Picker(label, selection: $selection, content: {
                ForEach(options, id: \.self) { value in
                    Text(value)
                }
            }).labelsHidden()
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
    var model: AudioGuiInitialModel!
    @State var selection = 0

    init() {
        self.model = controller.onInit()
        print("INITIALIZED \(String(describing: self.model))")
    }

    var body: some View {
        VStack {
            Section(header: Text("Audio Settings")) {
                SelectInput(label: "Audio Host", options: self.model.hostIds)
                SelectInput(label: "Audio Input Device", options: self.model.inputIds)
                SelectInput(label: "Audio Output Device", options: self.model.outputIds)
            }.font(.system(.body).bold())
        }.padding()
    }
}


@available(macOS 11.0, *)
struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
