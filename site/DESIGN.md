# jev-stars site DESIGN.md (verified spec, v2)

## Design contract

- First-Read Object: the install command `cargo install jev-stars` inside an asymmetric split hero, left copy and right live terminal. A visitor grasps the product in one glance.
- Primary Action: copy the install command, then read the proof cards. One action, no signup funnel.
- Design Read: devtool landing for AI agent builders, terminal native memory language, Primer Brand honesty in static HTML.
- Dials: VARIANCE 8, MOTION 6, DENSITY 4.
- Performance budget: single HTML file plus one CSS file, zero JS frameworks, zero webfonts. Page under 60KB, CSS under 12KB, LCP under 1.0s on 4G, INP under 100ms, CLS 0.
- Token drift story: tokens live only in this file and `style.css` under `:root`. Any new color lands here first. CI diffs computed tokens against this file and fails on change without a spec update. Baseline locked at v2 hero split plus bento plus accordion.

## Causal lines (derivation receipts)

- Accent gold `#f5b301`: causal line is the GitHub star glyph itself, the product counts stars, so the star color is the accent. Not a warm default, a product receipt.
- Substrate near black `#0f1114`: causal line is the terminal surface where `jst` runs, pulled from the hero terminal block background in `index.html`.
- Mono first hierarchy: causal line is the CLI output blocks that carry every proof on the page, set in `index.html` `pre` blocks.

## Tokens

- Substrate: `#0f1114` (near black, warm neutral tint). Never flat black anywhere.
- Surface ladder: `#0f1114` base, `#161a20` raised, `#1e242c` overlay. Elevation by surface step only, no drop shadows.
- Accent: gold `#f5b301` (star). Text on accent: `#0f1114`. Single accent only, no gradients. Accent scarcity: one accent bearing element per viewport (one CTA, one lit stat, one terminal cursor).
- Text: `#ece8e2` primary, `#a8a29a` secondary, `#6b6560` muted. Contrast pairs verified: primary on substrate 15.9 to 1, secondary on substrate 7.1 to 1.
- Focus ring: 2px solid `#f5b301` with 2px offset on all interactive elements. Visible focus via `:focus-visible` on every link, button, tab.
- Neutrals carry the anchor warmth, no pure grey next to gold.

## Type (2 plus 1 ceiling)

- Display: `Clash Display`, Fontshare self hosted woff2 with `font-display: swap`, fallback stack `-apple-system, Segoe UI, sans-serif`. Weight 600 only, ceiling fixed. Scale by ratio 1.333 off 16px body, display cap 5.5rem. Headlines sized to character count: short lines get full display, long lines step down one rung.
- Body: system stack, 400 and 500 only. Weight contrast 200 minimum against display.
- Mono: `JetBrains Mono, Consolas, monospace` for commands, stats, code, terminal. Tabular figures on all numerals via `font-variant-numeric: tabular-nums`.
- Headings use `text-wrap: balance`, body uses `text-wrap: pretty`.
- Letter spacing tightens with size, near 0 at body. No all caps display under line height 1.02.
- Banned as defaults: Inter, Roboto, Space Grotesk, system only display. Clash Display rotates per project, never reused twice in a row across repos.

## Radii (concentric)

- Outer card radius `R_outer = 16px`, padding `p = 16px`.
- Inner element radius `R_inner = max(0, R_outer - p) = 0px` for flush inner blocks, `8px` for nested chips where `p = 8px`.
- Buttons pill, cards 16px, code blocks 16px with 0px inner rows. One radius system, held everywhere.

## Motion

- Easing: `cubic-bezier(0.16, 1, 0.3, 1)` everywhere for enter and arrive. Exit runs at 65 percent of enter duration, capped 200ms. Never the browser default ease, never `ease-in-out`. Never `transition: all`, transition `color, background-color, transform, opacity` only.
- Spring note: stiffness 400 plus damping 32 tier for press feedback, instant tactile drop under 120ms.
- Reveal: IntersectionObserver adds `.in`, translateY(12px) to 0, 500ms, stagger 60ms capped at 8 children. Animate transform and opacity only.
- Terminal: typed output loop, caret blink 1s steps, pauses on `prefers-reduced-motion: reduce` showing the full static frame.
- Starfield canvas: single 2D canvas behind the data section, one rAF loop, paused via IntersectionObserver when offscreen, full static frame under `prefers-reduced-motion: reduce`.
- Press: `:active` scale 0.97 under 120ms on buttons and tabs.
- `prefers-reduced-motion: reduce` disables reveal, typing, canvas, and smooth scroll, rendering the complete static page.

## Layout (one tool, one page, rotated families)

- Single page scope: one tool, one conversion (install). Sections never repeat a layout family.
- Header: slim bar, logo plus wordmark left, nav right on one line, max 72px tall.
- Hero: asymmetric split. Left column holds eyebrow, H1 max 2 lines, lede max 20 words, install command, badges. Right column holds the live terminal island with real `jst context` output. Hero fits the first viewport, top padding capped at 6rem.
- Proof: three proof cards plus one wide ledger strip (hash gate rerun story), not a plain table.
- Data: bento grid, exactly 7 cells for 7 languages (2 large plus 5 small), 2 cells carry visual variation (star counts as large display numbers). Starfield canvas sits behind. No filled track progress bars, numbers plus hairline rules only.
- Commands: grouped 3 cluster accordion (find, remember, serve), not a definition list. First cluster open by default, others collapsed with full content reachable by keyboard.
- MCP: config block plus copy button plus tool chips.
- Install: three tabs kept (cargo, binary, source), working pattern preserved.
- FAQ plus Limits: single Read mode column, stacked headline plus body, no split header pattern.
- Footer: mandatory order Ecosystem, Author, Social, closer line. Untouched.
- Max width 1080px, gutters 24px, 96px between sections.
- Eyebrow budget: max 1 per 3 sections, hero counts as 1. No section number eyebrows, no pills overlaid on images, no scroll cues, no locale strips.
- One CTA intent per page: copy install. No duplicate intent labels.
- Mobile under 768px: every multi column layout collapses to strict single column, touch targets 44px minimum, no horizontal overflow at 600px.

## Signature detail

- The terminal island types real measured `jst` output on load with a gold block cursor. Structural, not a sticker: the product is a CLI, the hero shows the CLI working.

## Craft rules

- Optical centering on logo glyphs, 1px outline on terminal so edges never vanish on dark, 44px hit targets minimum.
- Icons: Phosphor single family if icons are added later. No hand rolled SVG paths, no emoji glyphs.
- Copy: real measured numbers only, each with a rerun command in proof. No placeholder blocks. Buttons name the verb. No em dash character anywhere in visible copy, use commas, colons, or periods.
- Empty states: accordion and tabs render usable without JS (first cluster open, all install blocks visible when JS is off).
- Links describe destinations, form free page needs no labels audit, all images have alt or are aria hidden.

## Do and Do not

- Do keep one accent, one theme, one radius system, one icon family.
- Do preserve slugs, anchor IDs, nav labels, OG tags, canonical, JSON-LD from v1.
- Do not add a second accent, a light section, a centered hero, equal cards, purple gradients, or a marquee.
- Do not overlay labels on images, print category labels under logos, or add version footers.
- Do not animate width, top, left, height, or color channels. Transform and opacity only.
