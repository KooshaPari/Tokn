# ADR-002: Plugin System with WASM Isolation

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

We need an extensible plugin system that allows custom functionality without risking the core system. Plugins must be able to extend storage backends, audit logging, rate limiting, and custom claim validation.

Traditional approaches:
- **Dynamic library loading (dylib)** - Dangerous, crashes affect core
- **IPC/RPC** - Latency overhead, complex setup
- **Scripting languages** - Security concerns, limited capability

---

## Decision

We will use **WebAssembly (WASM)** as the plugin isolation mechanism.

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Plugin System Architecture                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                           Tokn Core                                     │ │
│  │                                                                        │ │
│  │  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐   │ │
│  │  │  Storage Plugin  │  │   Audit Plugin   │  │  RateLimit Plugin │   │ │
│  │  │   Interface      │  │    Interface     │  │    Interface      │   │ │
│  │  └────────┬─────────┘  └────────┬─────────┘  └────────┬─────────┘   │ │
│  │           │                     │                     │              │ │
│  │           └─────────────────────┼─────────────────────┘              │ │
│  │                                 │                                      │ │
│  │                    ┌────────────▼────────────┐                        │ │
│  │                    │     Plugin Host (WASM)   │                        │ │
│  │                    │   ┌─────────────────┐   │                        │ │
│  │                    │   │  WASM Runtime   │   │                        │ │
│  │                    │   │  (Wasmtime)     │   │                        │ │
│  │                    │   └─────────────────┘   │                        │ │
│  │                    └─────────────────────────┘                        │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                        Plugin Sandbox                                   │ │
│  │                                                                        │ │
│  │   ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐               │ │
│  │   │Storage  │  │ Audit   │  │ Rate    │  │Custom   │               │ │
│  │   │Plugin   │  │ Plugin  │  │Limit    │  │Claims   │               │ │
│  │   │.wasm    │  │ .wasm   │  │Plugin   │  │Plugin   │               │ │
│  │   │         │  │         │  │ .wasm   │  │ .wasm   │               │ │
│  │   └─────────┘  └─────────┘  └─────────┘  └─────────┘               │ │
│  │                                                                        │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Plugin Interface Definition

```rust
#[wasmer::import_module]
pub trait StoragePlugin {
    fn store(&mut self, key: &[u8], value: &[u8]) -> Result<(), PluginError>;
    fn retrieve(&mut self, key: &[u8]) -> Result<Vec<u8>, PluginError>;
    fn delete(&mut self, key: &[u8]) -> Result<(), PluginError>;
    fn list(&mut self, prefix: &[u8]) -> Result<Vec<Vec<u8>>, PluginError>;
}

#[wasmer::import_module]
pub trait AuditPlugin {
    fn log_event(&mut self, event: AuditEvent) -> Result<(), PluginError>;
    fn query(&mut self, filter: AuditFilter) -> Result<Vec<AuditEntry>, PluginError>;
}
```

### Rationale

| Aspect | WASM | Native Plugins | RPC | Scripting |
|--------|------|----------------|-----|-----------|
| **Isolation** | ✅ Strong | ❌ None | ✅ Strong | ⚠️ Limited |
| **Performance** | ✅ Near-native | ✅ Native | ❌ Overhead | ⚠️ Slow |
| **Security** | ✅ Sandboxed | ❌ Dangerous | ✅ Sandboxed | ❌ Risky |
| **Portability** | ✅ Cross-platform | ❌ Platform-specific | ✅ Any | ✅ Any |
| **Complexity** | Medium | Low | High | Low |
| **Hot Reload** | ✅ Yes | ⚠️ Complex | ✅ Yes | ✅ Yes |

### Plugin Capabilities

1. **Storage Plugins** - Custom storage backends
   - PostgreSQL extensions
   - Redis modules
   - S3-compatible storage
   - Distributed filesystems

2. **Audit Plugins** - Custom audit trails
   - SIEM integration
   - Blockchain anchoring
   - Distributed logging

3. **Rate Limiting Plugins** - Custom rate policies
   - Token bucket algorithms
   - Sliding window counters
   - Distributed rate limiting

4. **Custom Claims Plugins** - Claim transformation
   - Identity verification
   - Age verification
   - Custom claim enrichment

---

## Consequences

### Positive
- Strong isolation prevents plugin crashes from affecting core
- Memory safety guarantees prevent buffer overflows
- Cross-platform plugin compatibility
- Hot module replacement without restart
- Sandboxed execution limits attack surface

### Negative
- WASM runtime adds ~5MB to binary size
- Some restrictions on what plugins can do (no native threads)
- Debugging WASM plugins more complex
- Learning curve for plugin developers

### Mitigation
- Provide rich SDK for common languages (Rust, Go, Python)
- Comprehensive plugin testing framework
- Performance profiling tools
- Detailed documentation and examples

---

## References

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [Wasmtime - WebAssembly Runtime](https://github.com/bytecodealliance/wasmtime)
- [wasmer - Universal WebAssembly Runtime](https://github.com/wasmerio/wasmer)
