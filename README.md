# CIR (Confidential Inference Runtime)

## ✅ STATUS: DEPLOYED TO PRODUCTION

**Current Environment:** DigitalOcean Cloud (Ubuntu 24.04)  
**Deployment Date:** February 9, 2026  
**Next Milestone:** Azure AMD SEV-SNP migration for hardware TEE attestation

---

## What's Working NOW:

✅ **Constant-time matrix multiplication** (Rust implementation)  
✅ **Deployed to production cloud infrastructure**  
✅ **Cryptographic attestation generation** (SHA-256 fingerprinting)  
✅ **Attestation report export** (JSON format for verification)  
✅ **16ms execution time** (200×200 matrix, data-independent)  
✅ **Remote server compilation and execution** (proven portability)

---

## Week 5 Achievement:

**Built, deployed, and verified CIR in production within 4 weeks.**

- Week 1-4: Local development and constant-time verification
- Week 5: Cloud deployment and production testing ✅
- Week 6: Azure SEV-SNP integration (hardware attestation)
- Week 7: First enterprise pitch

---

## Technical Proof:
```
--- CIR PoC: Secure Inference Engine (200x200) ---

[Step 1] Running Constant-Time Calculation...
   > Done in 16ms

[Step 2] Generating Cryptographic Fingerprint...
   > Result Hash: 704dc3569d50486d5b01f77aac85e961320ed4bf33cd611d555cc513b5cdc96a

[Step 3] Exporting Attestation Report...
   > SAVED: 'attestation_report.json' created successfully.
```

**Production deployment confirmed: February 9, 2026**

---



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
