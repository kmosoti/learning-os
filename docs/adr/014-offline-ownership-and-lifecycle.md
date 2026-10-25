# ADR-014: Offline ownership and PWA lifecycle

Status: Proposed formal record; fixed direction accepted and implementation authorized by the owner on 2026-10-25 with “Implement the proposed plan.” Mechanism choices are deferred to the approved experiments. PR approval/merge remains pending.

Related: [#49](https://github.com/kmosoti/learning-os/issues/49) and [execution contract](../architecture/v2-execution-contract.md).

## Decision
The browser executes the application locally; predownloaded source and question bundles allow the complete seeded lesson offline. Axum SSR is a delivery mechanism, not a prerequisite for local commands. Cross-device replication is deferred. Stable operation/event/device identity and causal revision metadata preserve a future explicit replication boundary without introducing a CRDT.

## Bootstrap and updates
Hydrate the same neutral SSR bootstrap, then read the local store. Cache shell, bindings, WASM, styles, fonts and lesson material. A direct session URL must hard-reload offline. An old cache must not overwrite a newer schema. Use staged cache activation and user-safe updates; no unconditional takeover of an active session. Storage migration and cache installation failures leave the prior usable version intact.

## Visibility and privacy
Withheld feedback is absent from UI/DOM/accessibility output until permitted. Offline grading data is inherently inspectable; no anti-cheating secrecy claim. Personal evidence is not uploaded; no provider credentials or live inference in the slice. Render source/model text as untrusted. Rich executable artifacts stay disabled.

## Confirmation
Offline direct navigation, acknowledged write plus abrupt browser restart, retry after lost ACK, two-tab revision conflict, quota failure, partial/blocked migration, export/import, draft restoration, safe cache upgrade and owner-run Android installation. Report browser versions and durability limits.
