# Phase 3: Paint the Beast

**Ship:** The monster has personality now. Make it gross, shiny, dark, slimy.

---

## Slice 3.1: Material component

- `BodyMaterial` component with: `base_color` (Color), `roughness` (f32), `metallic` (f32)
- When component changes, update the `StandardMaterial` on the body part's mesh
- Default material per part type (e.g. spheres start pinkish, capsules greenish)

**Accept:** Each body part has editable color, roughness, metallic. Changes reflect instantly.

---

## Slice 3.2: Color picker UI

- Color picker with H/S/V sliders or wheel
- Shows current color of selected part
- Dragging a slider updates the part in real time
- Hex input field for precise color

**Accept:** Select a part, adjust color sliders, part color changes live.

---

## Slice 3.3: Material sliders

- Roughness slider: 0.0 (mirror) to 1.0 (rough)
- Metallic slider: 0.0 (dielectric) to 1.0 (metal)
- Both update in real time on selected part

**Accept:** Slide roughness → part goes from shiny to dull. Slide metallic → part goes from plastic to metallic.

---

## Slice 3.4: Symmetry painting

- Toggle for symmetry mode
- When on, editing a part's material also applies to its mirror counterpart
- Mirror detection: parts with similar position on opposite X side
- Works for color, roughness, metallic

**Accept:** Paint left fin → right fin gets same color. Toggle off → paints only selected.

---

## Slice 3.5: Undo / redo

- `EditHistory` resource: stack of undoable actions
- Each material change is an action: `Action::SetMaterial { entity, old, new }`
- Ctrl+Z: undo last action
- Ctrl+Shift+Z: redo last undone action
- Stack depth: at least 50 actions

**Accept:** Paint a part red. Paint it blue. Ctrl+Z → back to red. Ctrl+Shift+Z → blue again.

---

## Learning targets

- Bevy material system (`StandardMaterial`)
- Reactivity: detect component changes and propagate
- Undo/redo pattern with action stacks
- UI widgets: sliders, color pickers
- Symmetry math: mirroring across axis
- Cloning and diffing ECS components

## Dependencies

| Crate | Why |
|-------|-----|
| `bevy` | Engine |
| `bevy_egui` | UI sliders & color picker |
