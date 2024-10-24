# calmuxd
`calmuxd` is an extremely basic calendar feed muxing agent.
Given a configured array of `webdav://` (`.ics`) feeds, it emits a unified feed of all parsed events.

## Notes
- Calendar contents are fetched on request. No caching is performed.
- Similarly, no event deduplication is performed. Please do such yourself manually.

## Development
We leverage [direnv](https://github.com/direnv/direnv) to maintain a standardized development environment via [Nix flakes](https://nixos.wiki/wiki/flakes).
Please install and configure both on your system before continuing.

A development workflow might be similar to following:

1. Check out this repository
2. If you have not already, `direnv allow` to enter the Nix-provided development environment. (Alternatively, run `nix shell` directly.)
3. Make tweaks as necessary.
4. Ensure you have run `cargo fmt` for Rust changes. If Nix-related files have been modified, similarly perform `nix fmt`.
