---
id: D-261128-keep-published-package-name
status: accepted
date: 2026-11-28
reopen_when: a 1.0 release, a breaking API change, or a decision to move the repo to the evocaslab org
---

# Keep the published name `path_as_uri` instead of `evocaslab-path-as-uri`

## Context

The public-repo standard asks for registry names `evocaslab-<name>`. `path_as_uri` has been on crates.io since 2025-04-20 (<https://crates.io/crates/path_as_uri>), and it has about 1000 downloads. A rename on crates.io is a new package: the old one stays, and every user has to change a manifest line to keep getting fixes.

## Drivers

- users keep working without a change
- one name per package, no two packages with the same code
- the org naming rule for new packages

## Considered options

| Option                                                   | Fits the drivers                          | Cost                                | Hard to undo                               | Evidence                               |
| -------------------------------------------------------- | ----------------------------------------- | ----------------------------------- | ------------------------------------------ | -------------------------------------- |
| **keep `path_as_uri`** (chosen)                          | users: yes; naming rule: no, an exception | none                                | no                                         | <https://crates.io/crates/path_as_uri> |
| publish `evocaslab-path-as-uri`, deprecate `path_as_uri` | users must move                           | two packages for a while, a release | yes, a published name cannot be taken back | <https://crates.io/crates/path_as_uri> |

## Decision

In the context of a small published package with existing users, facing the org rule for new names, we chose to keep `path_as_uri` over a renamed copy, to achieve no breaking change for users, accepting that the `package-names` check of the public-repo standard reports it until the checker has an exception list.

## Consequences

- Good: no user has to change a manifest; links on the registry keep working.
- Bad: the name does not follow the `evocaslab-` rule; the public-ready check fails on `package-names`.

## Confirmation

Review only: the manifest name is checked by the public-ready check, which reports it.

## Sources

- <https://crates.io/crates/path_as_uri> (read 2026-11-28)
- Public-repo standard, rule 11 (names of public tools and libraries)
