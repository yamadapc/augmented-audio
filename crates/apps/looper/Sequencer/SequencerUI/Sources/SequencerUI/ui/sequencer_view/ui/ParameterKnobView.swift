//
//  File.swift
//
//
//  Created by Pedro Tacla Yamada on 17/3/2022.
//

import SwiftUI

struct ParameterKnobView: View {
    @ObservedObject var parameter: FloatParameter
    var body: some View {
        KnobView(
            label: parameter.label,
            onChanged: { newValue in
                parameter.value = Float(newValue)
            },
            value: Double(parameter.value)
        )
    }
}
