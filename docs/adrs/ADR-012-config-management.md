# ADR-012: Structured Configuration with Environment Overrides

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

We need a configuration system that supports:
- Type-safe configuration
- Environment-specific overrides
- Secrets management integration
- Configuration validation
- Hot reload capability

---

## Decision

We will use **YAML configuration with environment variable overrides** and runtime validation.

### Configuration Structure

```yaml
# config/tokn.yaml (base configuration)
server:
  host: "0.0.0.0"
  port: 8080
  workers: 4
  max_connections: 10000

database:
  primary:
    host: "localhost"
    port: 5432
    name: "tokn"
    user: "tokn_user"
    max_connections: 20
    min_connections: 5
    connection_timeout: 30s
    idle_timeout: 10m
  replica:
    host: "localhost"
    port: 5433
    name: "tokn"
    user: "tokn_user"

redis:
  cluster:
    - host: "localhost"
      port: 6379
  password: "${REDIS_PASSWORD}"
  database: 0
  pool_size: 10

security:
  token:
    issuer: "https://tokn.example.com"
    default_ttl: 1h
    max_ttl: 24h
    min_ttl: 1m
  signing:
    default_algorithm: "Ed25519"
    key_rotation_days: 90
  rate_limiting:
    enabled: true
    default_rate: 1000
    default_burst: 100

plugins:
  storage:
    - name: "postgres"
      enabled: true
    - name: "redis"
      enabled: true
  audit:
    - name: "redis_stream"
      enabled: true
    - name: "postgres"
      enabled: true

observability:
  tracing:
    enabled: true
    endpoint: "http://jaeger:14268/api/traces"
    sample_rate: 0.1
  metrics:
    enabled: true
    endpoint: "/metrics"
  logging:
    level: "info"
    format: "json"
```

### Environment Override Pattern

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Configuration Override Hierarchy                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Precedence (highest to lowest):                                              │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  1. Environment Variables                                             │  │
│  │     TOKN_DATABASE_PRIMARY_HOST=prod-db.example.com                   │  │
│  │     TOKN_SECURITY_TOKEN_ISSUER=https://tokn.production.com           │  │
│  │     TOKN_REDIS_PASSWORD=secret123                                   │  │
│  │                                                                      │  │
│  │     Format: TOKN_{SECTION}_{KEY} in uppercase                        │  │
│  │     Nested: TOKN_DATABASE__PRIMARY__HOST (double underscore)          │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  2. Environment-Specific Config                                       │  │
│  │     config/tokn.production.yaml                                       │  │
│  │     config/tokn.development.yaml                                      │  │
│  │     config/tokn.staging.yaml                                          │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                              │                                              │
│                              ▼                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  3. Base Configuration                                                │  │
│  │     config/tokn.yaml                                                  │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Implementation

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    
    #[serde(default)]
    pub database: DatabaseConfig,
    
    #[serde(default)]
    pub redis: RedisConfig,
    
    #[serde(default)]
    pub security: SecurityConfig,
    
    #[serde(default)]
    pub plugins: PluginsConfig,
    
    #[serde(default)]
    pub observability: ObservabilityConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        // 1. Load base config
        let base = Self::load_from_file("config/tokn.yaml")?;
        
        // 2. Load environment-specific config
        let env = std::env::var("TOKN_ENV")
            .unwrap_or_else(|_| "development".to_string());
        
        let env_config = Self::load_from_file(&format!("config/tokn.{}.yaml", env))?;
        
        // 3. Merge configs (env overrides base)
        let merged = base.merge(&env_config);
        
        // 4. Apply environment variable overrides
        let final_config = merged.apply_env_overrides()?;
        
        // 5. Validate configuration
        final_config.validate()?;
        
        Ok(final_config)
    }
    
    fn apply_env_overrides(mut self) -> Result<Self, ConfigError> {
        // Parse TOKN_* environment variables
        for (key, value) in std::env::vars() {
            if let Some(config_key) = key.strip_prefix("TOKN_") {
                self.set_override(config_key, value)?;
            }
        }
        Ok(self)
    }
    
    fn validate(&self) -> Result<(), ConfigError> {
        // Validate server settings
        if self.server.port == 0 {
            return Err(ConfigError::Validation("server.port must be non-zero".into()));
        }
        
        // Validate database connection
        if self.database.primary.host.is_empty() {
            return Err(ConfigError::Validation("database.primary.host required".into()));
        }
        
        // Validate token settings
        if self.security.token.default_ttl == Duration::ZERO {
            return Err(ConfigError::Validation("token.default_ttl must be non-zero".into()));
        }
        
        if self.security.token.default_ttl > self.security.token.max_ttl {
            return Err(ConfigError::Validation("default_ttl must be <= max_ttl".into()));
        }
        
        Ok(())
    }
}
```

### Secrets Management

| Secret Type | Storage | Rotation |
|-------------|---------|----------|
| **Database Password** | Environment or Vault | Manual |
| **Redis Password** | Environment or Vault | Manual |
| **Signing Keys** | HSM or Vault | Automated (90 days) |
| **API Keys** | Database | Manual |
| **JWT Secrets** | Environment | Manual |

---

## Consequences

### Positive
- Clear configuration hierarchy
- Type-safe and validated
- Environment-specific overrides
- Secrets kept out of version control
- Hot reload support

### Negative
- YAML complexity for nested config
- Environment variable proliferation possible
- Validation complexity
- Documentation burden

### Mitigation
- Generate config docs from schema
- Provide config examples for common setups
- Use config validation in CI
- Document override precedence clearly

---

## References

- [12-Factor App Configuration](https://12factor.net/config)
- [Serde YAML](https://docs.rs/serde_yaml/)
- [Vault Configuration](https://www.vaultproject.io/docs/configuration)
