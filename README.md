# RedOx Engine

**RedOx Engine** — A high‑performance modular game engine written in Rust.  
The name *RedOx* symbolises transformation and energy, reflecting the engine’s focus on speed, flexibility, and modern rendering techniques.

## Core Architecture

- **Modular workspace** – The engine is split into independent crates (`redox_math`, `redox_ecs`, `redox_render`, …) with strict one‑way dependencies.
- **Custom ECS (archetype‑based)** – Cache‑friendly entity‑component system designed for zero‑allocation in the hot loop.
- **GPU‑driven rendering** – Future versions will leverage compute shaders and indirect drawing to minimise CPU overhead and maximise frame rates.
- **Event‑driven communication** – Modules interact through a global event system (double‑buffered, thread‑safe), ensuring loose coupling.

## Current Progress

The **mathematical foundation** (`redox_math`) is **complete and fully tested**.  
All geometric primitives, transformation utilities, and culling logic are implemented and covered by integration tests.

## Key Features (Implemented)

- **Optimised 3D Math** – Built on [`glam`](https://crates.io/crates/glam) for zero‑cost abstractions and SIMD acceleration.
- **Geometric Primitives** – Axis‑aligned bounding boxes (`Aabb`), spheres (`Sphere`), and planes (`Plane`) with transformation and containment checks.
- **Advanced Frustum Culling** – Extract view frustum planes from a view‑projection matrix (Gribb‑Hartmann method) and test against AABBs and spheres.
- **100% Test Pass Rate** – Integration tests verify correctness of all mathematical operations and culling logic.

## Tech Stack

| Category       | Libraries / Tools                                                                 |
|----------------|------------------------------------------------------------------------------------|
| Language       | Rust 2024 edition                                                                  |
| Linear Algebra | [`glam`](https://crates.io/crates/glam)                                           |
| Rendering      | [`wgpu`](https://crates.io/crates/wgpu) (planned)                                 |
| Physics        | [`rapier3d`](https://crates.io/crates/rapier3d) (planned)                         |
| Audio          | [`kira`](https://crates.io/crates/kira) (planned)                                 |
| Windowing/Input| [`winit`](https://crates.io/crates/winit) (planned)                               |
| UI / Debug     | [`egui`](https://crates.io/crates/egui) (planned)                                 |
| Parallelism    | [`rayon`](https://crates.io/crates/rayon) (planned)                               |
| Serialization  | [`serde`](https://crates.io/crates/serde) + `ron` / `bincode` (planned)           |

## Development Goal

Sustain **200+ FPS** on target hardware (e.g., NVIDIA RTX 4060 Ti) in complex scenes, achieved through careful CPU/GPU balance and data‑oriented design.

## Status

RedOx Engine is under active development. The first milestone – a complete mathematical core – is finished.  
Beta release is targeted for **autumn 2026**.

## License

This project is licensed under either of [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE), at your option.

---

RedOx Engine