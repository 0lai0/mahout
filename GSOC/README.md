<!--
Licensed to the Apache Software Foundation (ASF) under one or more
contributor license agreements.  See the NOTICE file distributed with
this work for additional information regarding copyright ownership.
The ASF licenses this file to You under the Apache License, Version 2.0
(the "License"); you may not use this file except in compliance with
the License.  You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
-->

# GSoC 2026 Work Product — Apache Mahout (QDP / QuMat)

**One-line pitch:** improve Rust code quality on the QuMat Quantum Data Plane (QDP) pipeline — correct f32 end-to-end file ingest, early GPU memory checks, safer hot-path structure — and raise the website / docs CI bar so those changes stay reviewable.

This branch is the [Google Summer of Code 2026](https://summerofcode.withgoogle.com/) work-product report. The code itself lands on [apache/mahout](https://github.com/apache/mahout) `main` through the pull requests below. The Apache Mahout project README lives on [`main`](https://github.com/apache/mahout/blob/main/README.md).

| | |
|---|---|
| Student | Yu-Chen Lai ([@0lai0](https://github.com/0lai0)) |
| Org / project | Apache Mahout — [GSOC-327](https://issues.apache.org/jira/browse/GSOC-327) |
| Mentors | Rich Huang ([@rich7420](https://github.com/rich7420)), plus reviews from [@ryankert01](https://github.com/ryankert01), [@guan404ming](https://github.com/guan404ming), [@viiccwen](https://github.com/viiccwen), [@400Ping](https://github.com/400Ping) |
| Tracking issue | [#1080](https://github.com/apache/mahout/issues/1080) (original GSoC scope) |
| Roadmap I opened | [#1338](https://github.com/apache/mahout/issues/1338) — QDP pipeline cleanup |
| Code | [apache/mahout](https://github.com/apache/mahout) (`qdp-core`, `qdp-python`, `qdp-kernels`, `docs/`, `website/`) |

QDP today: read data (synthetic / file / streaming Parquet) → CPU prefetch → GPU `encode_batch` → yield `QuantumTensor` to Python (`QuantumDataLoader`).

---

## Merged pull requests

All links are `apache/mahout`. Each line is what the change actually did.

### GSoC core (roadmap #1338)

- [#1343](https://github.com/apache/mahout/pull/1343) — Parameterize `DataReader` / `StreamingDataReader` with a sealed `FloatElem` (`f32` \| `f64`) so file sources are no longer hard-wired to `Vec<f64>` (A1 / #1339).
- [#1393](https://github.com/apache/mahout/pull/1393) — Make `ParquetReader` generic; accept `List<f32|f64>` and `FixedSizeList`, with Arrow `cast` on dtype mismatch and a zero-copy path when types already match (A2 / #1340).
- [#1402](https://github.com/apache/mahout/pull/1402) — Fix `List<T>` readers (`Parquet`, streaming Parquet, Arrow IPC) that treated a null outer row as length 0 and aborted with `Inconsistent sample sizes` (follow-up of #1393, #1401).
- [#1407](https://github.com/apache/mahout/pull/1407) — Wire `PipelineConfig.dtype` through file load so `dtype=f32` Parquet reaches f32 kernels instead of silently widening to f64 (A3 / #1341).
- [#1422](https://github.com/apache/mahout/pull/1422) — End-to-end f32-vs-f64 Parquet fidelity tests (CPU smoke in CI; GPU cases self-skip without CUDA) plus an optional throughput benchmark (A4 / #1342).
- [#1454](https://github.com/apache/mahout/pull/1454) — Add pure `estimate_memory()` (no CUDA) for CPU prefetch pool + GPU state-buffer footprint so callers can size a config before the first batch (B1 / #1429).

### Earlier docs / QDP work (community bonding through early GSoC)

These landed before / alongside the pipeline tracks and are part of the same GSOC-327 theme (Rust quality, tests, website).

- [#956](https://github.com/apache/mahout/pull/956) — Correct the documented command for building the Docusaurus website.
- [#959](https://github.com/apache/mahout/pull/959) — Fix the Apache “How to contribute” URL in the docs.
- [#1011](https://github.com/apache/mahout/pull/1011) — Centralize DLPack tensor cleanup behind `free_dlpack_tensor` (null / missing-deleter checks; related to [#1009](https://github.com/apache/mahout/issues/1009)).
- [#1120](https://github.com/apache/mahout/pull/1120) — Document that `stream_encode` hard-codes `NullHandling::FillZero`.
- [#1124](https://github.com/apache/mahout/pull/1124) — Remove duplicate `cuda_error_to_string` in `gpu/memory.rs` and reuse the crate error mapping.
- [#1136](https://github.com/apache/mahout/pull/1136) — Stop allocating a fresh `Vec` every iteration in `run_throughput_pipeline`.
- [#1154](https://github.com/apache/mahout/pull/1154) — Add root `make benchmark` / `setup-benchmark` so the QDP benchmark path uses the unified `.venv`.
- [#1157](https://github.com/apache/mahout/pull/1157) — Fix the website getting-started URL.
- [#1163](https://github.com/apache/mahout/pull/1163) — Add TypeScript `typecheck` to website CI and fix the typing issues that blocked it.
- [#1200](https://github.com/apache/mahout/pull/1200) — Unit tests for the streaming amplitude encoder.
- [#1203](https://github.com/apache/mahout/pull/1203) — Unit tests for Parquet readers.

---

## Still in review

### [#1462](https://github.com/apache/mahout/pull/1462) — `StreamingProducer`: `VecDeque` for O(1) buffer advance

| | |
|---|---|
| Status | **Open**, under review. [@rich7420](https://github.com/rich7420) LGTM. [@viiccwen](https://github.com/viiccwen) asked for a before/after hot-path benchmark; numbers were posted, then further review comments. |
| Closes | [#1436](https://github.com/apache/mahout/issues/1436) (Track E3) |
| Diff | **+678 / −27, 1 file** (`qdp/qdp-core/src/pipeline_runner.rs`) |

**Why this design.** The streaming producer used a `Vec` plus `buffer_cursor` and reclaimed the consumed prefix with `Vec::drain(..cursor)` once the cursor passed a compaction heuristic — an O(n) memmove of the live tail on the hot path. A `VecDeque` advances the head instead, so discarding a consumed prefix is O(1) amortized, the compaction constant goes away, and output is unchanged.

Batch copies go through `as_slices()` + `extend_from_slice` (not `extend(drain)`), because `Drain` is not `TrustedLen` and would copy element-by-element. Capacity is reserved to the steady-state peak so the ring does not reallocate mid-run. Buffer-only microbench (`make -C qdp bench_streaming_buffer`): ~1.36–1.55× when `batch ≪ chunk`; ~1.0× when batch and chunk are the same size (both do the same work). End-to-end Parquet is ~1.01–1.02× because decode dominates — the merge criteria are O(1) advance and constant capacity, not E2E throughput.

### [#1466](https://github.com/apache/mahout/pull/1466) — Reject pipeline configs that exceed free VRAM

| | |
|---|---|
| Status | **Open**, under review. [@ryankert01](https://github.com/ryankert01) asked for the false-reject vs still-OOM design choice (guard too tight vs too loose). |
| Closes | [#1430](https://github.com/apache/mahout/issues/1430) (Track B2) |
| Diff | **+1,035 / −11, 11 files** (Rust guard + tests, Python `estimate_memory()`, docs, CI) |

**Why this design.** An oversized pipeline currently runs until `encode_batch` allocates GPU buffers, then fails with a message that does not say what to change. After `normalize()`, the three `PipelineIterator` constructors compare `estimate_memory().gpu_state_bytes` to free device memory and reject in milliseconds, before any input file is read.

The budget is **two** concurrent batch state buffers: a `for qt in loader:` loop keeps the previous DLPack tensor alive while the next batch is allocated. That is a **floor, not the true peak** — encoders also `htod_sync_copy` the input batch, so amplitude’s real steady state is nearer 2.5 buffers. A config sitting just under free memory can still OOM; this narrows the window rather than closing it. Modelling the true peak means changing the memory model, which #1430 puts out of scope.

The comparison is against free memory *sampled at that moment* — nothing is reserved, so passing the check is a fast sanity check, not a lease. The CUDA query is skipped when there is no usable device (stub / empty `CUDA_VISIBLE_DEVICES`). CI injects `(free, total)` so the accept/reject rule runs without a GPU. On the Python surface the check runs when **iteration starts**, not when `QuantumDataLoader(...)` returns. `qumat_qdp.estimate_memory()` exposes the same arithmetic without opening a device.

---

## Roadmap I own: [#1338](https://github.com/apache/mahout/issues/1338)

I opened and drive this parent issue: **GSoC 2026 — QDP Pipeline Cleanup**. Scope is `qdp-core`, `qdp-python`, `qdp-kernels`. Prerequisite [#1310](https://github.com/apache/mahout/issues/1310) (single-sample f32 on `QuantumEncoder`) was confirmed before overlapping GPU trait work.

Child issues (all opened by me under #1338):

| Track | Issue | Title | Status |
|-------|-------|--------|--------|
| **A — Parquet / reader f32** | [#1339](https://github.com/apache/mahout/issues/1339) A1 | `FloatElem` + `DataReader<T>` | **Done** — [#1343](https://github.com/apache/mahout/pull/1343) |
| | [#1340](https://github.com/apache/mahout/issues/1340) A2 | `ParquetReader` f32/f64 + Arrow cast | **Done** — [#1393](https://github.com/apache/mahout/pull/1393) |
| | [#1341](https://github.com/apache/mahout/issues/1341) A3 | File load respects `PipelineConfig.dtype` | **Done** — [#1407](https://github.com/apache/mahout/pull/1407) |
| | [#1342](https://github.com/apache/mahout/issues/1342) A4 | f32 Parquet fidelity tests + benchmark | **Done** — [#1422](https://github.com/apache/mahout/pull/1422) |
| *(A follow-up)* | [#1401](https://github.com/apache/mahout/issues/1401) | Null outer `List` rows | **Done** — [#1402](https://github.com/apache/mahout/pull/1402) |
| **B — GPU memory estimate** | [#1429](https://github.com/apache/mahout/issues/1429) B1 | `estimate_memory()` | **Done** — [#1454](https://github.com/apache/mahout/pull/1454) |
| | [#1430](https://github.com/apache/mahout/issues/1430) B2 | Fail fast when estimate exceeds VRAM | **In review** — [#1466](https://github.com/apache/mahout/pull/1466) |
| **C — Preprocessor Python API** | [#1431](https://github.com/apache/mahout/issues/1431) C1 | PyO3 bindings for `Preprocessor` | **Open** — no PR yet |
| | [#1432](https://github.com/apache/mahout/issues/1432) C2 | Document / export in `qumat_qdp` | **Open** — blocked on C1 |
| **D — Kernel dedup** | [#1433](https://github.com/apache/mahout/issues/1433) D1 | `KernelElem` + `define_stub!` for f32/f64 FFI stubs | **Open** — stretch |
| **E — `pipeline_runner` structure** | [#1434](https://github.com/apache/mahout/issues/1434) E1 | Unify constructors with `Source` enum | **Open** — no PR yet |
| | [#1435](https://github.com/apache/mahout/issues/1435) E2 | Unify producers via `AdapterBackedProducer` | **Open** — no PR yet |
| | [#1436](https://github.com/apache/mahout/issues/1436) E3 | `StreamingProducer` `VecDeque` | **In review** — [#1462](https://github.com/apache/mahout/pull/1462) |

**Done when** (from #1338):

- [x] `QuantumDataLoader(...).dtype("f32").source_file("x.parquet")` uses f32 kernels end-to-end (A1–A4 + #1402)
- [ ] Loader build fails early with a clear message when VRAM is insufficient (B2 — #1466 in review; partial [#1262](https://github.com/apache/mahout/issues/1262))
- [ ] `Preprocessor` exposed and documented in Python (C1–C2)
- [ ] Producers / constructors deduplicated; CI green; ≤2% throughput regression (E1–E3; E3 in review)

---

## Not done yet / next direction

**Finish what is already in review**

1. Land [#1466](https://github.com/apache/mahout/pull/1466) (B2) after answering the false-reject vs residual-OOM question: keep the two-buffer floor, document that amplitude’s true peak is nearer 2.5, and do not silently auto-downsize configs.
2. Land [#1462](https://github.com/apache/mahout/pull/1462) (E3) after remaining review comments. Treat it as an O(1) / constant-capacity refactor, not an E2E speedup.

**Still open on #1338**

3. **C1 / C2** ([#1431](https://github.com/apache/mahout/issues/1431), [#1432](https://github.com/apache/mahout/issues/1432)) — bind existing `Preprocessor` helpers (`validate_input`, L2 norms) through PyO3, then export and document them. Bind `f64` only; do not invent `normalize` / `standardize`.
4. **E1 / E2** ([#1434](https://github.com/apache/mahout/issues/1434), [#1435](https://github.com/apache/mahout/issues/1435)) — one `PipelineIterator::new(source, config)` and one `AdapterBackedProducer` instead of three constructors / three ~80%-identical producers. One PR at a time on `pipeline_runner.rs` (E3 is already occupying that file).
5. **D1** ([#1433](https://github.com/apache/mahout/issues/1433)) — collapse copy-paste f32/f64 kernel stubs behind `KernelElem`. Stretch; must not change CUDA kernel behavior or public FFI names.

**Follow-ups called out in merged / open PRs (not in the 12-issue table)**

6. Native f32 readers for Arrow IPC / NumPy / PyTorch / TensorFlow — #1407 still reads those as f64 and warns on narrowing.
7. Tighten the VRAM model (input-batch upload + encoder temporaries) so the guard’s peak matches reality; dedicated Python exception type instead of a generic `RuntimeError`.
8. Remainder of [#1009](https://github.com/apache/mahout/issues/1009) / [#1080](https://github.com/apache/mahout/issues/1080): narrower `unsafe`, rustdoc coverage, `cargo doc` in CI. A draft of that last piece lives on this fork as [0lai0/mahout#1](https://github.com/0lai0/mahout/pull/1) and is not an apache/mahout PR.

**Explicitly out of scope for this GSoC** (from #1338): epoch / shuffle / multi-source loaders, multi-GPU accounting, full `EncodingPlan` + QPU lowering.

---

## How to inspect the work

```bash
git clone https://github.com/apache/mahout.git
cd mahout
# merged pipeline work is on main; open PRs are linked above
```

QDP setup, tests, and benchmarks: [qdp/DEVELOPMENT.md](https://github.com/apache/mahout/blob/main/qdp/DEVELOPMENT.md). Website: [website/README.md](https://github.com/apache/mahout/blob/main/website/README.md).
