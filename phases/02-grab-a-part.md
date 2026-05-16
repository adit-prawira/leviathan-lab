# Phase 2: Grab a Part

**Ship:** You can poke and prod the monster. Change its shape. Break it. Fix it.

---

## Slice 2.1: Body part data structure

- `BodyPart` component with fields: `name`, `part_type` (Sphere, Capsule)
- `BodyHierarchy` resource that stores parent-child relationships as a tree
- Each body part entity tagged with `BodyPart` component
- Tree serializable for future save/load (Phase 6)

**Accept:** Monster body parts are identifiable entities with names and hierarchy.

---

## Slice 2.2: Raycast picking

- Mouse click fires a ray from camera through cursor position
- Ray hits `BodyPart` mesh → entity is "selected"
- If nothing hit → deselect all
- Selected entity stored in `Selection` resource
- Visual highlight on selected part (emissive glow or outline)

**Accept:** Click a body part → it highlights. Click empty space → deselect.

---

## Slice 2.3: Transform gizmos

- When a part is selected, show 3D manipulator gizmo
- Gizmo handles: translate (axis arrows), rotate (axis rings), scale (axis boxes)
- Drag a handle to modify the part's transform in real time
- Gizmo mode togglable (T = translate, R = rotate, S = scale)
- Gizmo visual: colored by axis (R=X, G=Y, B=Z)

**Accept:** Select a part, drag gizmo handles, part moves/rotates/scales in real time.

---

## Slice 2.4: Properties panel

- Simple UI panel (Bevy UI or `bevy_egui`)
- Shows selected part: name, position (x/y/z), rotation (euler), scale (x/y/z)
- Numeric fields editable (text input or drag-value)
- Updates part transform in real time

**Accept:** Select a part → panel shows its values. Edit a number → part updates.

---

## Slice 2.5: Input mapping

- Left click: select (handled by raycast system)
- T / R / S keys: cycle gizmo mode
- Delete / Backspace: remove selected part (placeholder, full delete in Phase 4)
- Escape: deselect

**Accept:** Keyboard shortcuts work. No overlap with orbit controls.

---

## Learning targets

- Bevy ECS: querying, filtering, components, resources
- Ray-mesh intersection (bevy_mod_raycast or own implementation)
- Gizmo rendering (gizmo systems in Bevy or custom)
- Bevy UI / egui integration
- Transform math: local vs world space
- Input handling: keyboard + mouse

## Dependencies

| Crate | Why |
|-------|-----|
| `bevy` | Engine |
| `bevy_egui` (optional) | UI panel |
| `bevy_mod_raycast` or custom | Ray picking |
