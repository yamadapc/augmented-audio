# Auto-update
This file documents the auto-update process and components.

> This is WIP & only partially implemented. The releases will be uploaded to S3.

## Overview
We'd like the applications to provide prompts for users telling them they may update the software.

Whenever we publish a new update for the application, the developer adds metadata around the release to a central
server-side store.

When the application boots on users' computers as well as on a polling basis, it checks if there's a new update and
prompts the user to download it.

## Entities and definitions
### Release
A `Release` represents a published version of the app.
```typescript
interface Release {
    /**
     * A version key for this release
     */
    key: ReleaseKey,
    /**
     * A timestamp for when this was released
     */
    createdAt: Date,
    /**
     * Release notes in text/html format
     */
    releaseNotes: null | {
        text: null | string,
        html: null | string,
    },
    /**
     * An URL from which the updated "APP BUNDLE" can be downloaded from
     */
    fileDownloadUrl: string,
    /**
     * An URL from which the user may manually view & download the release
     */
    userDownloadUrl: string,
}
```

### ReleaseKey
```typescript
type ReleaseKey = string;
```
At build time, a **ReleaseKey** will be baked into the binary. This should match the release key on the update server.

The release key should follow semantic versioning.

## Update process
At most once a day, after starting, the app will fetch the latest **ReleaseKey** from the **UPDATES_ORIGIN**.

This will be fetched by issuing a GET request to `$UPDATES_ORIGIN/latest_release.json`. This request should return a
`Release` object with a `200` response. Extra headers may be provided and handled by the server.

Fetching `$UPDATES_ORIGIN/releases.json` should return a list of the last few releases so a more comprehensive can be
rendered if desired.

To release, we'll push a new **release** and update the key.
