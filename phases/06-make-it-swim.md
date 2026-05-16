# Phase 6: Make It Swim

**Ship:** The monster lives. It swims. You can save it and show your friends.

---

## Slice 6.1: Skeleton / joint system

- Each body part (in hierarchy) is a joint with a rest pose
- `Joint` component: rest rotation (relative to parent), current rotation offset
- Joint rotation offset is separate from body part base transform
- Root joint is the base body part (no parent)

**Accept:** Each body part has a joint that can rotate independently of its base transform.

---

## Slice 6.2: FK pose mode

- Tool mode toggle: "Edit" vs "Pose"
- In Pose mode, gizmo rotates joints (no translation/scale)
- Rotating a joint rotates the body part + all its children
- FK chain: rotating tail root ripples through entire tail

**Accept:** Switch to Pose mode. Rotate a joint → body part and children rotate. Looks natural.

---

## Slice 6.3: Idle swim animation

- Idle animation: gentle S-curve wave traveling through body
- Wave starts at head, propagates to tail
- Formula: `rotation = sin(time * frequency + joint_index * phase_shift) * amplitude`
- Parameters: frequency, amplitude, phase shift (tweakable)
- Only active when no user pose override on that joint

**Accept:** Monster gently swims in place. Body undulates like a real swimming creature.

---

## Slice 6.4: Animation blending

- Keyframe data structure: `Keyframe { time, joint_rotations: HashMap<Entity, Quat> }`
- Animation: sequence of keyframes with blend between them
- Blend: spherical linear interpolation (slerp) between keyframes
- Play / pause / stop controls in UI
- Loop toggle

**Accept:** Set a pose at frame 0 and frame 60. Play → smoothly animates between them. Loops.

---

## Slice 6.5: Save monster to JSON

- Serialize entire monster: body hierarchy, part types, transforms, materials, appendages, joints
- Format: JSON or RON (use `serde`)
- File extension: `.leviathan`
- File save dialog: name your monster
- Saves to `assets/monsters/` or a user-chosen path

**Accept:** Build a monster. Save. `.leviathan` file appears on disk. File contains all monster data.

---

## Slice 6.6: Load monster from JSON

- File open dialog or drag-drop `.leviathan` file
- Deserialize and reconstruct all entities, components, hierarchy
- Clears current scene before loading
- Error handling: malformed file shows error dialog, doesn't crash
- Loaded monster is immediately editable

**Accept:** Save a monster. Close app. Open app. Load the file → monster is back, fully editable.

---

## Learning targets

- Serialization with `serde` + JSON
- Forward kinematics joint rotation
- Trigonometric animation (sine wave for swimming)
- Keyframe interpolation and slerp
- File I/O in Rust (save dialog, file handling)
- Entity reconstruction from serialized data

## Dependencies

| Crate | Why |
|-------|-----|
| `bevy` | Engine |
| `serde` | Serialization |
| `serde_json` | JSON format |
| `bevy_mod_picking` or native | File dialog |
