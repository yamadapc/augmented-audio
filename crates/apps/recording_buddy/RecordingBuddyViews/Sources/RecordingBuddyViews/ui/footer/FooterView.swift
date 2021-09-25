//
//  SwiftUIView.swift
//  
//
//  Created by Pedro Tacla Yamada on 1/9/21.
//

import SwiftUI

struct FooterView: View {
    @Environment(\.appContext) var appContext: AppContext

    var body: some View {
        let actionImage = NSImage(
            named: NSImage.actionTemplateName
        )!
        actionImage.isTemplate = true

        return HStack {
            Button(action: {
                try? appContext.navigationDelegate().navigate(.openSettings)
            }) {
                Image(nsImage: actionImage)
                Text("Settings")
            }
            .buttonStyle(PlainButtonStyle())
            .frame(alignment: .center)
            .padding(5)
            .background(Color(white: 0, opacity: 0))
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(10.0)
    }
}

struct FooterView_Previews: PreviewProvider {
    static var previews: some View {
        FooterView()
            .environment(\.appContext, EmptyAppContext())
            .frame(maxWidth: 300, alignment: .topLeading)
    }
}
