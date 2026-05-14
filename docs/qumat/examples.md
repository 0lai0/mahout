---
title: Examples - Qumat
---

# Examples

## Example 1: Create a Quantum Circuit and Measure

```python
from qumat import QuMat

# Initialize with the Qiskit backend
config = {
    "backend_name": "qiskit",
    "backend_options": {
        "simulator_type": "aer",
        "shots": 1024,
    },
}
qc = QuMat(config)

# Create a 2-qubit circuit
qc.create_empty_circuit(2)

# Apply gates: create a Bell state |Φ+⟩ = (|00⟩ + |11⟩) / √2
qc.apply_hadamard_gate(0)
qc.apply_cnot_gate(0, 1)

# Measure and get results
result = qc.execute_circuit()
print(result)  # e.g. {'00': 512, '11': 512}
```

## Example 2: Use a Different Backend

QuMat provides a unified API across multiple quantum backends:

```python
from qumat import QuMat

# Same circuit code, different backend
config_cirq = {
    "backend_name": "cirq",
    "backend_options": {
        "simulator_type": "density_matrix",
        "shots": 1024,
    },
}
qc = QuMat(config_cirq)
qc.create_empty_circuit(2)
qc.apply_hadamard_gate(0)
qc.apply_cnot_gate(0, 1)

result = qc.execute_circuit()
print(result)
```

## Example 3: Parameterized Rotation Gates

```python
import math
from qumat import QuMat

config = {
    "backend_name": "qiskit",
    "backend_options": {"simulator_type": "aer", "shots": 1024},
}
qc = QuMat(config)
qc.create_empty_circuit(1)

# Rotate around X axis by π/4
qc.apply_rotation_gate(0, "rx", math.pi / 4)

result = qc.execute_circuit()
print(result)
```
