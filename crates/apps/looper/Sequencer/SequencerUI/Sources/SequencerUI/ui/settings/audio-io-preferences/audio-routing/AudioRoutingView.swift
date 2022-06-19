import SwiftUI

struct AudioInputView: View {
    var body: some View {
        Text("Audio input X")
    }
}

struct AudioRoutingView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        VStack {
            AudioInputView()
        }
    }
}
