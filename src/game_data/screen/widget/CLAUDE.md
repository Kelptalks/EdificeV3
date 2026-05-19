# Widget System

UI layer for EdificeV3. Everything drawn to screen that isn't the raw voxel
world is a widget: panels, buttons, text, the drone-programming editor, windows,
and even the world viewport itself (`PlayWorldViewRender`).

## Core: the `Widget` trait (`widget.rs`)

Every concrete widget implements `Widget`. It requires only two methods; the
rest have defaults backed by `WidgetProperties`:

- `get_widget_properties()` / `get_mut_widget_properties()` — required.
- `size()` — required. Computes layout (the widget's `prefered_scale`).
- `render(&mut self, &mut TextureManager, &ScreenData, &mut EventManager, &PlayerData)` — required. Draws and handles input.
- Provided: `get_pos`, `get_scale`, `get_preffered_scale`, `get_id`, `set_buffers`, `set_parent_pos`, `mouse_on`.

## `WidgetType` enum (`widget.rs`)

Concrete widgets can't be stored heterogeneously, so every one is wrapped in the
`WidgetType` enum. Containers hold `Vec<WidgetType>` (or similar) children.

`WidgetType` itself implements `Widget` — dispatch is the `widget_match!` macro,
which matches every variant and forwards the call. **When you add a new widget
type you must add it to both the `WidgetType` enum and the `widget_match!`
macro**, or it won't compile / won't render.

Extra `WidgetType` helpers: `new_panel`, `find_with_id` (recursive id lookup
through container variants), `as_scripting_widget`, `extract_body_widget`.

### Gotcha: recursive type
`PlayWorldViewRender` is a `WidgetType` variant **and** holds a widget for its
overlay. A struct that is itself a `WidgetType` variant cannot hold a bare
`WidgetType` field (infinite size) — box it: `Box<WidgetType>`.

## `WidgetProperties` (`widget_properties.rs`)

Shared layout/render state every widget owns. Key fields:

- `parent_pos` / `parent_scale` — the parent's rectangle, set before layout.
- `external_buffers` / `internal_buffers` — margin (outside) and padding (inside), `[f32; 4]`.
- `pos` `[f32; 4]` / `scale` `[f32; 2]` — this widget's final rectangle.
- `prefered_scale` — size the widget wants; computed in `size()`.
- `bounds` — optional clip rect set by the parent.
- `id: WidgetId` — process-unique, from an atomic counter (`WidgetId::get_next_id()`).

Positions/scales are in NDC. `scale_based_off_parent()` derives `pos`/`scale`
from `parent_pos` + `external_buffers`.

## `Panel` — the main container (`panel/panel.rs`)

Holds children as `Vec<PanelSection>`. Lays them out along a **stretch axis**;
the **cross axis** is controlled by `alignment`.

- `PanelOrientation`: `Vertical` (stretch = y, cross = x) or `Horizontal` (stretch = x, cross = y).
- `PanelAlignment`: `TopLeft`, `BotRight`, `Center`, `Fill` — applies to the cross axis only.
  - e.g. a `Vertical` + `Center` panel stacks children top-to-bottom, centered horizontally.
- Constructors: `new(parent_pos, buffers)`, `new_blank()`, `new_with_parent_props(&WidgetProperties)`.
- `set_orientation(orientation, alignment)`, `set_color(PanelColor)`.
- `add_text_display(String) -> &mut TextDisplay` (chain `.set_text_scale(TextSize)`); other `add_*` builders add children.
- `wrap_into_widget() -> WidgetType`.

Most concrete widgets follow the same `wrap_into_widget()` convention.

## Layout / render flow

Per frame the parent sets a child's `parent_pos`, calls `size()` (bottom-up:
children size first, parents aggregate `prefered_scale`), then `render()`.
`TextSize` lives in `widget_calculations.rs` (`ExtraExtraSmall` … `ExtraLarge`).

## Directory map

- `widget.rs` — `Widget` trait, `WidgetType` enum, `widget_match!`.
- `widget_properties.rs` — `WidgetProperties`, `WidgetId`.
- `widget_calculations.rs` — layout math, `TextSize`, scale constants.
- `panel/` — `Panel` and friends (background, sections, color).
- `tab_panel/`, `scroll_panel/` — other containers.
- `button/`, `bar_button/`, `toggle_button/` — interactive widgets.
- `text/` — `TextDisplay` (header.rs), `TextInput`.
- `drone_programming/` — visual scripting editor (slot widgets per `ScriptElement`).
- `game_object_prop_displays/` — per-entity property panels (e.g. inventory display).
- `window_manager/` — `WidgetWindowManager`, window definitions.
- `world_rendering/` — `PlayWorldViewRender` (the voxel viewport), `ViewMode`, `WorldViewData`.
- `prelude.rs` — common re-exports.

## Adding a new widget

1. Create the struct; implement `Widget` (`size`, `render`, the two property accessors).
2. Add a `wrap_into_widget()` convenience method.
3. Add a variant to `WidgetType` **and** a line to `widget_match!` in `widget.rs`.
