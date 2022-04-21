import SwiftUI
import WebView
import WebKit

class AboutNavigationDelegate: NSObject, WKNavigationDelegate {
  func webView(_ webView: WKWebView, decidePolicyFor navigationAction: WKNavigationAction) async -> WKNavigationActionPolicy {
    if let url = navigationAction.request.mainDocumentURL,
       !url.isFileURL {
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
