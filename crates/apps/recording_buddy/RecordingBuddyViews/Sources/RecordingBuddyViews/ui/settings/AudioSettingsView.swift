//
//  SwiftUIView.swift
//  
//
//  Created by Pedro Tacla Yamada on 1/9/21.
//

import SwiftUI

public typealias HostId = String
public typealias InputId = String
public typealias OutputId = String

public class AvailableAudioOptionsModel: ObservableObject {
    @Published public var hostIds: [HostId]
    @Published public var inputIds: [InputId]
    @Published public var outputIds: [OutputId]

    public init() {
        self.hostIds = []
        self.inputIds = []
        self.outputIds = []
    }

    public init(
        hostIds: [HostId],
        inputIds: [InputId],
        outputIds: [OutputId]
    ) {
        self.hostIds = hostIds
        self.inputIds = inputIds
        self.outputIds = outputIds
    }
}

public class AudioOptionsModel: ObservableObject {
    @Published public var hostId: HostId?
    @Published public var inputId: InputId?

    public init() {}

    public init(
        hostId: HostId,
        inputId: InputId
    ) {
        self.hostId = hostId
        self.inputId = inputId
    }
}

@available(macOS 11.0, *)
struct SelectInput: View {
    @State var selection: Int? = nil
    var label: String
    @Binding var options: [String]
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
public struct AudioSettingsView: View {
    @ObservedObject var model: AudioOptionsModel
    @ObservedObject var audioInfo: AvailableAudioOptionsModel

    public init(
        model: AudioOptionsModel,
        audioInfo: AvailableAudioOptionsModel
    ) {
        self.model = model
        self.audioInfo = audioInfo
    }

    public var body: some View {
        VStack {
            SelectInput(
                label: "Audio Host",
                options: $audioInfo.hostIds,
                model: $model.hostId
            )
            SelectInput(
                label: "Audio Input Device",
                options: $audioInfo.inputIds,
                model: $model.inputId
            )
        }.padding()
    }
}

@available(macOS 11.0, *)
struct AudioSettingsView_Previews: PreviewProvider {
    static var previews: some View {
        AudioSettingsView(
            model: AudioOptionsModel(),
            audioInfo: AvailableAudioOptionsModel()
        )
    }
}
