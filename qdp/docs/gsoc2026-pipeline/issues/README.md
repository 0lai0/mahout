# Issue index

Hub: [../README.md](../README.md)

| ID | GitHub | Title | Depends |
|----|--------|-------|---------|
| A1 | [#1339](https://github.com/apache/mahout/issues/1339) | [Refactor] Add FloatElem trait and DataReader<T> | — |
| A2 | [#1340](https://github.com/apache/mahout/issues/1340) | [Feature] ParquetReader: f32/f64 columns + Arrow cast | A1 |
| A3 | [#1341](https://github.com/apache/mahout/issues/1341) | [Feature] Pipeline file load respects PipelineConfig.dtype | A2 |
| A4 | [#1342](https://github.com/apache/mahout/issues/1342) | [Testing] Parquet f32 tests and benchmark | A3 |
| B1 | | `[pipeline][gsoc] Add estimate_memory() for GPU footprint` | — |
| B2 | | `[pipeline][gsoc] Fail fast when estimated memory exceeds VRAM` | B1 |
| C1 | | `[pipeline][gsoc] PyO3 bindings for Preprocessor` | — |
| C2 | | `[pipeline][gsoc] Document Preprocessor in qumat_qdp public API` | C1 |
| D1 | | `[kernels][gsoc] KernelElem trait to deduplicate f32/f64 stubs` | — |
| E1 | | `[pipeline][gsoc] Unify PipelineIterator constructors with Source enum` | A3† |
| E2 | | `[pipeline][gsoc] Unify batch producers via AdapterBackedProducer` | E1 |
| E3 | | `[pipeline][gsoc] StreamingProducer: use VecDeque for O(1) buffer advance` | E2 |
