# CYBERGRID palettes — `core` branch

This branch wires in the shared [`cybercore`](https://github.com/darkstardevx/cybercore)
crate (schema **v2**) as a second theme source, alongside CYBERDECK's own
file-based chooser (`/api/themes`). Nothing about the existing chooser changes.

`cybercore/schema/cybergrid.json` is the one place the CYBERGRID palette is
edited for the whole Cybercore Systems Framework. Schema v2 carries a **map of
named themes**, each with 11 semantic roles: `bg white acid_green hot_pink
purple cyan orange red panel line muted`.

## Endpoints (added in `src/cybergrid.rs`)

| route | returns |
|---|---|
| `GET /api/cybergrid/themes` | `{ "active": "<slug>", "themes": [ { "name", …11 roles… } ] }` for every theme |
| `GET /api/cybergrid/css/:name` | a ready `:root{ --bg:#…; --fg:#…; --acid:#…; … }` block (`text/css`); unknown name falls back to the active theme |

Shipped themes: `neon-night` (active), `tokyo-night`, `dracula`, `nord`,
`catppuccin-mocha`, `gruvbox-dark`, `kanagawa`, `synthwave-84`,
`synthwave-grid-runner`, `neosynth-laser-grid`, `cyberpunk-sub-grid-zero`,
`cyberpunk-2077`.

## Pinning the active theme

Set `CYBERGRID_THEME=<slug>` in the environment to override the schema default
for this process. To add or change themes, edit `cybergrid.json` in the
`cybercore` repo (`tools/ansi2cybergrid.py` converts 16-colour terminal themes)
and bump the `tag = "…"` in `Cargo.toml`.

## Front-end wiring (not done here)

The chooser UI can `fetch('/api/cybergrid/themes')` to list them and, on
select, inject `/api/cybergrid/css/<name>` into a `<style>` / `<link>` — same
shape the existing theme apply path already uses.
