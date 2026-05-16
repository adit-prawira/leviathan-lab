# Phase 5: Spikes & Fins

**Ship:** The monster starts looking like a real creature. Add dorsal fins, tentacle beards, rows of teeth.

---

## Slice 5.1: Appendage data types

- `Appendage` component with fields: `appendage_type` (enum), `surface_pos` (vec3 local), `size` (f32)
- Appendage type enum: `Fin`, `Spike`, `Tentacle`, `Eye`, `Tooth`
- Each type maps to a procedural mesh or a loaded `.glb` asset
- Appendage entity tagged with `Appendage` + `BodyPart` (or separate tag)

**Accept:** Each appendage type has a distinct visual shape. Types are swappable.

---

## Slice 5.2: Surface placement

- Tool mode: "Place Appendage" (press P or toolbar)
- Click on a body part surface → raycast hits mesh surface
- Appendage spawns at hit point, aligned to surface normal
- Size defaults to 0.3 units, can be scaled later
- Multiple appendages can stack on same body part

**Accept:** Click on body → spike/fin/eye appears on the clicked spot, facing outward from surface.

---

## Slice 5.3: Parent follow

- Appendage is child of the body part it was placed on
- When body part moves/rotates/scales, appendage follows
- World position recalculated from local offset on parent

**Accept:** Move body part → attached appendages move with it. Reparent body → appendages still stick.

---

## Slice 5.4: Appendage gallery UI

- UI panel with scrollable grid of appendage types
- Each type shown with name + small preview icon
- Click to select active appendage type for placement
- Currently selected type highlighted

**Accept:** Gallery shows all 5 appendage types. Click a type → Place tool uses that type.

---

## Slice 5.5: Appendage transform editing

- Select an appendage → gizmo can move it along the body surface
- Surface constraint: movement stays on parent mesh surface
- Size slider in properties panel
- Rotation: appendage can spin around surface normal

**Accept:** Select appendage, slide it along body surface. Scale it. Spin it.

---

## Slice 5.6: Appendage removal

- Select appendage, press Delete → removes appendage only (not parent body)
- Can't select body through appendage (appendage has priority if clicked)
- Undoable

**Accept:** Delete an appendage → body part stays, appendage gone. Undo brings it back.

---

## Learning targets

- Procedural mesh generation (cone for spike, plane for fin, tube for tentacle)
- Surface normal calculation and alignment
- Ray-mesh hit with barycentric coordinates for surface position
- Local vs world space for parented entities
- UI scrollable grids / galleries

## Dependencies

| Crate | Why |
|-------|-----|
| `bevy` | Engine |
| `bevy_egui` | Gallery UI |
