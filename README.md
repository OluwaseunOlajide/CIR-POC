# CIR (Confidential Inference Runtime)

Constant-time AI inference with cryptographic attestation for AMD SEV-SNP.

## Current Status (Week 4)

**Working:**
- ✅ Constant-time matrix multiplication (Rust)
- ✅ Timing independence verified (~22ms regardless of input)
- ✅ Attestation report generation (SHA-256 + JSON schema)
- ✅ Verification tool (`cir-verify`)

**Environment:**
Simulation mode on local hardware. Azure SEV-SNP deployment pending Founders Hub approval.

**Next:**
Deploy to Azure DC2as_v5, integrate hardware attestation, extend to additional operations.

## The Gap

Current confidential AI platforms attest to:
- ✅ Memory isolation (TEE is genuine)
- ✅ Code integrity (correct binary running)
- ❌ **Execution behavior** (side-channel resistance)

CIR closes the gap: proof that inference ran with constant-time guarantees.

## Benchmarks

200×200 matrix:
- Random data: 22.1ms ±0.3ms
- Zero data: 21.9ms ±0.2ms
- **Variance: <2% (constant-time verified)**
