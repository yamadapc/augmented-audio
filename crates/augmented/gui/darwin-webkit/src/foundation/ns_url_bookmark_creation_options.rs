pub enum NSURLBookmarkCreationOptions {
    NSURLBookmarkCreationPreferFileIDResolution = 1 << 8,
    NSURLBookmarkCreationMinimalBookmark = 1 << 9,
    NSURLBookmarkCreationSuitableForBookmarkFile = 1 << 10,
    NSURLBookmarkCreationWithSecurityScope = 1 << 11,
    NSURLBookmarkCreationSecurityScopeAllowOnlyReadAccess = 1 << 12,
}

pub type NSURLBookmarkFileCreationOptions = NSURLBookmarkCreationOptions;
