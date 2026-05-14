---
title: Core Concepts - Qumat
---

# Core Concepts

## Backend Abstraction

QuMat provides a **unified Python API** that works across multiple quantum computing backends:

| Backend | Module | Simulator Types |
|---------|--------|-----------------|
| Qiskit | `qiskit_backend.py` | `aer`, `statevector` |
| Cirq | `cirq_backend.py` | `density_matrix`, `statevector` |
| Amazon Braket | `amazon_braket_backend.py` | `default`, `sv1` |

You configure the backend once at initialization; all subsequent circuit operations use the same interface regardless of the backend:

```python
from qumat import QuMat

config = {
    "backend_name": "qiskit",          # or "cirq", "amazon_braket"
    "backend_options": {
        "simulator_type": "aer",
        "shots": 1024,
    },
}
qc = QuMat(config)
```

## Circuits and Gates

A quantum circuit in QuMat is created with a fixed number of qubits, then gates are applied sequentially:

```python
qc.create_empty_circuit(num_qubits=3)

# Single-qubit gates
qc.apply_hadamard_gate(qubit=0)
qc.apply_pauli_x_gate(qubit=1)

# Two-qubit gates
qc.apply_cnot_gate(control=0, target=1)

# Parameterized rotation gates
qc.apply_rotation_gate(qubit=0, axis="rx", angle=3.14159)
```

## Measurement

After building a circuit, call `execute_circuit()` to run the simulation and get measurement results as a dictionary of bitstrings to counts:

```python
result = qc.execute_circuit()
# result = {'000': 250, '001': 0, '010': 512, ...}
```

## QDP Integration

For GPU-accelerated quantum state encoding, QuMat integrates with the **QDP** (Quantum Data Plane) subsystem. See the [QDP documentation](/docs/qdp/) for details on:

- `QdpEngine` — GPU encoder for classical-to-quantum data encoding
- `QuantumTensor` — DLPack-compatible tensor for zero-copy PyTorch integration
- `QuantumDataLoader` — Batch iterator for training loops
