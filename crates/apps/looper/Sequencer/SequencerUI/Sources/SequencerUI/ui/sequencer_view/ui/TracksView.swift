//
//  SwiftUIView.swift
//
//
//  Created by Pedro Tacla Yamada on 11/3/2022.
//

import SwiftUI

struct TracksView: View {
    var selectedTrack: Int
    var onClickTrack: (Int) -> Void

    var body: some View {
        HStack {
            ForEach(1 ..< 9) { i in
                Group {
                    TrackButton(
                        action: {
                            onClickTrack(i)
                        },
                        label: "\(i)",
                        isSelected: selectedTrack == i
                    )
                }
            }

            TrackButton(
                action: {
                    print("")
                },
                label: "Master",
                isSelected: false
            ).frame(maxWidth: .infinity)
        }
        .frame(maxWidth: .infinity, alignment: .bottomLeading)
        .padding(PADDING)
    }
}

struct TracksView_Previews: PreviewProvider {
    static var previews: some View {
        TracksView(
            selectedTrack: 1,
            onClickTrack: { _ in }
        )
    }
}
