// = copyright ====================================================================
// Continuous: Live-looper and performance sampler
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
import SwiftUI
import WebKit
import WebView

class AboutNavigationDelegate: NSObject, WKNavigationDelegate {
    func webView(_: WKWebView, decidePolicyFor navigationAction: WKNavigationAction) async -> WKNavigationActionPolicy {
        if let url = navigationAction.request.mainDocumentURL,
           !url.isFileURL
        {
            NSWorkspace.shared.open(url)
            return WKNavigationActionPolicy.cancel
        }
        return WKNavigationActionPolicy.allow
    }
}

struct AboutPageView: View {
    @StateObject var webViewStore = WebViewStore()
    var navigationDelegate = AboutNavigationDelegate()

    var body: some View {
        WebView(webView: webViewStore.webView)
            .onAppear {
                self.webViewStore.webView.navigationDelegate = self.navigationDelegate
                if let url: URL = Bundle.main.url(forResource: "license", withExtension: ".html") {
                    self.webViewStore.webView.loadFileURL(url, allowingReadAccessTo: url)
                }
            }
    }
}
