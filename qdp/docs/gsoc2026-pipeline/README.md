# GSoC 2026 — QDP Pipeline

**Scope:** `qdp-core`, `qdp-python`, `qdp-kernels`

**Pipeline today:** read data (synthetic / file / streaming Parquet) → CPU prefetch → GPU `encode_batch` → yield `QuantumTensor` to Python (`QuantumDataLoader`)

**Prerequisite:** [#1310](https://github.com/apache/mahout/issues/1310) (single-sample f32 on `QuantumEncoder` trait) — confirm merged before starting

**Child specs:** [issues/](issues/) · **Tracker:** [issues/README.md](issues/README.md)

---

## Track A — Parquet / reader f32

Make file/Parquet sources honor `dtype=f32` end-to-end (no forced f64 widening).

| ID | Issue | What to do |
|----|-------|------------|
| A1 | [[Refactor] Add FloatElem trait and DataReader #1339](https://github.com/apache/mahout/issues/1339) | Generalize `DataReader::read_batch` from `Vec<f64>` to `Vec<T>` in `reader.rs` |
| A2 | [[Feature] ParquetReader: f32/f64 columns + Arrow cast #1340](https://github.com/apache/mahout/issues/1340) | Accept f32 Parquet columns; Arrow cast when dtypes differ |
| A3 | [[Feature] Pipeline file load respects PipelineConfig.dtype #1341](https://github.com/apache/mahout/issues/1341) | Wire A2 into `pipeline_runner` so `dtype=f32` reaches f32 kernels |
| A4 | [[Testing] Parquet f32 tests and benchmark #1342](https://github.com/apache/mahout/issues/1342) | Fidelity tests + optional throughput benchmark |

**Order:** A1 → A2 → A3 → A4

---

## Track B — GPU memory estimate

Early OOM detection: fail at loader build, not after 90 min of running.

| ID | Issue | What to do |
|----|-------|------------|
| B1 | `[pipeline][gsoc] Add estimate_memory() for GPU footprint` | Pure function estimating CPU + GPU memory from config params |
| B2 | `[pipeline][gsoc] Fail fast when estimated memory exceeds VRAM` | Call B1 at loader construction; return actionable error |

---

## Track C — Preprocessor Python API

Expose `Preprocessor` helpers that already exist in Rust to Python users.

| ID | Issue | What to do |
|----|-------|------------|
| C1 | `[pipeline][gsoc] PyO3 bindings for Preprocessor` | Bind `validate_input`, `calculate_l2_norm` etc. via PyO3 |
| C2 | `[pipeline][gsoc] Document Preprocessor in qumat_qdp public API` | Export from `__init__.py`, add docstrings |

---

## Track D — Kernel dedup

Collapse ~270 duplicate f32/f64 stubs in `qdp-kernels`.

| ID | Issue | What to do |
|----|-------|------------|
| D1 | `[kernels][gsoc] KernelElem trait to deduplicate f32/f64 stubs` | Add `KernelElem` trait + `define_stub!` macro to replace copy-paste stubs |

---

## Track E — pipeline_runner structure

Unify three `PipelineIterator` constructors and three producers in `pipeline_runner.rs`.

| ID | Issue | What to do |
|----|-------|------------|
| E1 | `[pipeline][gsoc] Unify PipelineIterator constructors with Source enum` | 3 `new_*` methods → 1 `PipelineIterator::new(source, config)` |
| E2 | `[pipeline][gsoc] Unify batch producers via AdapterBackedProducer` | 3 producers with 80% identical code → `AdapterBackedProducer` |
| E3 | `[pipeline][gsoc] StreamingProducer: use VecDeque for O(1) buffer advance` | Replace `Vec::drain` O(n) with `VecDeque` |

**Order:** E1 → E2 → E3 (one PR at a time on `pipeline_runner.rs`; start E after A3 merged)

---

## Overall order

`#1310` → **A1→A2→A3→A4** → parallel **B1→B2**, **C1→C2**, **D1** → **E1→E2→E3**

## Schedule (~12 weeks, solo)

W1–3 A1–A2 · W4–6 A3–A4+B · W7 C · W8–11 E (+D1 stretch) · W12 polish

---

## Done when

- [ ] `QuantumDataLoader(...).dtype("f32").source_file("x.parquet")` uses f32 kernels end-to-end
- [ ] Loader build fails early with clear message when VRAM insufficient (partial [#1262](https://github.com/apache/mahout/issues/1262) — [Feature] Replace static MAX_QUBITS with runtime capacity policy in QDP)
- [ ] `Preprocessor` exposed and documented in Python
- [ ] Producers / constructors deduplicated; CI green; ≤2% throughput regression
