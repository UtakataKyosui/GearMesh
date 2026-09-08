# State Slots (Design Proposal)

**Status**: Proposal — not implemented. This document records the design decision
for how GearMesh should define *state values* shared between Rust and the UI,
before any code is written.

**Scope**: Applies to desktop (Tauri) and client/server (axum) setups alike. The
IR described here is transport-agnostic; transports fill in the read/write/notify
primitives.

---

## 1. Motivation

GearMesh currently shares one kind of knowledge across the Rust/TypeScript
boundary: **the shape of a value**. A state value that Rust owns needs four:

| Knowledge | Where it lives today |
|-----------|----------------------|
| The value's type | Rust type — the only piece GearMesh carries |
| How to read it | `invoke("get_theme")` — a string written on both sides |
| How change is announced | `emit("theme-changed")` / `listen("theme-changed")` — a string written on both sides |
| How to request a write | `invoke("set_theme", { theme })` — name *and* argument key written on both sides |

Three of the four are duplicated by hand, and none of the three is checked by a
compiler. Renaming an event silently breaks the UI at runtime.

This is the same failure the DRY principle names — one piece of knowledge with
more than one authoritative representation — applied to the wiring rather than to
the value. GearMesh closes half the gap; state slots close the rest.

## 2. Principle: ownership follows responsibility

The placement rule this design follows: **a state value is defined where the
responsibility for its meaning lives** — that is, wherever the logic that decides
what must happen when it changes lives.

- Rust owns it → the definition is a Rust state slot, and the UI only projects it.
- The UI owns it → it never enters GearMesh at all (see §7).

The corollary matters as much as the rule: once Rust owns a value, the UI must
not keep a second authoritative copy of it. It reads through, renders, and asks
Rust to change it. Data flows one way:

```
Rust state ──(event / read)──> UI projection ──(write request)──> Rust state
```

The write request is not a UI-side update; it is a message to the owner, whose
result comes back through the same one-way read path.

## 3. The unit of definition is a slot, not a type

The first real decision: does a state definition attach to the *type* of the
value, or to a separate *slot*?

**Rejected — attaching to the type:**

```rust
// Forces type == state, one to one
#[derive(GearMesh)]
#[gear_mesh(state(get = "get_theme", event = "theme://changed"))]
pub enum Theme { Light, Dark, System }
```

This is the cheapest option — it reuses the existing derive and `inventory`
pipeline unchanged — but it conflates two different concepts. A type is a
*reusable vocabulary*: `Theme` may appear as a state value, as a command
argument, and as a field of a persisted config. A state value is a *slot* the
application has exactly one of. Binding them together means `String` can back at
most one state value, which is untenable.

**Chosen — slots declared on the state container:**

```rust
#[derive(GearMeshState)]
pub struct AppState {
    /// The theme the user selected.
    #[state(set = "set_theme")]          // read + notify derived from the key
    theme: Theme,

    /// The theme actually rendered, after resolving the OS setting.
    #[state(readonly, derived_from = "theme")]
    effective_theme: ResolvedTheme,
}
```

Why the container:

- It matches the runtime shape Tauri already has (`Mutex<AppState>` behind
  `State`), so the declaration sits on the thing that really is the state.
- Value types stay reusable; `Theme` keeps its `#[derive(GearMesh)]` and nothing
  more.
- Every state value the application owns is enumerable in one place. "Rust is the
  source of truth" becomes visible in the source layout, not just in prose.

**Escape hatch:** state that does not live in a single container (several
independent `State<T>` registrations, values computed on demand) is declared with
a standalone macro:

```rust
gear_mesh::states! {
    window_geometry: WindowGeometry => { get: "get_window_geometry", readonly },
}
```

### Names derive from one key

The slot key is the single authority for naming. `theme` yields
`state://theme/get` and `state://theme/changed` by default; explicit `get`,
`set`, and `event` values exist only to adopt commands that already exist. A
project that starts from GearMesh writes each name zero times.

## 4. Granularity: one slot per value

The alternative — one event carrying the whole `AppState` — is cheaper to
implement and wrong for the same reason: the UI then has to diff the payload to
find out what changed, and *which fields matter to which view* becomes knowledge
maintained in a second place.

Per-value slots multiply commands and events, but every one of them is generated,
so the cost is not paid by a human. Subscribers wake only for the value they use,
so re-rendering stays minimal.

Group fields into one slot only when they **change together and are read
together** — `WindowGeometry { x, y, w, h }` is one slot; `theme` and `locale`
are two.

## 5. Generated output: a projection, never a store

The single most important constraint on the generated TypeScript: **it must not
hold the value.** A generated module-level cache would make GearMesh a factory
for exactly the duplicated-state bug this design exists to prevent.

```ts
// generated — holds no state of its own
export const theme = {
  key: "theme",
  get: (): Promise<Stamped<Theme>> => invoke("state://theme/get"),
  subscribe: (cb: (v: Stamped<Theme>) => void) => listen("state://theme/changed", …),
  set: (v: Theme): Promise<void> => invoke("set_theme", { theme: v }),
} as const;
```

Three properties carry the design:

- **`set` returns `Promise<void>`.** Returning the new value would invite
  optimistic local updates, which is how a second source of truth gets created.
  The result arrives through `subscribe`, like every other change.
- **`readonly` is a first-class attribute.** When a slot is read-only no `set` is
  generated at all, so projecting is the only thing the UI *can* do. The
  responsibility boundary is enforced by the emitted types.
- **Ordering is handled in the generated code, not by each caller.** See below.

### Revisions instead of a first-event flag

A UI that reads the initial value while also subscribing has a race: the initial
read can resolve *after* a change event and overwrite a newer value. Guarding
this with a hand-written "have I seen an event yet" flag works, but it must be
written correctly once per state value, and it is skipped sooner or later.

Instead, each slot carries a monotonic revision on the Rust side and both paths
return `Stamped<T> = { rev: number; value: T }`. Consumers keep the highest `rev`
they have seen and drop anything older. Late arrivals become harmless rather than
merely unlikely, and the rule is written once in the generated adapter.

This is the strongest practical argument for making the state value the unit of
definition: correctness knowledge like this can only be centralized if there is
something to centralize it into.

### Framework adapters

For React the right primitive is `useSyncExternalStore`: it reads through to an
external source rather than mirroring it, which is this design's data flow
expressed as an API. `getSnapshot` must be synchronous, so a small cache sits at
the boundary — but it is a *projection cache*, not an authority, and `rev` keeps
it honest. Adapters are opt-in and live behind feature flags; the core generated
binding stays framework-free.

## 6. Derived state is not duplicated state

Duplication means *the same meaning* held authoritatively in two places. Values
with different meanings may coexist freely: `theme` (what the user chose:
light / dark / system) and `effective_theme` (what is actually rendered, after
resolving `system` against the OS) are different values, and both are legitimate.

Two ways to express a derived value, both acceptable:

1. Resolve it in Rust and expose it as a `readonly` slot — right when the
   resolution depends on data Rust owns (OS settings, other slots).
2. Compute it in the UI as a pure function of a projected slot — right when it is
   purely presentational.

What is never acceptable is the same meaning owned on both sides with
synchronization between them.

## 7. Out of scope, deliberately

Modal open/closed, form input buffers, hover state, the active tab — state whose
meaning and change-handling belong entirely to the UI — are **not** GearMesh's
concern and must not be expressible as slots. A tool that offers to define
UI-owned state in Rust encourages the duplication this design is meant to
eliminate. The boundary is a feature and should be documented as one.

## 8. Placement in the codebase

Consistent with the existing layered architecture (see [ARCHITECTURE.md](ARCHITECTURE.md)):

- **`gear-mesh-core`** — a `StateSlot` IR alongside `GearMeshType`: key, `TypeRef`,
  `Access::{ReadOnly, ReadWrite}`, event name, command names, docs. Transport-agnostic,
  serializable, and language-agnostic like the rest of the IR.
- **`gear-mesh-derive`** — `#[derive(GearMeshState)]` and the `#[state(...)]`
  field attribute; registration through `inventory`, as types already are.
- **`gear-mesh-generator`** — `state_gen.rs` emitting the TypeScript projection.
- **Transport adapters** — behind feature flags, supplying `invoke`/`listen` for
  Tauri and HTTP + SSE for the axum case. One IR, two projections; this keeps the
  "language-agnostic IR" principle intact rather than baking Tauri into the core.

## 9. Phased rollout

1. **Read-only projection.** `StateSlot` IR, `readonly` slots, generated
   `get` + `subscribe`, `Stamped<T>` and revisions. One-way projection is
   complete and useful at this point; the phase is a natural stopping point.
2. **Writes.** `set` with `Promise<void>`, argument-key derivation, revision
   bumping on the Rust side.
3. **Framework adapters.** React `useSyncExternalStore` binding first.

## 10. Open questions

- **Command existence checking.** Deriving names is only half the win if a typo in
  an explicit `get = "..."` still fails at runtime. Emitting a reference to the
  command function from the macro would catch it at compile time; whether that is
  workable with `#[tauri::command]`'s expansion needs a spike.
- **Revision storage.** Per-slot counter versus one counter for the whole
  container. Per-slot is more precise; shared is cheaper and still monotonic.
- **Locking.** Slots are declared on a plain struct, but the runtime value is
  behind a `Mutex`. Whether the macro should generate accessors that take the lock,
  or stay out of the way entirely, is unresolved.
- **Persistence.** Slots that must survive a restart introduce a load phase, and
  the initial-read race is exactly where that shows up. Revisions should cover it,
  but it needs to be verified against a real persisted slot.

## 11. References

- [ARCHITECTURE.md](ARCHITECTURE.md) — crate layout and IR design principles
- [Tauriの状態値は責務を持つ側に定義すべき](https://zenn.dev/ayaextech_fill/articles/tauri-reactivity-state-management)
  — the responsibility-based placement rule and the duplicated-state failure mode
  this design is built around
