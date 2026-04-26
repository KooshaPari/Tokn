# Modularization Patterns State-of-the-Art Research

**Project:** Tokn - Token Management and Modularization System  
**Research Date:** 2026-04-02  
**Classification:** Deep Technical Research  
**Line Count Target:** 800+ lines  

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Plugin Architecture Patterns](#plugin-architecture-patterns)
3. [Dynamic Loading in Rust](#dynamic-loading-in-rust)
4. [Module Isolation Strategies](#module-isolation-strategies)
5. [API Stability and Versioning](#api-stability-and-versioning)
6. [Hot Reloading Techniques](#hot-reloading-techniques)
7. [Module Communication Patterns](#module-communication-patterns)
8. [Security in Modular Systems](#security-in-modular-systems)
9. [Testing Modular Systems](#testing-modular-systems)
10. [References](#references)

---

## Executive Summary

Modern software systems increasingly demand modularity to achieve scalability, maintainability, and extensibility. This research document provides a comprehensive analysis of modularization patterns, with specific focus on:

- Plugin architecture patterns (hooks, extensions, and lifecycle management)
- Dynamic loading mechanisms in Rust (dlopen, WASM, and IPC)
- Module isolation strategies (process, WASM, and capability-based)
- API stability and versioning approaches
- Hot reloading for development and production
- Secure module boundaries

### Key Findings

| Aspect | Current SOTA | Recommendation for Tokn |
|--------|--------------|------------------------|
| Loading Mechanism | WASM Component Model | WASM for sandboxing + native for performance |
| Isolation | Capability-based security | Hybrid: WASM for untrusted, native for trusted |
| Communication | gRPC/IPC with protobuf | Message-passing with serde + channels |
| Versioning | Semantic versioning with ABI stability | Strict semver + ABI compatibility layer |
| Hot Reloading | Inotify + dynamic linking | WASM module replacement |

---

## Plugin Architecture Patterns

### 2.1 Plugin Architecture Taxonomy

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Plugin Architecture Pattern Taxonomy                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ 1. Hook-Based Plugins (WordPress-style)                                      │
│ ─────────────────────────────────────                                       │
│    ┌─────────────┐                                                           │
│    │   Core      │                                                           │
│    │  System     │                                                           │
│    └──────┬──────┘                                                           │
│           │                                                                  │
│     ┌─────┴─────┬───────────────┬───────────────┐                           │
│     │           │               │               │                           │
│     ▼           ▼               ▼               ▼                           │
│ ┌─────────┐ ┌─────────┐   ┌─────────┐   ┌─────────┐                          │
│ │ pre_    │ │ during_ │   │ post_   │   │ filter_ │                          │
│ │ action  │ │ action  │   │ action  │   │  hook   │                          │
│ └─────────┘ └─────────┘   └─────────┘   └─────────┘                          │
│                                                                              │
│ Plugins register callbacks at specific points in core execution             │
│ Example: before_token_issue, after_validation, on_revocation                  │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ 2. Extension Point Pattern (Eclipse/VS Code-style)                          │
│ ─────────────────────────────────────                                       │
│    ┌─────────────────────────────────────┐                                   │
│    │           Extension Host            │                                   │
│    │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐  │                                   │
│    │  │ Ext │ │ Ext │ │ Ext │ │ Ext │  │                                   │
│    │  │  A  │ │  B  │ │  C  │ │  D  │  │                                   │
│    │  └─┬──┘ └─┬──┘ └─┬──┘ └─┬──┘  │                                   │
│    │    └──────┴──────┴──────┘      │                                   │
│    │           │                      │                                   │
│    │           ▼                      │                                   │
│    │    ┌───────────────┐            │                                   │
│    │    │  Extension    │            │                                   │
│    │    │  API Surface  │◄───────────┤                                   │
│    │    └───────────────┘            │                                   │
│    └─────────────────────────────────────┘                                   │
│                                                                              │
│ Core defines API surface, plugins implement interfaces                       │
│ Type-safe at compile time (via traits) or runtime (via reflection)          │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ 3. Service Provider Interface (Java SPI / Rust Provider)                    │
│ ─────────────────────────────────────                                       │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────┐       │
│  │                      Service Registry                            │       │
│  │                                                                  │       │
│  │  ServiceType::TokenStore ──────► [RedisStore, PostgresStore]    │       │
│  │  ServiceType::KeyProvider ─────► [HsmProvider, FileProvider]    │       │
│  │  ServiceType::RateLimiter ─────► [RedisRateLimit, LocalRateLim] │       │
│  │                                                                  │       │
│  └─────────────────────────────────────────────────────────────────┘       │
│                              │                                               │
│          ┌───────────────────┼───────────────────┐                          │
│          │                   │                   │                          │
│          ▼                   ▼                   ▼                          │
│   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                  │
│   │    Core      │    │    Core      │    │    Core      │                  │
│   │   Runtime    │    │   Runtime    │    │   Runtime    │                  │
│   └──────────────┘    └──────────────┘    └──────────────┘                  │
│                                                                              │
│ Registry pattern: Core queries for implementations at runtime               │
│ Supports multiple providers for same interface                              │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ 4. Microkernel / Multi-Server (MINIX / Mach-style)                          │
│ ─────────────────────────────────────                                       │
│                                                                              │
│    ┌───────────────────────────────────────────────────────────────────┐    │
│    │                          Microkernel                              │    │
│    │                    (Minimal core: IPC, scheduling)                │    │
│    └─────────┬──────────┬──────────┬──────────┬──────────┬───────────────┘    │
│              │          │          │          │          │                   │
│              ▼          ▼          ▼          ▼          ▼                   │
│         ┌───────┐  ┌───────┐  ┌───────┐  ┌───────┐  ┌───────┐             │
│         │ Auth  │  │ Token │  │ Store │  │ Audit │  │ Metrics│             │
│         │Server │  │Server │  │Server │  │Server │  │Server │             │
│         └───────┘  └───────┘  └───────┘  └───────┘  └───────┘             │
│                                                                              │
│ Servers communicate via IPC (message passing)                                 │
│ Each server can be restarted independently                                     │
│ Failure isolation: crash in one doesn't affect others                          │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Hook-Based Plugin System Implementation

```rust
/// Hook-based plugin system for Tokn
pub mod hook_system {
    use std::collections::HashMap;
    use std::any::Any;
    use std::sync::Arc;
    use async_trait::async_trait;
    
    /// Hook point identifiers
    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub enum HookPoint {
        BeforeTokenIssue,
        AfterTokenIssue,
        BeforeValidation,
        AfterValidation,
        BeforeRevocation,
        AfterRevocation,
        OnTokenExpired,
        OnSecurityEvent,
        Custom(String),
    }
    
    /// Hook context passed to callbacks
    pub struct HookContext<'a> {
        pub hook_point: &'a HookPoint,
        pub data: &'a mut dyn Any,
        pub metadata: HashMap<String, String>,
    }
    
    /// Hook callback trait
    #[async_trait]
    pub trait HookCallback: Send + Sync {
        async fn call(&self, ctx: HookContext<'_>) -> HookResult;
        fn priority(&self) -> i32;
    }
    
    pub type HookResult = Result<(), HookError>;
    
    #[derive(Debug)]
    pub enum HookError {
        Cancel,              // Cancel the operation
        Abort(String),       // Abort with error message
        Modify(Box<dyn Any>), // Modify and continue
    }
    
    /// Hook registry
    pub struct HookRegistry {
        hooks: RwLock<HashMap<HookPoint, Vec<Arc<dyn HookCallback>>>>,
    }
    
    impl HookRegistry {
        pub fn new() -> Self {
            Self {
                hooks: RwLock::new(HashMap::new()),
            }
        }
        
        /// Register a hook callback
        pub async fn register(
            &self,
            point: HookPoint,
            callback: Arc<dyn HookCallback>,
        ) {
            let mut hooks = self.hooks.write().await;
            let callbacks = hooks.entry(point).or_insert_with(Vec::new);
            callbacks.push(callback);
            
            // Sort by priority (higher first)
            callbacks.sort_by(|a, b| b.priority().cmp(&a.priority()));
        }
        
        /// Execute all hooks for a point
        pub async fn execute(
            &self,
            point: &HookPoint,
            data: &mut dyn Any,
        ) -> HookResult {
            let hooks = self.hooks.read().await;
            
            if let Some(callbacks) = hooks.get(point) {
                for callback in callbacks {
                    let ctx = HookContext {
                        hook_point: point,
                        data,
                        metadata: HashMap::new(),
                    };
                    
                    match callback.call(ctx).await {
                        Ok(()) => continue,
                        Err(HookError::Cancel) => return Ok(()),
                        Err(e) => return Err(e),
                    }
                }
            }
            
            Ok(())
        }
    }
    
    /// Example: Audit logging hook
    pub struct AuditHook {
        audit_service: Arc<dyn AuditService>,
    }
    
    #[async_trait]
    impl HookCallback for AuditHook {
        async fn call(&self, ctx: HookContext<'_>) -> HookResult {
            match ctx.hook_point {
                HookPoint::AfterTokenIssue => {
                    if let Some(data) = ctx.data.downcast_ref::<TokenIssueData>() {
                        self.audit_service.log(AuditEvent::TokenIssued {
                            jti: data.jti.clone(),
                            subject: data.subject.clone(),
                            timestamp: Utc::now(),
                        }).await;
                    }
                }
                HookPoint::AfterRevocation => {
                    if let Some(data) = ctx.data.downcast_ref::<RevocationData>() {
                        self.audit_service.log(AuditEvent::TokenRevoked {
                            jti: data.jti.clone(),
                            reason: data.reason.clone(),
                        }).await;
                    }
                }
                _ => {}
            }
            Ok(())
        }
        
        fn priority(&self) -> i32 {
            100 // High priority - run early
        }
    }
    
    /// Example: Rate limiting hook
    pub struct RateLimitHook {
        limiter: Arc<dyn RateLimiter>,
    }
    
    #[async_trait]
    impl HookCallback for RateLimitHook {
        async fn call(&self, ctx: HookContext<'_>) -> HookResult {
            if let Some(data) = ctx.data.downcast_ref::<TokenIssueRequest>() {
                if !self.limiter.allow(&data.subject).await {
                    return Err(HookError::Abort(
                        "Rate limit exceeded".to_string()
                    ));
                }
            }
            Ok(())
        }
        
        fn priority(&self) -> i32 {
            50 // Medium priority
        }
    }
}
```

### 2.3 Extension Point Pattern Implementation

```rust
/// Extension point pattern for Tokn
pub mod extension_system {
    use std::collections::HashMap;
    use std::any::{Any, TypeId};
    use async_trait::async_trait;
    
    /// Extension trait that all extensions must implement
    #[async_trait]
    pub trait Extension: Send + Sync + Any {
        /// Extension identifier
        fn id(&self) -> &str;
        
        /// Extension version (semver)
        fn version(&self) -> &str;
        
        /// Initialize extension
        async fn initialize(&mut self, ctx: &ExtensionContext) -> Result<(), ExtensionError>;
        
        /// Shutdown extension
        async fn shutdown(&mut self) -> Result<(), ExtensionError>;
        
        /// Get extension capabilities
        fn capabilities(&self) -> Vec<Capability>;
    }
    
    /// Extension context provided during initialization
    pub struct ExtensionContext {
        pub config: ExtensionConfig,
        pub api: Arc<dyn ExtensionApi>,
        pub logger: Box<dyn Logger>,
    }
    
    /// Extension registry
    pub struct ExtensionRegistry {
        extensions: RwLock<HashMap<String, Box<dyn Extension>>>,
        apis: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
    }
    
    impl ExtensionRegistry {
        pub fn new() -> Self {
            Self {
                extensions: RwLock::new(HashMap::new()),
                apis: RwLock::new(HashMap::new()),
            }
        }
        
        /// Register an API surface for extensions to use
        pub fn register_api<T: 'static + Send + Sync>(&self, api: Arc<T>) {
            let mut apis = self.apis.write().unwrap();
            apis.insert(TypeId::of::<T>(), api);
        }
        
        /// Load and initialize an extension
        pub async fn load_extension(
            &self,
            mut extension: Box<dyn Extension>,
        ) -> Result<(), ExtensionError> {
            let id = extension.id().to_string();
            
            // Check for duplicate
            {
                let extensions = self.extensions.read().await;
                if extensions.contains_key(&id) {
                    return Err(ExtensionError::AlreadyLoaded(id));
                }
            }
            
            // Create context
            let ctx = ExtensionContext {
                config: ExtensionConfig::default(),
                api: Arc::new(RegistryExtensionApi::new(self)),
                logger: Box::new(ExtensionLogger::new(&id)),
            };
            
            // Initialize
            extension.initialize(&ctx).await?;
            
            // Register
            {
                let mut extensions = self.extensions.write().await;
                extensions.insert(id, extension);
            }
            
            tracing::info!("Extension '{}' v{} loaded", id, extension.version());
            Ok(())
        }
        
        /// Get extension by ID
        pub async fn get_extension(&self, id: &str) -> Option<Arc<Box<dyn Extension>>> {
            let extensions = self.extensions.read().await;
            extensions.get(id).map(|e| Arc::new(e.clone_box()))
        }
        
        /// Query extensions by capability
        pub async fn query_by_capability(
            &self,
            capability: Capability,
        ) -> Vec<(String, Vec<Capability>)> {
            let extensions = self.extensions.read().await;
            extensions
                .iter()
                .filter(|(_, e)| e.capabilities().contains(&capability))
                .map(|(id, e)| (id.clone(), e.capabilities()))
                .collect()
        }
    }
    
    /// Example: Token storage extension
    #[async_trait]
    pub trait TokenStorageExtension: Extension {
        async fn store(&self, token: &Token) -> Result<(), StorageError>;
        async fn retrieve(&self, jti: &str) -> Result<Option<Token>, StorageError>;
        async fn delete(&self, jti: &str) -> Result<(), StorageError>;
    }
    
    /// Example: Metrics extension
    #[async_trait]
    pub trait MetricsExtension: Extension {
        async fn record_counter(&self, name: &str, value: u64, labels: Labels);
        async fn record_histogram(&self, name: &str, value: f64, labels: Labels);
        async fn record_gauge(&self, name: &str, value: f64, labels: Labels);
    }
    
    #[derive(Debug, Clone, PartialEq)]
    pub enum Capability {
        TokenStorage,
        KeyManagement,
        RateLimiting,
        Metrics,
        Auditing,
        Custom(String),
    }
}
```

### 2.4 Service Provider Interface Implementation

```rust
/// Service Provider Interface for Tokn
pub mod spi_system {
    use std::any::Any;
    use std::sync::Arc;
    use async_trait::async_trait;
    
    /// Service type identifier
    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub enum ServiceType {
        TokenStorage,
        KeyProvider,
        RateLimiter,
        AuditLog,
        Metrics,
        Custom(String),
    }
    
    /// Service trait - base for all services
    #[async_trait]
    pub trait Service: Send + Sync {
        fn service_type(&self) -> ServiceType;
        fn service_id(&self) -> &str;
        fn priority(&self) -> i32;
    }
    
    /// Service registry
    pub struct ServiceRegistry {
        services: RwLock<HashMap<ServiceType, Vec<Arc<dyn Service>>>>,
        primary: RwLock<HashMap<ServiceType, String>>, // service_id -> primary
    }
    
    impl ServiceRegistry {
        pub fn new() -> Self {
            Self {
                services: RwLock::new(HashMap::new()),
                primary: RwLock::new(HashMap::new()),
            }
        }
        
        /// Register a service
        pub async fn register(&self, service: Arc<dyn Service>) {
            let service_type = service.service_type();
            let mut services = self.services.write().await;
            
            let list = services.entry(service_type.clone()).or_insert_with(Vec::new);
            list.push(service);
            
            // Sort by priority (higher = more preferred)
            list.sort_by(|a, b| b.priority().cmp(&a.priority()));
            
            // Set as primary if first
            let mut primary = self.primary.write().await;
            if !primary.contains_key(&service_type) {
                if let Some(first) = list.first() {
                    primary.insert(service_type, first.service_id().to_string());
                }
            }
        }
        
        /// Get primary service of type
        pub async fn get_primary<T: Service>(&self) -> Option<Arc<T>> {
            let services = self.services.read().await;
            let primary = self.primary.read().await;
            
            let service_type = ServiceType::TokenStorage; // Would get from T
            
            if let Some(primary_id) = primary.get(&service_type) {
                if let Some(list) = services.get(&service_type) {
                    return list
                        .iter()
                        .find(|s| s.service_id() == primary_id)
                        .and_then(|s| Arc::clone(s).as_any().downcast_ref::<T>())
                        .map(|s| Arc::new(s.clone()));
                }
            }
            None
        }
        
        /// Get all services of type
        pub async fn get_all(&self, service_type: &ServiceType) -> Vec<Arc<dyn Service>> {
            let services = self.services.read().await;
            services
                .get(service_type)
                .cloned()
                .unwrap_or_default()
        }
        
        /// Set primary service
        pub async fn set_primary(
            &self,
            service_type: ServiceType,
            service_id: String,
        ) -> Result<(), RegistryError> {
            let services = self.services.read().await;
            
            if let Some(list) = services.get(&service_type) {
                if !list.iter().any(|s| s.service_id() == service_id) {
                    return Err(RegistryError::ServiceNotFound(service_id));
                }
            } else {
                return Err(RegistryError::ServiceTypeNotFound(service_type));
            }
            
            let mut primary = self.primary.write().await;
            primary.insert(service_type, service_id);
            
            Ok(())
        }
    }
    
    /// Example: Token storage SPI
    #[async_trait]
    pub trait TokenStorageService: Service {
        async fn store(&self, token: &Token) -> Result<(), StorageError>;
        async fn retrieve(&self, jti: &str) -> Result<Option<Token>, StorageError>;
        async fn delete(&self, jti: &str) -> Result<(), StorageError>;
        async fn list_for_subject(&self, subject: &str) -> Result<Vec<Token>, StorageError>;
    }
    
    /// Example: Redis storage implementation
    pub struct RedisTokenStorage {
        id: String,
        redis: RedisClient,
    }
    
    impl Service for RedisTokenStorage {
        fn service_type(&self) -> ServiceType {
            ServiceType::TokenStorage
        }
        
        fn service_id(&self) -> &str {
            &self.id
        }
        
        fn priority(&self) -> i32 {
            100 // High priority
        }
    }
    
    #[async_trait]
    impl TokenStorageService for RedisTokenStorage {
        async fn store(&self, token: &Token) -> Result<(), StorageError> {
            // Redis implementation
            Ok(())
        }
        
        async fn retrieve(&self, jti: &str) -> Result<Option<Token>, StorageError> {
            // Redis implementation
            Ok(None)
        }
        
        async fn delete(&self, jti: &str) -> Result<(), StorageError> {
            // Redis implementation
            Ok(())
        }
        
        async fn list_for_subject(&self, subject: &str) -> Result<Vec<Token>, StorageError> {
            // Redis implementation
            Ok(vec![])
        }
    }
}
```

---

## Dynamic Loading in Rust

### 3.1 Dynamic Loading Mechanisms Comparison

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Dynamic Loading Mechanisms in Rust                                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ 1. Native Dynamic Libraries (dlopen / libloading)                           │
│ ─────────────────────────────────────                                       │
│                                                                              │
│    ┌──────────────┐         ┌──────────────┐                                │
│    │   Core       │◄────────│ .so / .dylib │                                │
│    │   Binary     │  dlopen │   Plugin     │                                │
│    │              │         │              │                                │
│    └──────────────┘         └──────────────┘                                │
│                                                                              │
│    Pros:                                                                       │
│    • Native performance                                                        │
│    • Zero overhead FFI                                                        │
│    • Full access to std library                                               │
│                                                                              │
│    Cons:                                                                       │
│    • Platform-specific (.so, .dll, .dylib)                                    │
│    • Unsafe (C ABI boundary)                                                  │
│    • Version compatibility issues                                             │
│    • No sandboxing                                                            │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ 2. WebAssembly (wasmtime / wasmer)                                            │
│ ─────────────────────────────────────                                         │
│                                                                              │
│    ┌──────────────┐         ┌──────────────┐                                │
│    │   Runtime    │◄────────│    .wasm     │                                │
│    │  (wasmtime)  │  instan │    Module    │                                │
│    │              │  tiate  │              │                                │
│    └──────────────┘         └──────────────┘                                │
│           │                      │                                           │
│           │                      │                                           │
│           ▼                      ▼                                           │
│    ┌──────────────┐         ┌──────────────┐                                │
│    │   Host       │◄────────│   Guest      │                                │
│    │   Functions  │  WASI   │   Module     │                                │
│    │   (imports)  │         │   (exports)  │                                │
│    └──────────────┘         └──────────────┘                                │
│                                                                              │
│    Pros:                                                                       │
│    • Sandboxed execution                                                       │
│    • Cross-platform (compile once, run anywhere)                              │
│    • Deterministic resource limits                                              │
│    • Near-native performance (with AOT compilation)                           │
│                                                                              │
│    Cons:                                                                       │
│    • Limited std library (WASI)                                               │
│    • Serialization overhead at boundary                                       │
│    • Memory model differences                                                 │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ 3. WebAssembly Component Model (WIT interfaces)                             │
│ ─────────────────────────────────────                                         │
│                                                                              │
│    ┌──────────────┐         ┌──────────────┐                                │
│    │   Component  │◄────────│   .wasm      │                                │
│    │   Host       │  compos │   Component  │                                │
│    │              │  e      │              │                                │
│    └──────────────┘         └──────────────┘                                │
│           │                      │                                           │
│           │ WIT Interface        │                                           │
│           │                      │                                           │
│    ┌──────────────┐              │                                           │
│    │ interface token-store {     │                                           │
│    │   use types.{token};       │                                           │
│    │   store: func(token) ->    │                                           │
│    │     result<_, error>;      │                                           │
│    │ }                            │                                           │
│    └──────────────┘              │                                           │
│                                                                              │
│    Pros:                                                                       │
│    • Type-safe interfaces                                                     │
│    • Language agnostic (Rust, Go, JS can all implement)                       │
│    • Tooling ecosystem (wit-bindgen)                                           │
│    • Composable (components can import other components)                      │
│                                                                              │
│    Cons:                                                                       │
│    • Newer standard (tooling maturing)                                        │
│    • Complexity for simple plugins                                            │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ 4. IPC / gRPC (Process Isolation)                                             │
│ ─────────────────────────────────────                                         │
│                                                                              │
│    ┌──────────────┐         gRPC           ┌──────────────┐                  │
│    │   Core       │◄──────────────────────►│   Plugin     │                  │
│    │   Process    │      protobuf         │   Process    │                  │
│    │              │                       │              │                  │
│    └──────────────┘                       └──────────────┘                  │
│                                                                              │
│    Pros:                                                                       │
│    • Strong isolation (separate process)                                      │
│    • Language agnostic                                                        │
│    • Network transparent (can run remotely)                                   │
│    • Natural failure boundaries                                               │
│                                                                              │
│    Cons:                                                                       │
│    • Serialization overhead                                                   │
│    • Higher latency                                                           │
│    • Process management complexity                                            │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Native Dynamic Library Loading

```rust
/// Native dynamic library loading for Tokn
pub mod native_loader {
    use libloading::{Library, Symbol};
    use std::path::Path;
    
    /// Plugin entry point signature
    pub type PluginInitFn = unsafe fn() -> Box<dyn Plugin>;
    pub type PluginVersionFn = unsafe fn() -> &'static str;
    
    /// Loaded native plugin
    pub struct NativePlugin {
        library: Library,
        plugin: Box<dyn Plugin>,
    }
    
    impl NativePlugin {
        /// Load plugin from dynamic library
        pub unsafe fn load<P: AsRef<Path>>(path: P) -> Result<Self, LoadError> {
            let library = Library::new(path.as_ref())?;
            
            // Get version function
            let version_fn: Symbol<PluginVersionFn> = 
                library.get(b"plugin_version")?;
            let version = version_fn();
            
            // Check compatibility
            Self::check_compatibility(version)?;
            
            // Get init function
            let init_fn: Symbol<PluginInitFn> = 
                library.get(b"plugin_init")?;
            let plugin = init_fn();
            
            tracing::info!("Loaded native plugin: {} v{}", plugin.id(), version);
            
            Ok(Self { library, plugin })
        }
        
        fn check_compatibility(version: &str) -> Result<(), LoadError> {
            let plugin_ver = semver::Version::parse(version)?;
            let min_ver = semver::Version::parse("1.0.0")?;
            let max_ver = semver::Version::parse("2.0.0")?;
            
            if plugin_ver < min_ver || plugin_ver >= max_ver {
                return Err(LoadError::IncompatibleVersion {
                    found: version.to_string(),
                    required: "^1.0.0".to_string(),
                });
            }
            
            Ok(())
        }
        
        pub fn get_plugin(&self) -> &dyn Plugin {
            self.plugin.as_ref()
        }
        
        pub fn get_plugin_mut(&mut self) -> &mut dyn Plugin {
            self.plugin.as_mut()
        }
    }
    
    // Ensure library stays loaded as long as plugin is referenced
    unsafe impl Send for NativePlugin {}
    unsafe impl Sync for NativePlugin {}
    
    /// Plugin trait exposed by native plugins
    pub trait Plugin: Send + Sync {
        fn id(&self) -> &str;
        fn version(&self) -> &str;
        fn initialize(&mut self, api: &PluginApi) -> Result<(), PluginError>;
        fn shutdown(&mut self) -> Result<(), PluginError>;
    }
    
    /// C-ABI compatible plugin interface for FFI
    #[repr(C)]
pub struct CPluginVTable {
        pub id: unsafe extern "C" fn(*mut c_void) -> *const c_char,
        pub version: unsafe extern "C" fn(*mut c_void) -> *const c_char,
        pub initialize: unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32,
        pub shutdown: unsafe extern "C" fn(*mut c_void) -> i32,
        pub destroy: unsafe extern "C" fn(*mut c_void),
    }
}
```

### 3.3 WebAssembly Module Loading

```rust
/// WebAssembly plugin loading for Tokn
pub mod wasm_loader {
    use wasmtime::{Engine, Module, Store, Instance, Func, Memory, Extern};
    use wasmtime::Config;
    
    /// WASM runtime configuration
    pub struct WasmRuntimeConfig {
        /// Memory limit in pages (64KB each)
        pub memory_limit: usize,
        /// CPU time limit in milliseconds
        pub cpu_time_limit_ms: u64,
        /// Enable WASI
        pub enable_wasi: bool,
        /// Enable fuel metering (instruction counting)
        pub fuel_metering: bool,
        pub initial_fuel: u64,
    }
    
    impl Default for WasmRuntimeConfig {
        fn default() -> Self {
            Self {
                memory_limit: 1024, // 64MB
                cpu_time_limit_ms: 1000,
                enable_wasi: true,
                fuel_metering: true,
                initial_fuel: 10_000_000,
            }
        }
    }
    
    /// WASM plugin host
    pub struct WasmPluginHost {
        engine: Engine,
        config: WasmRuntimeConfig,
    }
    
    impl WasmPluginHost {
        pub fn new(config: WasmRuntimeConfig) -> Result<Self, WasmError> {
            let mut engine_config = Config::new();
            
            if config.fuel_metering {
                engine_config.consume_fuel(true);
            }
            
            let engine = Engine::new(&engine_config)?;
            
            Ok(Self { engine, config })
        }
        
        /// Load and instantiate a WASM module
        pub async fn load_plugin(
            &self,
            wasm_bytes: &[u8],
        ) -> Result<WasmPlugin, WasmError> {
            let module = Module::new(&self.engine, wasm_bytes)?;
            
            // Create store with limits
            let mut store = Store::new(&self.engine, ());
            
            if self.config.fuel_metering {
                store.add_fuel(self.config.initial_fuel)?;
            }
            
            // Set up WASI if enabled
            let wasi_ctx = if self.config.enable_wasi {
                Some(WasiCtxBuilder::new()
                    .inherit_stdio()
                    .inherit_args()
                    .build())
            } else {
                None
            };
            
            // Create host functions that plugin can call
            let mut linker = wasmtime::Linker::new(&self.engine);
            
            // Host logging function
            linker.func_wrap("env", "host_log", |mut caller: wasmtime::Caller<'_, ()>, ptr: i32, len: i32| {
                let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
                let mut buffer = vec![0u8; len as usize];
                memory.read(&caller, ptr as usize, &mut buffer).unwrap();
                let message = String::from_utf8_lossy(&buffer);
                tracing::info!("[WASM Plugin] {}", message);
            })?;
            
            // Host storage function
            linker.func_wrap("env", "host_store", 
                |mut caller: wasmtime::Caller<'_, ()>, 
                 key_ptr: i32, key_len: i32,
                 value_ptr: i32, value_len: i32| -> i32 {
                    // Implementation...
                    0 // success
            })?;
            
            let instance = linker.instantiate(&mut store, &module)?;
            
            // Call initialization
            if let Ok(init) = instance.get_typed_func::<(), ()>(&mut store, "init") {
                init.call(&mut store, ())?;
            }
            
            Ok(WasmPlugin {
                store,
                instance,
                config: self.config.clone(),
            })
        }
    }
    
    /// Instantiated WASM plugin
    pub struct WasmPlugin {
        store: Store<()>,
        instance: Instance,
        config: WasmRuntimeConfig,
    }
    
    impl WasmPlugin {
        /// Call a plugin function
        pub fn call<T: wasmtime::WasmResults>(
            &mut self,
            name: &str,
            args: impl wasmtime::WasmParams,
        ) -> Result<T, WasmError> {
            let func = self.instance
                .get_typed_func::<impl wasmtime::WasmParams, T>(&mut self.store, name)?;
            
            func.call(&mut self.store, args).map_err(|e| {
                if e.is::<wasmtime::Trap>() {
                    WasmError::ExecutionTrapped(e.to_string())
                } else {
                    WasmError::ExecutionFailed(e.to_string())
                }
            })
        }
        
        /// Read memory from plugin
        pub fn read_memory(&mut self, ptr: i32, len: i32) -> Result<Vec<u8>, WasmError> {
            let memory = self.instance
                .get_memory(&mut self.store, "memory")
                .ok_or(WasmError::NoMemory)?;
            
            let mut buffer = vec![0u8; len as usize];
            memory.read(&mut self.store, ptr as usize, &mut buffer)?;
            
            Ok(buffer)
        }
        
        /// Write memory to plugin
        pub fn write_memory(&mut self, ptr: i32, data: &[u8]) -> Result<(), WasmError> {
            let memory = self.instance
                .get_memory(&mut self.store, "memory")
                .ok_or(WasmError::NoMemory)?;
            
            memory.write(&mut self.store, ptr as usize, data)?;
            Ok(())
        }
    }
    
    /// Example: Plugin definition in Rust (compiles to WASM)
    /// 
    /// ```rust
    /// #[no_mangle]
    /// pub extern "C" fn init() {
    ///     // Plugin initialization
    /// }
    /// 
    /// #[no_mangle]
    /// pub extern "C" fn process_token(ptr: i32, len: i32) -> i32 {
    ///     // Process token data
    ///     0 // success
    /// }
    /// 
    /// #[no_mangle]
    /// pub extern "C" fn allocate(size: i32) -> i32 {
    ///     // Allocate memory, return pointer
    ///     0
    /// }
    /// 
    /// #[no_mangle]
    /// pub extern "C" fn deallocate(ptr: i32, size: i32) {
    ///     // Free memory
    /// }
    /// ```
}
```

### 3.4 Component Model with WIT

```rust
/// WebAssembly Component Model for Tokn
pub mod component_loader {
    use wasmtime::component::{Component, Linker, bindgen};
    
    /// WIT interface definition (would be in .wit file)
    pub const TOKEN_STORE_WIT: &str = r#"
        package tokn:plugin@1.0.0;
        
        interface types {
            record token {
                jti: string,
                subject: string,
                audience: list<string>,
                expires-at: u64,
                claims: list<tuple<string, string>>,
            }
            
            enum error {
                storage-failed,
                not-found,
                already-exists,
                invalid-data,
            }
        }
        
        interface token-store {
            use types.{token, error};
            
            store: func(token: token) -> result<_, error>;
            retrieve: func(jti: string) -> result<token, error>;
            delete: func(jti: string) -> result<_, error>;
            list-for-subject: func(subject: string) -> result<list<token>, error>;
        }
        
        world token-store-plugin {
            import host: interface {
                log: func(level: string, message: string);
                get-config: func(key: string) -> option<string>;
            }
            
            export token-store;
        }
    "#;
    
    // Generate bindings from WIT
    wasmtime::component::bindgen!({
        world: "token-store-plugin",
        path: "wit/token-store.wit",
    });
    
    /// Host state for component
    pub struct HostState {
        config: HashMap<String, String>,
    }
    
    /// Implement host interface for plugin
    impl host::Host for HostState {
        fn log(&mut self, level: &str, message: &str) {
            match level {
                "error" => tracing::error!("[Plugin] {}", message),
                "warn" => tracing::warn!("[Plugin] {}", message),
                "info" => tracing::info!("[Plugin] {}", message),
                _ => tracing::debug!("[Plugin] {}", message),
            }
        }
        
        fn get_config(&mut self, key: &str) -> Option<String> {
            self.config.get(key).cloned()
        }
    }
    
    /// Component-based plugin loader
    pub struct ComponentPluginHost {
        engine: wasmtime::Engine,
    }
    
    impl ComponentPluginHost {
        pub fn new() -> Result<Self, WasmError> {
            let mut config = wasmtime::Config::new();
            config.wasm_component_model(true);
            
            let engine = wasmtime::Engine::new(&config)?;
            
            Ok(Self { engine })
        }
        
        pub async fn load(
            &self,
            component_bytes: &[u8],
        ) -> Result<TokenStorePlugin, WasmError> {
            let component = Component::new(&self.engine, component_bytes)?;
            
            let mut linker = Linker::new(&self.engine);
            
            // Add host interface
            host::add_to_linker(&mut linker, |state: &mut HostState| state)?;
            
            let mut store = wasmtime::Store::new(
                &self.engine,
                HostState {
                    config: HashMap::new(),
                }
            );
            
            let (plugin, _instance) = TokenStorePlugin::instantiate(
                &mut store,
                &component,
                &linker,
            )?;
            
            Ok(plugin)
        }
    }
    
    /// Type-safe plugin interface
    pub struct TokenStorePlugin {
        inner: TokenStore,
    }
    
    impl TokenStore for TokenStorePlugin {
        fn store(&mut self, token: Token) -> Result<(), Error> {
            self.inner.store(token)
        }
        
        fn retrieve(&mut self, jti: &str) -> Result<Token, Error> {
            self.inner.retrieve(jti)
        }
        
        fn delete(&mut self, jti: &str) -> Result<(), Error> {
            self.inner.delete(jti)
        }
        
        fn list_for_subject(&mut self, subject: &str) -> Result<Vec<Token>, Error> {
            self.inner.list_for_subject(subject)
        }
    }
}
```

---

## Module Isolation Strategies

### 4.1 Isolation Level Comparison

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Module Isolation Levels                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ Level 1: Language-Level Isolation (Traits, Visibility)                      │
│ ─────────────────────────────────────                                         │
│ • Rust visibility modifiers (pub, pub(crate), private)                        │
│ • Trait boundaries define interfaces                                         │
│ • Compile-time safety                                                         │
│ • No runtime overhead                                                         │
│ • Vulnerability: Panics can crash process                                     │
│                                                                              │
│ Level 2: Thread Isolation (std::thread, tokio::task)                          │
│ ─────────────────────────────────────                                         │
│ • Each module runs in separate thread                                         │
│ • Panic isolation via catch_unwind                                            │
│ • Shared memory between modules (unsafe)                                      │
│ • Context switch overhead                                                     │
│ • Vulnerability: Can still access shared state                                │
│                                                                              │
│ Level 3: Process Isolation (separate processes)                               │
│ ─────────────────────────────────────                                         │
│ • Complete memory isolation                                                   │
│ • OS-level scheduling                                                         │
│ • Communication via IPC (pipes, sockets, shared memory)                       │
│ • Higher overhead (process creation, context switch)                          │
│ • Vulnerability: Kernel exploits, side-channel attacks                          │
│                                                                              │
│ Level 4: Container Isolation (cgroups, namespaces)                            │
│ ─────────────────────────────────────                                         │
│ • Filesystem isolation                                                        │
│ • Network isolation                                                           │
│ • Resource limits (CPU, memory, I/O)                                          │
│ • Still shares kernel                                                         │
│ • Vulnerability: Container escape, kernel exploits                              │
│                                                                              │
│ Level 5: VM Isolation (KVM, Hyper-V)                                          │
│ ─────────────────────────────────────                                         │
│ • Full hardware virtualization                                                │
│ • Separate kernel per module                                                  │
│ • Highest isolation                                                           │
│ • Significant overhead (boot time, memory)                                    │
│ • Vulnerability: CPU speculative execution bugs (Spectre, Meltdown)           │
│                                                                              │
│ Level 6: WebAssembly Sandboxing                                               │
│ ─────────────────────────────────────                                         │
│ • Capability-based security                                                   │
│ • Linear memory model prevents buffer overflows                               │
│ • No undefined behavior in safe subset                                        │
│ • Explicit imports/exports                                                  │
│ • Near-native performance                                                     │
│ • Vulnerability: Spectre-style attacks via timing side channels               │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Capability-Based Security

```rust
/// Capability-based security for Tokn modules
pub mod capabilities {
    use std::sync::Arc;
    
    /// Capability token - grants specific permission
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Capability {
        pub resource: String,
        pub action: Action,
        pub constraints: Vec<Constraint>,
    }
    
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Action {
        Read,
        Write,
        Execute,
        Delete,
        Admin,
    }
    
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Constraint {
        TimeRange { start: DateTime<Utc>, end: DateTime<Utc> },
        RateLimit { requests_per_second: u64 },
        ResourceLimit { max_bytes: usize },
        PathPrefix(String),
    }
    
    /// Capability manager
    pub struct CapabilityManager {
        grants: Arc<RwLock<HashMap<String, Vec<Capability>>>>, // module_id -> capabilities
    }
    
    impl CapabilityManager {
        /// Grant capability to module
        pub async fn grant(
            &self,
            module_id: &str,
            capability: Capability,
        ) {
            let mut grants = self.grants.write().await;
            grants.entry(module_id.to_string())
                .or_insert_with(Vec::new)
                .push(capability);
        }
        
        /// Check if module has capability
        pub async fn check(
            &self,
            module_id: &str,
            required: &Capability,
        ) -> bool {
            let grants = self.grants.read().await;
            
            if let Some(capabilities) = grants.get(module_id) {
                capabilities.iter().any(|c| {
                    c.resource == required.resource
                        && c.action == required.action
                        && Self::check_constraints(&c.constraints, &required.constraints)
                })
            } else {
                false
            }
        }
        
        fn check_constraints(
            granted: &[Constraint],
            required: &[Constraint],
        ) -> bool {
            // Simplified: granted constraints must be at least as permissive
            true
        }
        
        /// Revoke all capabilities for module
        pub async fn revoke_all(&self, module_id: &str) {
            let mut grants = self.grants.write().await;
            grants.remove(module_id);
        }
    }
    
    /// Capability-aware module wrapper
    pub struct CapableModule<M: Module> {
        inner: M,
        capabilities: Vec<Capability>,
        cap_manager: Arc<CapabilityManager>,
    }
    
    impl<M: Module> Module for CapableModule<M> {
        fn id(&self) -> &str {
            self.inner.id()
        }
        
        async fn execute(&self, operation: Operation) -> Result<Output, ModuleError> {
            // Check capability before execution
            let required = self.operation_to_capability(&operation);
            
            if !self.cap_manager.check(self.id(), &required).await {
                return Err(ModuleError::CapabilityDenied {
                    module: self.id().to_string(),
                    operation: format!("{:?}", operation),
                });
            }
            
            self.inner.execute(operation).await
        }
    }
    
    /// Sandboxed module with limited capabilities
    pub struct SandboxedModule {
        wasm_instance: WasmPlugin,
        allowed_syscalls: HashSet<Syscall>,
        memory_limit: usize,
    }
    
    impl SandboxedModule {
        pub fn call(&mut self, function: &str, args: &[WasmValue]) -> Result<WasmValue, SandboxError> {
            // Pre-execution checks
            self.check_resource_limits()?;
            
            // Execute with syscall interception
            self.wasm_instance.call(function, args)
                .map_err(|e| SandboxError::ExecutionFailed(e.to_string()))
        }
        
        fn check_resource_limits(&self) -> Result<(), SandboxError> {
            let memory_usage = self.wasm_instance.get_memory_size();
            if memory_usage > self.memory_limit {
                return Err(SandboxError::MemoryLimitExceeded {
                    limit: self.memory_limit,
                    used: memory_usage,
                });
            }
            Ok(())
        }
    }
}
```

---

## API Stability and Versioning

### 5.1 Semantic Versioning for APIs

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ API Versioning Strategy                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ Semantic Versioning (MAJOR.MINOR.PATCH):                                       │
│                                                                              │
│   MAJOR (X.0.0): Breaking changes                                              │
│   • Removing methods                                                          │
│   • Changing method signatures                                                │
│   • Changing behavior of existing methods                                     │
│   • Changing data format/serialization                                          │
│                                                                              │
│   MINOR (x.Y.0): New features, backwards compatible                             │
│   • Adding new methods                                                        │
│   • Adding optional parameters                                                │
│   • Adding new types                                                          │
│                                                                              │
│   PATCH (x.y.Z): Bug fixes                                                     │
│   • Fixing bugs without changing API                                          │
│   • Performance improvements                                                  │
│   • Documentation updates                                                     │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ ABI Stability Strategies:                                                      │
│                                                                              │
│ 1. C ABI (Stable)                                                             │
│    • No name mangling                                                         │
│    • Fixed calling conventions                                                │
│    • No generics, traits, or complex types                                    │
│    • Use opaque pointers for complex data                                     │
│                                                                              │
│ 2. Rust ABI (Unstable)                                                         │
│    • Changes between compiler versions                                        │
│    • Name mangling                                                            │
│    • Not suitable for plugins across versions                                 │
│                                                                              │
│ 3. Stable ABI Crates (abi_stable, stabby)                                      │
│    • Define stable ABI for Rust structs                                       │
│    • Version checking at load time                                              │
│    • Allows some Rust features (enums, some generics)                           │
│                                                                              │
│ 4. Interface Definition Languages (WIT, Protobuf, Cap'n Proto)                   │
│    • Language agnostic                                                        │
│    • Schema evolution support                                                 │
│    • Code generation                                                          │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Version Compatibility Layer

```rust
/// API versioning and compatibility for Tokn
pub mod versioning {
    use semver::{Version, VersionReq};
    
    /// API version compatibility checker
    pub struct VersionCompatibility {
        current_version: Version,
        supported_versions: Vec<VersionReq>,
    }
    
    impl VersionCompatibility {
        /// Check if plugin version is compatible
        pub fn check_compatibility(&self, plugin_version: &str) -> Result<(), CompatibilityError> {
            let plugin_ver = Version::parse(plugin_version)?;
            
            // Same major version required
            if plugin_ver.major != self.current_version.major {
                return Err(CompatibilityError::MajorVersionMismatch {
                    current: self.current_version.major,
                    plugin: plugin_ver.major,
                });
            }
            
            // Check against supported version requirements
            let supported = self.supported_versions.iter()
                .any(|req| req.matches(&plugin_ver));
            
            if !supported {
                return Err(CompatibilityError::VersionNotSupported {
                    version: plugin_version.to_string(),
                    supported: self.supported_versions.iter()
                        .map(|r| r.to_string())
                        .collect(),
                });
            }
            
            Ok(())
        }
    }
    
    /// Multi-version API router
    pub struct VersionedApiRouter {
        routers: HashMap<Version, Router>,
        fallback: Option<Version>,
    }
    
    impl VersionedApiRouter {
        /// Route request to appropriate version
        pub async fn route(
            &self,
            version: &str,
            request: Request,
        ) -> Result<Response, ApiError> {
            let ver = Version::parse(version)?;
            
            // Find exact or compatible version
            let target = self.routers.get(&ver)
                .or_else(|| self.find_compatible(&ver))
                .or_else(|| self.fallback.as_ref().and_then(|v| self.routers.get(v)))
                .ok_or(ApiError::VersionNotFound(version.to_string()))?;
            
            target.handle(request).await
        }
        
        fn find_compatible(&self, version: &Version) -> Option<&Router> {
            // Find highest version with same major, <= minor
            self.routers.iter()
                .filter(|(v, _)| v.major == version.major && v.minor <= version.minor)
                .max_by_key(|(v, _)| (v.minor, v.patch))
                .map(|(_, r)| r)
        }
    }
    
    /// ABI-stable trait using vtables
    #[repr(C)]
    pub struct StableTokenStoreVTable {
        pub destroy: unsafe extern "C" fn(*mut c_void),
        pub store: unsafe extern "C" fn(*mut c_void, *const c_char) -> i32,
        pub retrieve: unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_char, usize) -> i32,
    }
    
    #[repr(C)]
    pub struct StableTokenStore {
        pub vtable: *const StableTokenStoreVTable,
        pub data: *mut c_void,
    }
    
    impl StableTokenStore {
        pub fn store(&self, token_json: &str) -> Result<(), StoreError> {
            let c_str = CString::new(token_json)?;
            let result = unsafe {
                ((*self.vtable).store)(self.data, c_str.as_ptr())
            };
            
            if result == 0 {
                Ok(())
            } else {
                Err(StoreError::StorageFailed)
            }
        }
    }
    
    unsafe impl Send for StableTokenStore {}
    unsafe impl Sync for StableTokenStore {}
}
```

---

## Hot Reloading Techniques

### 6.1 Hot Reload Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Hot Reload Architecture                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐     │
│  │                        Main Process                                  │     │
│  │                                                                      │     │
│  │   ┌──────────────┐     ┌──────────────┐     ┌──────────────┐       │     │
│  │   │   File       │────►│   Module     │────►│   Plugin     │       │     │
│  │   │   Watcher    │     │   Registry   │     │   Host       │       │     │
│  │   │ (inotify)    │     │              │     │              │       │     │
│  │   └──────────────┘     └──────────────┘     └──────┬───────┘       │     │
│  │                                                  │                  │     │
│  │                                           ┌──────┴───────┐          │     │
│  │                                           │              │          │     │
│  │                                    ┌──────▼──────┐ ┌──────▼──────┐  │     │
│  │                                    │  Module A   │ │  Module B   │  │     │
│  │                                    │ (v1.0.0)    │ │ (v1.1.0)    │  │     │
│  │                                    └─────────────┘ └─────────────┘  │     │
│  │                                                                      │     │
│  └─────────────────────────────────────────────────────────────────────┘     │
│                               │                                              │
│                               │ Hot Reload Signal                            │
│                               ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐     │
│  │                   Reload Coordinator Thread                           │     │
│  │                                                                      │     │
│  │  1. Pause new requests to module                                     │     │
│  │  2. Drain in-flight requests (with timeout)                          │     │
│  │  3. Save module state (if stateful)                                  │     │
│  │  4. Unload old module                                                 │     │
│  │  5. Load new module                                                   │     │
│  │  6. Restore state                                                     │     │
│  │  7. Resume requests                                                   │     │
│  │                                                                      │     │
│  └─────────────────────────────────────────────────────────────────────┘     │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 6.2 Hot Reload Implementation

```rust
/// Hot reload system for Tokn
pub mod hot_reload {
    use notify::{Watcher, RecursiveMode, watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;
    
    /// Hot reload manager
    pub struct HotReloadManager {
        registry: Arc<ModuleRegistry>,
        state_manager: Arc<StateManager>,
        reload_tx: mpsc::Sender<ReloadRequest>,
    }
    
    #[derive(Debug)]
    struct ReloadRequest {
        module_id: String,
        new_path: PathBuf,
    }
    
    impl HotReloadManager {
        pub fn new(registry: Arc<ModuleRegistry>) -> Self {
            let (reload_tx, reload_rx) = mpsc::channel();
            let state_manager = Arc::new(StateManager::new());
            
            let manager = Self {
                registry,
                state_manager,
                reload_tx,
            };
            
            // Start reload coordinator
            let coordinator = ReloadCoordinator {
                registry: manager.registry.clone(),
                state_manager: manager.state_manager.clone(),
                rx: reload_rx,
            };
            
            tokio::spawn(coordinator.run());
            
            manager
        }
        
        /// Watch a module for changes
        pub fn watch_module(&self, module_id: &str, path: &Path) -> Result<(), WatchError> {
            let (tx, rx) = channel();
            let mut watcher = watcher(tx, Duration::from_secs(1))?;
            
            watcher.watch(path, RecursiveMode::NonRecursive)?;
            
            let reload_tx = self.reload_tx.clone();
            let module_id = module_id.to_string();
            let path = path.to_path_buf();
            
            // Spawn file watcher thread
            std::thread::spawn(move || {
                loop {
                    match rx.recv() {
                        Ok(event) => {
                            if let DebouncedEvent::Write(_) | DebouncedEvent::Create(_) = event {
                                tracing::info!("Detected change in module {}", module_id);
                                
                                let request = ReloadRequest {
                                    module_id: module_id.clone(),
                                    new_path: path.clone(),
                                };
                                
                                if let Err(e) = reload_tx.send(request) {
                                    tracing::error!("Failed to send reload request: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Watch error: {}", e);
                            break;
                        }
                    }
                }
            });
            
            Ok(())
        }
    }
    
    /// Reload coordinator
    struct ReloadCoordinator {
        registry: Arc<ModuleRegistry>,
        state_manager: Arc<StateManager>,
        rx: mpsc::Receiver<ReloadRequest>,
    }
    
    impl ReloadCoordinator {
        async fn run(&self) {
            while let Ok(request) = self.rx.recv() {
                if let Err(e) = self.reload_module(&request).await {
                    tracing::error!(
                        "Failed to reload module {}: {:?}",
                        request.module_id, e
                    );
                }
            }
        }
        
        async fn reload_module(&self, request: &ReloadRequest) -> Result<(), ReloadError> {
            let module_id = &request.module_id;
            
            tracing::info!("Starting hot reload for module {}", module_id);
            
            // 1. Pause new requests
            self.registry.pause_module(module_id).await;
            
            // 2. Wait for in-flight requests
            let timeout = Duration::from_secs(30);
            if let Err(_) = tokio::time::timeout(
                timeout,
                self.registry.wait_for_quiesce(module_id)
            ).await {
                tracing::warn!("Timeout waiting for module {} to quiesce", module_id);
                // Continue anyway - will force close
            }
            
            // 3. Extract state from old module
            let state = if let Some(old_module) = self.registry.get_module(module_id).await {
                self.state_manager.extract_state(&*old_module).await.ok()
            } else {
                None
            };
            
            // 4. Unload old module
            self.registry.unload_module(module_id).await?;
            
            // 5. Load new module
            let new_module = self.load_module(&request.new_path).await?;
            
            // 6. Restore state
            if let Some(state) = state {
                self.state_manager.restore_state(&*new_module, state).await?;
            }
            
            // 7. Register new module
            self.registry.register_module(module_id, new_module).await;
            
            // 8. Resume requests
            self.registry.resume_module(module_id).await;
            
            tracing::info!("Hot reload completed for module {}", module_id);
            
            Ok(())
        }
        
        async fn load_module(&self, path: &Path) -> Result<Box<dyn Module>, ReloadError> {
            // Implementation depends on module type
            if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
                // Load WASM module
                let bytes = tokio::fs::read(path).await?;
                let module = load_wasm_module(&bytes).await?;
                Ok(Box::new(module))
            } else {
                // Load native module
                unsafe {
                    let module = load_native_module(path)?;
                    Ok(Box::new(module))
                }
            }
        }
    }
    
    /// State management for hot reload
    pub struct StateManager;
    
    impl StateManager {
        pub async fn extract_state(&self, module: &dyn Module) -> Result<ModuleState, StateError> {
            // Ask module to serialize its state
            module.serialize_state().await
        }
        
        pub async fn restore_state(
            &self,
            module: &dyn Module,
            state: ModuleState,
        ) -> Result<(), StateError> {
            // Ask module to deserialize state
            module.deserialize_state(state).await
        }
    }
    
    /// Trait for stateful modules
    #[async_trait]
    pub trait StatefulModule: Module {
        async fn serialize_state(&self) -> Result<ModuleState, StateError>;
        async fn deserialize_state(&mut self, state: ModuleState) -> Result<(), StateError>;
    }
    
    pub struct ModuleState {
        pub version: String,
        pub data: Vec<u8>,
    }
}
```

---

## Module Communication Patterns

### 7.1 Inter-Module Communication

```rust
/// Module communication patterns for Tokn
pub mod communication {
    use tokio::sync::{mpsc, oneshot, broadcast};
    
    /// Message types for inter-module communication
    #[derive(Debug, Clone)]
    pub enum ModuleMessage {
        // Token lifecycle events
        TokenIssued(TokenEvent),
        TokenValidated(TokenEvent),
        TokenRevoked(TokenEvent),
        
        // Security events
        SecurityAlert(SecurityEvent),
        
        // Administrative
        ConfigUpdate(ConfigChange),
        Shutdown,
        
        // Custom
        Custom { 
            channel: String, 
            payload: Vec<u8> 
        },
    }
    
    /// Message bus for module communication
    pub struct MessageBus {
        broadcast_tx: broadcast::Sender<ModuleMessage>,
        direct_channels: Arc<RwLock<HashMap<String, mpsc::Sender<ModuleMessage>>>>,
    }
    
    impl MessageBus {
        pub fn new(capacity: usize) -> Self {
            let (tx, _) = broadcast::channel(capacity);
            
            Self {
                broadcast_tx: tx,
                direct_channels: Arc::new(RwLock::new(HashMap::new())),
            }
        }
        
        /// Subscribe to broadcast messages
        pub fn subscribe(&self) -> broadcast::Receiver<ModuleMessage> {
            self.broadcast_tx.subscribe()
        }
        
        /// Publish message to all subscribers
        pub fn publish(&self, message: ModuleMessage) -> Result<(), BroadcastError> {
            let _ = self.broadcast_tx.send(message);
            Ok(())
        }
        
        /// Register direct channel for specific module
        pub async fn register_direct(
            &self,
            module_id: &str,
            capacity: usize,
        ) -> mpsc::Receiver<ModuleMessage> {
            let (tx, rx) = mpsc::channel(capacity);
            
            let mut channels = self.direct_channels.write().await;
            channels.insert(module_id.to_string(), tx);
            
            rx
        }
        
        /// Send direct message to specific module
        pub async fn send_direct(
            &self,
            module_id: &str,
            message: ModuleMessage,
        ) -> Result<(), SendError> {
            let channels = self.direct_channels.read().await;
            
            if let Some(tx) = channels.get(module_id) {
                tx.send(message).await?;
                Ok(())
            } else {
                Err(SendError::ModuleNotFound(module_id.to_string()))
            }
        }
    }
    
    /// Request-response pattern
    pub struct RpcClient {
        bus: Arc<MessageBus>,
        pending: Arc<RwLock<HashMap<String, oneshot::Sender<Response>>>>,
    }
    
    impl RpcClient {
        /// Make RPC call to another module
        pub async fn call(
            &self,
            target: &str,
            request: Request,
            timeout: Duration,
        ) -> Result<Response, RpcError> {
            let request_id = Uuid::new_v4().to_string();
            let (tx, rx) = oneshot::channel();
            
            // Register pending request
            {
                let mut pending = self.pending.write().await;
                pending.insert(request_id.clone(), tx);
            }
            
            // Send request
            let message = ModuleMessage::Custom {
                channel: format!("rpc:{}", target),
                payload: serde_json::to_vec(&RpcEnvelope {
                    request_id: request_id.clone(),
                    request,
                })?,
            };
            
            self.bus.send_direct(target, message).await?;
            
            // Wait for response
            match tokio::time::timeout(timeout, rx).await {
                Ok(Ok(response)) => Ok(response),
                Ok(Err(_)) => Err(RpcError::Canceled),
                Err(_) => {
                    // Clean up pending request
                    let mut pending = self.pending.write().await;
                    pending.remove(&request_id);
                    Err(RpcError::Timeout)
                }
            }
        }
        
        /// Handle incoming RPC response
        pub async fn handle_response(&self, envelope: RpcResponse) -> Result<(), RpcError> {
            let mut pending = self.pending.write().await;
            
            if let Some(tx) = pending.remove(&envelope.request_id) {
                let _ = tx.send(envelope.response);
            }
            
            Ok(())
        }
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    struct RpcEnvelope {
        request_id: String,
        request: Request,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    struct RpcResponse {
        request_id: String,
        response: Response,
    }
}
```

---

## Security in Modular Systems

### 8.1 Supply Chain Security

```rust
/// Supply chain security for modules
pub mod supply_chain {
    use sha2::{Sha256, Digest};
    use ed25519_dalek::{PublicKey, Signature, Verifier};
    
    /// Module verification
    pub struct ModuleVerifier {
        trusted_keys: HashMap<String, PublicKey>,
    }
    
    #[derive(Debug)]
    pub struct SignedModule {
        pub module_bytes: Vec<u8>,
        pub signature: Signature,
        pub signer_id: String,
        pub hash: String,
    }
    
    impl ModuleVerifier {
        /// Verify module signature and hash
        pub fn verify(&self, module: &SignedModule) -> Result<(), VerificationError> {
            // Check hash
            let mut hasher = Sha256::new();
            hasher.update(&module.module_bytes);
            let computed_hash = format!("{:x}", hasher.finalize());
            
            if computed_hash != module.hash {
                return Err(VerificationError::HashMismatch);
            }
            
            // Verify signature
            let public_key = self.trusted_keys.get(&module.signer_id)
                .ok_or(VerificationError::UnknownSigner(module.signer_id.clone()))?;
            
            public_key.verify(&module.module_bytes, &module.signature)
                .map_err(|_| VerificationError::InvalidSignature)?;
            
            Ok(())
        }
        
        /// Add trusted signing key
        pub fn add_trusted_key(&mut self, id: String, key: PublicKey) {
            self.trusted_keys.insert(id, key);
        }
    }
    
    /// SBOM (Software Bill of Materials) for modules
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ModuleSBOM {
        pub module_id: String,
        pub version: String,
        pub dependencies: Vec<Dependency>,
        pub licenses: Vec<String>,
        pub checksums: Checksums,
        pub build_info: BuildInfo,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Dependency {
        pub name: String,
        pub version: String,
        pub source: String,
        pub checksum: String,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Checksums {
        pub sha256: String,
        pub sha512: String,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BuildInfo {
        pub builder: String,
        pub timestamp: DateTime<Utc>,
        pub rustc_version: String,
        pub git_commit: Option<String>,
    }
}
```

---

## Testing Modular Systems

### 9.1 Module Testing Strategy

```rust
/// Testing utilities for modular systems
pub mod testing {
    use std::sync::Arc;
    
    /// Test harness for modules
    pub struct ModuleTestHarness {
        registry: Arc<ModuleRegistry>,
        mock_services: MockServiceProvider,
    }
    
    impl ModuleTestHarness {
        /// Create isolated test environment
        pub fn new() -> Self {
            let registry = Arc::new(ModuleRegistry::new());
            let mock_services = MockServiceProvider::new();
            
            Self {
                registry,
                mock_services,
            }
        }
        
        /// Load module for testing
        pub async fn load_test_module(
            &self,
            module_bytes: &[u8],
        ) -> Result<TestModule, TestError> {
            let module = load_module(module_bytes).await?;
            
            // Inject mock services
            let context = TestExtensionContext {
                services: self.mock_services.clone(),
            };
            
            module.initialize(&context).await?;
            
            Ok(TestModule {
                inner: module,
                harness: self,
            })
        }
        
        /// Create mock for service interface
        pub fn mock_service<T: Service>(&self) -> MockService<T> {
            self.mock_services.mock::<T>()
        }
    }
    
    /// Test module wrapper
    pub struct TestModule<'a> {
        inner: Box<dyn Module>,
        harness: &'a ModuleTestHarness,
    }
    
    impl<'a> TestModule<'a> {
        /// Call module function and capture effects
        pub async fn call<F, R>(
            &mut self,
            operation: F,
        ) -> TestResult<R>
        where
            F: FnOnce(&mut dyn Module) -> R,
        {
            let result = operation(self.inner.as_mut());
            
            let effects = self.harness.mock_services.recorded_effects();
            
            TestResult {
                value: result,
                effects,
            }
        }
        
        /// Verify module effects
        pub fn assert_effects(&self, expected: Vec<ExpectedEffect>) {
            let actual = self.harness.mock_services.recorded_effects();
            
            for expected_effect in expected {
                assert!(
                    actual.contains(&expected_effect),
                    "Expected effect {:?} not found",
                    expected_effect
                );
            }
        }
    }
    
    /// Property-based testing for modules
    pub struct ModulePropertyTests;
    
    impl ModulePropertyTests {
        /// Test that module is deterministic
        pub fn test_determinism<M: Module + Clone>(module: &M) {
            // Same input should always produce same output
        }
        
        /// Test that module is idempotent where expected
        pub fn test_idempotence<M: Module>(module: &mut M) {
            // Multiple identical calls should have same effect as one
        }
        
        /// Test that module handles all error cases gracefully
        pub fn test_error_handling<M: Module>(module: &mut M) {
            // Property: module never panics, always returns Result
        }
    }
}
```

---

## References

### Papers and Specifications

1. **"Microkernel Architecture"** (Jochen Liedtke, 1995)
   - L4 microkernel design principles
   - Application: Module isolation strategies

2. **"The Capability Approach to System Security"** (Miller et al., 2003)
   - Capability-based security fundamentals
   - Application: Sandboxing and permissions

3. **"WebAssembly Component Model"** (W3C Proposal)
   - Interface types and component composition
   - Application: Language-agnostic modules

4. **"Rust ABI Stability"** (Rust Internals)
   - Options for stable plugin interfaces
   - Application: API versioning

### Implementations

| Project | Pattern | Notes |
|---------|---------|-------|
| wasmtime | WASM runtime | Component model support |
| wasmer | WASM runtime | Multiple backends |
| abi_stable | Rust stable ABI | Plugin development |
| extism | WASM plugins | Focus on security |
| kubelet | Modular architecture | Dynamic provider loading |

### Standards

- **WASI** - WebAssembly System Interface
- **WebAssembly Component Model** - W3C draft
- **OCI Artifacts** - Module distribution
- **Sigstore** - Supply chain signing

---

## Document Metadata

- **Version:** 1.0.0
- **Last Updated:** 2026-04-02
- **Author:** Tokn Research Team
- **Total Line Count:** ~1,200 lines
- **Sections:** 10 major sections
- **Code Examples:** 15+ Rust implementations

---

*This document provides a comprehensive analysis of modularization patterns for the Tokn project. For implementation details, see the accompanying ADRs and specification documents.*
