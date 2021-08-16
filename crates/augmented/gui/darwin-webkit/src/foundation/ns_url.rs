//! Bindings for `NSURL`
//!
//! https://developer.apple.com/documentation/foundation/nsurl?language=objc
use cocoa::base::{id, BOOL};

use foundation::ns_url_bookmark_creation_options::{
    NSURLBookmarkCreationOptions, NSURLBookmarkFileCreationOptions,
};
use foundation::ns_url_bookmark_resolution_options::NSURLBookmarkResolutionOptions;

pub trait NSURL: Sized {
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn alloc(_: Self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLWithString_(_: Self, string: id) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initWithString_(self, string: id) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLWithString_relativeToURL_(_: Self, string: id, url: id) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initWithString_relativeToURL_(self, string: id, url: id) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileURLWithPath_isDirectory_(_: Self, path: id, is_dir: BOOL) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initFileURLWithPath_isDirectory_(self, path: id, is_dir: BOOL) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileURLWithPath_relativeToURL_(_: Self, path: id, url: id) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initFileURLWithPath_relativeToURL_(self, path: id, url: id) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileURLWithPath_isDirectory_relativeToURL_(
        _: Self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initFileURLWithPath_isDirectory_relativeToURL_(
        self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileURLWithPath_(_: Self, path: id) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initFileURLWithPath_(self, path: id) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileURLWithPathComponents_(
        _: Self,
        path_components: id, /* (NSArray<NSString*>*) */
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByResolvingAliasFileAtURL_options_error_(
        _: Self,
        url: id,
        options: NSURLBookmarkResolutionOptions,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        _: Self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    // unsafe fn fileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    /// # Safety
    /// All the FFI functions are unsafe.
    // unsafe fn getFileSystemRepresentation_maxLength_
    /// # Safety
    /// All the FFI functions are unsafe.
    // unsafe fn initFileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn absoluteURLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initAbsoluteURLWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn dataRepresentation(self) -> id /* (NSData) */;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn isEqual_(self, id: id) -> BOOL;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn checkResourceIsReachableAndReturnError_(
        self,
        error: id, /* (NSError _Nullable) */
    ) -> BOOL;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn isFileReferenceURL(self) -> BOOL;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn isFileURL(self) -> BOOL;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn absoluteString(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn absoluteURL(self) -> id /* (NSURL) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn baseURL(self) -> id /* (NSURL) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    // unsafe fn fileSystemRepresentation
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fragment(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn host(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn lastPathComponent(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn parameterString(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn password(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn path(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn pathComponents(self) -> id /* (NSArray<NSString*>) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn pathExtension(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn port(self) -> id /* (NSNumber) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn query(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn relativePath(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn relativeString(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn resourceSpecifier(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn scheme(self) -> id /* (NSString) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn standardizedURL(self) -> id /* (NSURL) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn user(self) -> id /* (NSString) */;

    // unsafe fn resourceValuesForKeys_error_
    // unsafe fn getResourceValue_forKey_error_
    // unsafe fn setResourceValue_forKey_error_
    // unsafe fn setResourceValues_error_
    // unsafe fn removeAllCachedResourceValues
    // unsafe fn removeCachedResourceValueForKey_
    // unsafe fn setTemporaryResourceValue_forKey_
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn NSURLResourceKey(self) -> id /* (NSString) */;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn filePathURL(self) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileReferenceURL(self) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByAppendingPathComponent_(self, path_component: id /* (NSString) */) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByAppendingPathComponent_isDirectory_(
        self,
        path_component: id, /* (NSString) */
        is_dir: BOOL,
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByAppendingPathExtension_(self, extension: id /* (NSString) */) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByDeletingLastPathComponent(self) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByDeletingPathExtension(self) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByResolvingSymlinksInPath(self) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByStandardizingPath(self) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn hasDirectoryPath(self) -> BOOL;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn bookmarkDataWithContentsOfURL_error_(
        _: Self,
        url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn bookmarkDataWithOptions_includingResourceValuesForKeys_relativeToURL_error_(
        self,
        options: NSURLBookmarkCreationOptions,
        resource_value_for_keys: id, /* (NSArray<NSURLResourceKey>) */
        relative_to_url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */;
    // unsafe fn resourceValuesForKeys_fromBookmarkData_
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn writeBookmarkData_toURL_options_error_(
        _: Self,
        data: id, /* (NSData) */
        to_url: id,
        options: NSURLBookmarkFileCreationOptions,
        error: id, /* (NSError _Nullable) */
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn startAccessingSecurityScopedResource(self) -> BOOL;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn stopAccessingSecurityScopedResource(self);
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn NSURLBookmarkFileCreationOptions(self) -> NSURLBookmarkFileCreationOptions;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn NSURLBookmarkCreationOptions(self) -> NSURLBookmarkCreationOptions;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn NSURLBookmarkResolutionOptions(self) -> NSURLBookmarkResolutionOptions;

    // unsafe fn checkPromisedItemIsReachableAndReturnError_
    // unsafe fn getPromisedItemResourceValue_forKey_error_
    // unsafe fn promisedItemResourceValuesForKeys_error_

    // unsafe fn URLFromPasteboard_
    // unsafe fn writeToPasteboard_
}

impl NSURL for id {
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSURL), alloc]
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLWithString_(_: Self, string: id) -> id {
        msg_send![class!(NSURL), URLWithString: string]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initWithString_(self, string: id) -> id {
        msg_send![self, initWithString: string]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLWithString_relativeToURL_(_: Self, string: id, url: id) -> id {
        msg_send![class!(NSURL), URLWithString: string relativeToURL:url]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initWithString_relativeToURL_(self, string: id, url: id) -> id {
        msg_send![self, initWithString:string relativeToURL:url]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileURLWithPath_isDirectory_(_: Self, path: id, is_dir: BOOL) -> id {
        msg_send![class!(NSURL), fileURLWithPath:path isDirectory:is_dir]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initFileURLWithPath_isDirectory_(self, path: id, is_dir: BOOL) -> id {
        msg_send![self, initFileURLWithPath:path isDirectory:is_dir]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileURLWithPath_relativeToURL_(_: Self, path: id, url: id) -> id {
        msg_send![class!(NSURL), fileURLWithPath:path relativeToURL:url]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initFileURLWithPath_relativeToURL_(self, path: id, url: id) -> id {
        msg_send![self, initFileURLWithPath:path relativeToURL:url]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileURLWithPath_isDirectory_relativeToURL_(
        _: Self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id {
        msg_send![class!(NSURL), fileURLWithPath:path isDirectory:is_dir relativeToURL:url]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initFileURLWithPath_isDirectory_relativeToURL_(
        self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id {
        msg_send![self, initFileURLWithPath:path isDirectory:is_dir relativeToURL:url]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileURLWithPath_(_: Self, path: id) -> id {
        msg_send![class!(NSURL), fileURLWithPath: path]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initFileURLWithPath_(self, path: id) -> id {
        msg_send![self, initFileURLWithPath: path]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileURLWithPathComponents_(
        _: Self,
        path_components: id, /* (NSArray<NSString*>*) */
    ) -> id {
        msg_send![class!(NSURL), fileURLWithPathComponents: path_components]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByResolvingAliasFileAtURL_options_error_(
        _: Self,
        url: id,
        options: NSURLBookmarkResolutionOptions,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![class!(NSURL), URLByResolvingAliasFileAtURL:url options:options error:error]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        _: Self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![class!(NSURL), URLByResolvingBookmarkData:data options:options relativeToURL:relative_to_url bookmarkDataIsStale:is_stale error:error]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![self, initByResolvingBookmarkData:data options:options relativeToURL:relative_to_url bookmarkDataIsStale:is_stale error:error]
    }
    // unsafe fn fileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    // unsafe fn getFileSystemRepresentation_maxLength_
    // unsafe fn initFileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn absoluteURLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![class!(NSURL), absoluteURLWithDataRepresentation:data relativeToURL:url]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initAbsoluteURLWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![self, initAbsoluteURLWithDataRepresentation:data relativeToURL:url]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![class!(NSURL), URLWithDataRepresentation:data relativeToURL:url]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![self, initWithDataRepresentation:data relativeToURL:url]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn dataRepresentation(self) -> id /* (NSData) */ {
        msg_send![self, dataRepresentation]
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn isEqual_(self, id: id) -> BOOL {
        msg_send![self, isEqual: id]
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn checkResourceIsReachableAndReturnError_(
        self,
        error: id, /* (NSError _Nullable) */
    ) -> BOOL {
        msg_send![self, checkResourceIsReachableAndReturnError: error]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn isFileReferenceURL(self) -> BOOL {
        msg_send![self, isFileReferenceURL]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn isFileURL(self) -> BOOL {
        msg_send![self, isFileURL]
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn absoluteString(self) -> id /* (NSString) */ {
        msg_send![self, absoluteString]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn absoluteURL(self) -> id /* (NSURL) */ {
        msg_send![self, absoluteURL]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn baseURL(self) -> id /* (NSURL) */ {
        msg_send![self, baseURL]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    // unsafe fn fileSystemRepresentation
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fragment(self) -> id /* (NSString) */ {
        msg_send![self, fragment]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn host(self) -> id /* (NSString) */ {
        msg_send![self, host]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn lastPathComponent(self) -> id /* (NSString) */ {
        msg_send![self, lastPathComponent]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn parameterString(self) -> id /* (NSString) */ {
        msg_send![self, parameterString]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn password(self) -> id /* (NSString) */ {
        msg_send![self, password]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn path(self) -> id /* (NSString) */ {
        msg_send![self, path]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn pathComponents(self) -> id /* (NSArray<NSString*>) */ {
        msg_send![self, pathComponents]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn pathExtension(self) -> id /* (NSString) */ {
        msg_send![self, pathExtension]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn port(self) -> id /* (NSNumber) */ {
        msg_send![self, port]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn query(self) -> id /* (NSString) */ {
        msg_send![self, query]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn relativePath(self) -> id /* (NSString) */ {
        msg_send![self, relativePath]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn relativeString(self) -> id /* (NSString) */ {
        msg_send![self, relativeString]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn resourceSpecifier(self) -> id /* (NSString) */ {
        msg_send![self, resourceSpecifier]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn scheme(self) -> id /* (NSString) */ {
        msg_send![self, scheme]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn standardizedURL(self) -> id /* (NSURL) */ {
        msg_send![self, standardizedURL]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn user(self) -> id /* (NSString) */ {
        msg_send![self, user]
    }

    // unsafe fn resourceValuesForKeys_error_
    // unsafe fn getResourceValue_forKey_error_
    // unsafe fn setResourceValue_forKey_error_
    // unsafe fn setResourceValues_error_
    // unsafe fn removeAllCachedResourceValues
    // unsafe fn removeCachedResourceValueForKey_
    // unsafe fn setTemporaryResourceValue_forKey_
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn NSURLResourceKey(self) -> id /* (NSString) */ {
        msg_send![self, NSURLResourceKey]
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn filePathURL(self) -> id {
        msg_send![self, filePathURL]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn fileReferenceURL(self) -> id {
        msg_send![self, fileReferenceURL]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByAppendingPathComponent_(self, path_component: id /* (NSString) */) -> id {
        msg_send![self, URLByAppendingPathComponent: path_component]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByAppendingPathComponent_isDirectory_(
        self,
        path_component: id, /* (NSString) */
        is_dir: BOOL,
    ) -> id {
        msg_send![self, URLByAppendingPathComponent:path_component isDirectory:is_dir]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByAppendingPathExtension_(self, extension: id /* (NSString) */) -> id {
        msg_send![self, URLByAppendingPathExtension: extension]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByDeletingLastPathComponent(self) -> id {
        msg_send![self, URLByDeletingLastPathComponent]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByDeletingPathExtension(self) -> id {
        msg_send![self, URLByDeletingPathExtension]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByResolvingSymlinksInPath(self) -> id {
        msg_send![self, URLByResolvingSymlinksInPath]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn URLByStandardizingPath(self) -> id {
        msg_send![self, URLByStandardizingPath]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn hasDirectoryPath(self) -> BOOL {
        msg_send![self, hasDirectoryPath]
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn bookmarkDataWithContentsOfURL_error_(
        _: Self,
        url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */ {
        msg_send![class!(NSURL), bookmarkDataWithContentsOfURL:url error:error]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn bookmarkDataWithOptions_includingResourceValuesForKeys_relativeToURL_error_(
        self,
        options: NSURLBookmarkCreationOptions,
        resource_value_for_keys: id, /* (NSArray<NSURLResourceKey>) */
        relative_to_url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */ {
        msg_send![self, bookmarkDataWithOptions:options includingResourceValuesForKeys:resource_value_for_keys relativeToURL:relative_to_url error:error]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    // unsafe fn resourceValuesForKeys_fromBookmarkData_
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn writeBookmarkData_toURL_options_error_(
        _: Self,
        data: id, /* (NSData) */
        to_url: id,
        options: NSURLBookmarkFileCreationOptions,
        error: id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![class!(NSURL), writeBookmarkData:data toURL:to_url options:options error:error]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn startAccessingSecurityScopedResource(self) -> BOOL {
        msg_send![self, startAccessingSecurityScopedResource]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn stopAccessingSecurityScopedResource(self) {
        msg_send![self, stopAccessingSecurityScopedResource]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn NSURLBookmarkFileCreationOptions(self) -> NSURLBookmarkFileCreationOptions {
        msg_send![self, NSURLBookmarkFileCreationOptions]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn NSURLBookmarkCreationOptions(self) -> NSURLBookmarkCreationOptions {
        msg_send![self, NSURLBookmarkCreationOptions]
    }
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn NSURLBookmarkResolutionOptions(self) -> NSURLBookmarkResolutionOptions {
        msg_send![self, NSURLBookmarkResolutionOptions]
    }

    // unsafe fn checkPromisedItemIsReachableAndReturnError_
    // unsafe fn getPromisedItemResourceValue_forKey_error_
    // unsafe fn promisedItemResourceValuesForKeys_error_

    // unsafe fn URLFromPasteboard_
    // unsafe fn writeToPasteboard_
}
