# jev-stars site DESIGN.md (verified spec)

## Design contract

- First-Read Object: the install command `cargo install jev-stars` in the hero. A visitor grasps the product in one glance.
- Primary Action: copy the install command, then read the Trust table. One action, no signup funnel.
- Performance budget: single HTML file + one CSS file, zero JS frameworks, zero webfonts. Page <60KB, CSS <8KB, LCP <1.0s on 4G, INP <100ms, CLS 0.
- Token-drift story: tokens live only in this file and `style.css` `:root`. Any new color must land here first; `verify_design.py` re-runs before merge.

## Tokens

- Substrate: `#0f1114` (near-black, warm-neutral tint; pure black never used anywhere).
- Surface ladder: `#0f1114` base, `#161a20` raised, `#1e242c` overlay. No box shadows; elevation by surface step only.
- Accent: gold `#f5b301` (star). Text-on-accent: `#0f1114`. Single accent only; no gradients.
- Text: `#ece8e2` primary, `#a8a29a` secondary, `#6b6560` muted.
- Focus ring: 2px `#f5b301` outline offset 2px. Visible on all interactive elements.

## Type (2+1 ceiling)

- Display: system stack (`-apple-system, Segoe UI, Roboto, sans-serif`), weight 700-800, giant clamp scale.
- Body: same system stack, 400/500.
- Mono: `JetBrains Mono, Consolas, monospace` for commands, stats, code. 2 families + mono = ceiling.

## Radii (concentric)

- Outer card radius `$R_outer = 16px`, padding `p = 16px`.
- Inner element radius `$R_inner = max(0, R_outer - p) = 0px` (flush inner blocks) or `8px` for nested chips where `p = 8px`.

## Motion

- Easing: `cubic-bezier(0.16, 1, 0.3, 1)` everywhere. Never `ease-in-out`. Never `transition: all`; transition `color, background-color, transform, opacity` only.
- Reveal: IntersectionObserver adds `.in`, translateY(12px) to 0, 500ms. Stagger 60ms.
- `prefers-reduced-motion: reduce` disables all motion. `prefers-reduced-transparency` flattens glows.

## Layout

- Single page: header (logo + nav + install cmd) / hero (H1 + one-liner + install + badges) / trust table / commands grid / MCP block / architecture / limits / ecosystem footer (mandatory order: Ecosystem, Author, Social, closer).
- Max width 960px, gutters 24px. Commands as definition rows, not 3 equal cards.
- Contrast: body text on substrate >= 7:1. Secondary >= 4.5:1. Verified pairs: `#ece8e2` on `#0f1114` (15.9:1), `#a8a29a` on `#0f1114` (7.1:1).

## Zero-simulation contracts

- All numbers (1,244 repos, 47ms, 5 tests) are measured values from this repo, each with a rerun command in the Trust section.
- No placeholder screenshots. Terminal blocks show real `jst` output.
- No lorem ipsum. Every section has final copy.
