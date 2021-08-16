use block::Block;
use cocoa::base::{id, BOOL};
use cocoa::foundation::NSRect;
use core_graphics::base::CGFloat;
use core_graphics::geometry::CGPoint;
use libc::c_double;

#[link(name = "WebKit", kind = "framework")]
extern "C" {
    pub static WKWebView: id;
}

pub trait WKWebView: Sized {
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(WKWebView), alloc]
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Determining Whether WebKit can Load Content
    unsafe fn handlesURLScheme_(_: Self, urlScheme: id) -> BOOL;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Initializing a Web View
    unsafe fn configuration(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initWithFrame_configuration_(self, frameRect: NSRect, configuration: id) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initWithCoder_(self, coder: id) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Inspecting the View Information
    unsafe fn scrollView(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn title(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URL(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn customUserAgent(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn setCustomUserAgent_(self, customUserAgent: id);

    // Maybe there's setCustomerUserAgent too ???
    // TODO
    // unsafe fn serverTrust(self) -> SecTrustRef;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Setting Delegates
    unsafe fn navigationDelegate(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Setting Delegates
    unsafe fn setNavigationDelegate_(self, navigationDelegate: id);

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Setting Delegates
    unsafe fn UIDelegate(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Setting Delegates
    unsafe fn setUIDelegate_(self, navigationDelegate: id);

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn estimatedProgress(self) -> c_double;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn hasOnlySecureContent(self) -> BOOL;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn loadHTMLString_baseURL_(self, string: id, baseURL: id) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn loading(self) -> BOOL;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn reload_(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn reload_sender_(self, sender: id) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn reloadFromOrigin_(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn reloadFromOrigin_sender_(self, sender: id) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn stopLoading_(self);

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn stopLoading_sender_(self, sender: id) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn loadData_MIMEType_characterEncodingName_baseURL_(
        self,
        data: id,
        MIMEType: id,
        characterEncodingName: id,
        baseURL: id,
    ) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Loading Content
    unsafe fn loadFileURL_allowingReadAccessToURL_(
        self,
        URL: id,
        allowingReadAccessToURL: id,
    ) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Scaling Content
    unsafe fn allowsMagnification(self) -> BOOL;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Scaling Content
    unsafe fn setAllowsMagnification_(self, allowsMagnification: BOOL);

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Scaling Content
    unsafe fn magnification(self) -> CGFloat;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Scaling Content
    unsafe fn setMagnification_(self, magnification: CGFloat);

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Scaling Content
    unsafe fn setMagnification_centeredAtPoint_(
        self,
        magnification: CGFloat,
        centeredAtPoint: CGPoint,
    );

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn allowsBackForwardNavigationGestures(self) -> BOOL;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn setAllowsBackForwardNavigationGestures_(
        self,
        allowsBackForwardNavigationGestures: BOOL,
    );

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn backForwardList(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn canGoBack(self) -> BOOL;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn canGoForward(self) -> BOOL;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn allowsLinkPreview(self) -> BOOL;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn setAllowsLinkPreview_(self, allowsLinkPreview: BOOL);

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn goBack_(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn goBack_sender_(self, sender: id) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn goForward_(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn goForward_sender_(self, sender: id) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn goToBackForwardListItem_(self, item: id) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Navigating
    unsafe fn loadRequest_(self, request: id);

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Executing JavaScript
    unsafe fn evaluateJavaScript_(
        self,
        javascriptString: id,
        completionHandler: &Block<(id, id), ()>,
    );

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// # Category
    /// Taking Snapshots
    unsafe fn takeSnapshotWithConfiguration_(
        self,
        snapshotConfiguration: id,
        completionHandler: extern "C" fn(id, id),
    );
}

impl WKWebView for id {
    /// Determining Whether WebKit can Load Content
    unsafe fn handlesURLScheme_(_: Self, urlScheme: id) -> BOOL {
        msg_send![class!(WKWebView), handlesURLScheme: urlScheme]
    }

    /// Initializing a Web View
    unsafe fn configuration(self) -> id {
        msg_send![self, configuration]
    }

    /// Initializing a Web View
    unsafe fn initWithFrame_configuration_(self, frameRect: NSRect, configuration: id) -> id {
        msg_send![self, initWithFrame:frameRect configuration:configuration]
    }

    /// Initializing a Web View
    unsafe fn initWithCoder_(self, coder: id) -> id {
        msg_send![self, initWithCoder: coder]
    }

    /// Inspecting the View Information
    unsafe fn scrollView(self) -> id {
        msg_send![self, scrollView]
    }

    /// Inspecting the View Information
    unsafe fn title(self) -> id {
        msg_send![self, title]
    }

    /// Inspecting the View Information
    unsafe fn URL(self) -> id {
        msg_send![self, URL]
    }

    /// Inspecting the View Information
    unsafe fn customUserAgent(self) -> id {
        msg_send![self, customUserAgent]
    }

    /// Inspecting the View Information
    unsafe fn setCustomUserAgent_(self, customUserAgent: id) {
        msg_send![self, setCustomUserAgent: customUserAgent]
    }

    /// Setting Delegates
    unsafe fn navigationDelegate(self) -> id {
        msg_send![self, navigationDelegate]
    }

    /// Setting Delegates
    unsafe fn setNavigationDelegate_(self, navigationDelegate: id) {
        msg_send![self, setNavigationDelegate: navigationDelegate]
    }

    /// Setting Delegates
    unsafe fn UIDelegate(self) -> id {
        msg_send![self, UIDelegate]
    }

    /// Setting Delegates
    unsafe fn setUIDelegate_(self, navigationDelegate: id) {
        msg_send![self, setUIDelegate: navigationDelegate]
    }

    /// Loading Content
    unsafe fn estimatedProgress(self) -> c_double {
        msg_send![self, estimatedProgress]
    }

    /// Loading Content
    unsafe fn hasOnlySecureContent(self) -> BOOL {
        msg_send![self, hasOnlySecureContent]
    }

    /// Loading Content
    unsafe fn loadHTMLString_baseURL_(self, string: id, baseURL: id) -> id {
        msg_send![self, loadHTMLString:string baseURL:baseURL]
    }

    /// Loading Content
    unsafe fn loading(self) -> BOOL {
        msg_send![self, loading]
    }

    /// Loading Content
    unsafe fn reload_(self) -> id {
        msg_send![self, reload]
    }

    /// Loading Content
    unsafe fn reload_sender_(self, sender: id) -> id {
        msg_send![self, reload: sender]
    }

    /// Loading Content
    unsafe fn reloadFromOrigin_(self) -> id {
        msg_send![self, reloadFromOrigin]
    }

    /// Loading Content
    unsafe fn reloadFromOrigin_sender_(self, sender: id) -> id {
        msg_send![self, reloadFromOrigin: sender]
    }

    /// Loading Content
    unsafe fn stopLoading_(self) {
        msg_send![self, stopLoading]
    }

    /// Loading Content
    unsafe fn stopLoading_sender_(self, sender: id) -> id {
        msg_send![self, stopLoading: sender]
    }

    /// Loading Content
    unsafe fn loadData_MIMEType_characterEncodingName_baseURL_(
        self,
        data: id,
        MIMEType: id,
        characterEncodingName: id,
        baseURL: id,
    ) -> id {
        msg_send![
            self,
            loadData:data
                MIMEType:MIMEType
                characterEncodingName:characterEncodingName
                baseURL:baseURL
        ]
    }

    /// Loading Content
    unsafe fn loadFileURL_allowingReadAccessToURL_(
        self,
        URL: id,
        allowingReadAccessToURL: id,
    ) -> id {
        msg_send![
            self,
            loadFileURL:URL
                allowingReadAccessToURL:allowingReadAccessToURL
        ]
    }

    // Scaling Content
    unsafe fn allowsMagnification(self) -> BOOL {
        msg_send![self, allowsMagnification]
    }

    unsafe fn setAllowsMagnification_(self, allowsMagnification: BOOL) {
        msg_send![self, setAllowsMagnification: allowsMagnification]
    }

    unsafe fn magnification(self) -> CGFloat {
        msg_send![self, magnification]
    }

    unsafe fn setMagnification_(self, magnification: CGFloat) {
        msg_send![self, setMagnification: magnification]
    }

    unsafe fn setMagnification_centeredAtPoint_(
        self,
        magnification: CGFloat,
        centeredAtPoint: CGPoint,
    ) {
        msg_send![
            self,
            setMagnification:magnification
                centeredAtPoint:centeredAtPoint
        ]
    }

    // Navigating
    unsafe fn allowsBackForwardNavigationGestures(self) -> BOOL {
        msg_send![self, allowsBackForwardNavigationGestures]
    }

    unsafe fn setAllowsBackForwardNavigationGestures_(
        self,
        allowsBackForwardNavigationGestures: BOOL,
    ) {
        msg_send![
            self,
            setAllowsBackForwardNavigationGestures: allowsBackForwardNavigationGestures
        ]
    }

    unsafe fn backForwardList(self) -> id {
        msg_send![self, backForwardList]
    }

    unsafe fn canGoBack(self) -> BOOL {
        msg_send![self, canGoBack]
    }

    unsafe fn canGoForward(self) -> BOOL {
        msg_send![self, canGoForward]
    }

    unsafe fn allowsLinkPreview(self) -> BOOL {
        msg_send![self, allowsLinkPreview]
    }

    unsafe fn setAllowsLinkPreview_(self, allowsLinkPreview: BOOL) {
        msg_send![self, setAllowsLinkPreview: allowsLinkPreview]
    }

    unsafe fn goBack_(self) -> id {
        msg_send![self, goBack]
    }

    unsafe fn goBack_sender_(self, sender: id) -> id {
        msg_send![self, goBack: sender]
    }

    unsafe fn goForward_(self) -> id {
        msg_send![self, goForward]
    }

    unsafe fn goForward_sender_(self, sender: id) -> id {
        msg_send![self, goForward: sender]
    }

    unsafe fn goToBackForwardListItem_(self, item: id) -> id {
        msg_send![self, goToBackForwardListItem: item]
    }

    unsafe fn loadRequest_(self, request: id) {
        msg_send![self, loadRequest: request]
    }

    // Executing JavaScript
    unsafe fn evaluateJavaScript_(
        self,
        javascriptString: id,
        completionHandler: &Block<(id, id), ()>,
    ) {
        msg_send![
            self,
            evaluateJavaScript:javascriptString
                completionHandler:completionHandler
        ]
    }

    // Taking Snapshots
    unsafe fn takeSnapshotWithConfiguration_(
        self,
        snapshotConfiguration: id,
        completionHandler: extern "C" fn(id, id),
    ) {
        msg_send![
            self,
            takeSnapshotWithConfiguration:snapshotConfiguration
                completionHandler:completionHandler
        ]
    }
}
