//
//  ContentView.swift
//  Storybook
//
//  Created by Pedro Tacla Yamada on 22/8/21.
//

import SwiftUI
import Combine

typealias HostId = String
typealias InputId = String
typealias OutputId = String

class AudioSettingsModel: ObservableObject {
    @Published var hostId: HostId?
    @Published var inputId: InputId?
    @Published var outputId: OutputId?

    init() {}

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

class ContentViewController {
    private var subscriptions = Set<AnyCancellable>()

    func onInit() -> AudioGuiInitialModel {
        return getAudioInfo()
    }

    func listenTo(model: AudioSettingsModel) {
        model.objectWillChange
            .receive(on: DispatchQueue.main)
            .sink(receiveValue: { [weak model] _ in
                print(String(describing: model))
            })
            .store(in: &subscriptions)
    }
}

@available(macOS 11.0, *)
struct SelectInput: View {
    @State var selection: Int? = nil
    var label: String
    var options: [String]
    @Binding var model: String?

    var body: some View {
        HStack {
            Text("\(label):").frame(width: 200, alignment: .trailing)

            Picker(label, selection: $selection, content: {
                ForEach(options.indices, id: \.self) { index in
                    Text(options[index]).tag(index as Int?)
                }
            })
            .onChange(of: selection, perform: { value in
                if let value = value {
                    model = options[value]
                }
            })
            .labelsHidden()
            .frame(maxWidth: 300)
        }
    }
}


@available(macOS 11.0, *)
struct ContentView: View {
    let controller = ContentViewController()
    var audioInfo: AudioGuiInitialModel!
    @ObservedObject var model = AudioSettingsModel()
    @State var selection = 0

    init() {
        self.audioInfo = controller.onInit()
        controller.listenTo(model: model)
    }

    var body: some View {
        VStack {
            SelectInput(
                label: "Audio Host",
                options: self.audioInfo.hostIds,
                model: $model.hostId
            )
            SelectInput(
                label: "Audio Input Device",
                options: self.audioInfo.inputIds,
                model: $model.inputId
            )
            SelectInput(
                label: "Audio Output Device",
                options: self.audioInfo.outputIds,
                model: $model.outputId
            )
        }.padding()
    }
}


@available(macOS 11.0, *)
struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
