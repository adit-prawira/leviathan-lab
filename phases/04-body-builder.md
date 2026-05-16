# Phase 4: Body Builder

**Ship:** True creature creation. Not just editing a fixed shape — you build the whole body.

---

## Slice 4.1: Add part tool

- Tool mode: "Add Part" (press A or click toolbar button)
- On click, a new body part spawns at cursor position in 3D space
- Part type chooser: Sphere or Capsule (dropdown or toggle)
- New part attaches to selected part's hierarchy (or root if nothing selected)
- New part gets default transform and material

**Accept:** Click "Add Part" → new sphere/capsule appears. It's part of the monster. Can be selected and moved.

---

## Slice 4.2: Delete part

- Select a part, press Delete
- Removes part and all its children
- Can't delete the last remaining part (monster must have at least 1)
- Undoable (Phase 3 undo stack)

**Accept:** Delete a part → it and its children vanish. Delete last part → nothing happens.

---

## Slice 4.3: Reparent drag

- In properties panel, show hierarchy tree
- Drag a part name onto another part name → reparents
- New parent transform computed so part doesn't visually jump (maintain world position)
- Can't reparent a part to itself or its own child

**Accept:** Drag tail onto body → tail becomes child of body. No visual pop. Undo works.

---

## Slice 4.4: Capsule resize

- When capsule is selected, show length/radius controls in properties panel
- Slider for radius (min 0.1, max 2.0)
- Slider for length (min 0.2, max 5.0)
- Updates capsule mesh scale in real time

**Accept:** Select a capsule, drag radius slider → capsule thickens. Drag length → stretches.

---

## Slice 4.5: Part palette

- UI panel listing all available body part types
- Visual preview (small icon or name)
- Click to select part type for Add tool
- Show current part type highlighted

**Accept:** Palette shows Sphere and Capsule. Click Sphere → Add tool places spheres. Click Capsule → places capsules.

---

## Slice 4.6: Validation

- Minimum 1 body part enforced
- No circular parent-child chains
- New parts get unique names: "Body", "Body.001", "Body.002" etc.
- Empty selection after delete → auto-select nearest remaining part

**Accept:** System handles edge cases gracefully. No crashes on edge inputs.

---

## Learning targets

- Dynamic entity spawning / despawning in Bevy
- Tree manipulation: reparenting with world-transform preservation
- Mesh scaling for procedural shapes (capsule stretch)
- Named entity management
- Tool mode state machine
- Input ray-plane intersection for part placement

## Dependencies

| Crate | Why |
|-------|-----|
| `bevy` | Engine |
| `bevy_egui` | Palette & inspector UI |
