# ADR-005: Audit Event Schema with Structured Logging

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

We need a comprehensive audit logging system that captures all token operations for security, compliance, and debugging. The system must be:
- Immutable once written
- Queryable for investigations
- Exportable to SIEM systems
- Performance-efficient

Previous approaches:
- **Unstructured text logs** - Difficult to query, inconsistent
- **Database append-only tables** - Not easily exportable
- **Cloud-native logging only** - Lock-in concerns

---

## Decision

We will use **structured audit events with dual-write to Redis Streams and PostgreSQL**.

### Audit Event Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    // Event Identity
    pub event_id: Uuid,
    pub event_type: AuditEventType,
    pub timestamp: DateTime<Utc>,
    
    // Token Context
    pub jti: Option<String>,
    pub subject: Option<String>,
    pub tenant_id: Option<String>,
    
    // Actor Context
    pub actor_id: Option<String>,
    pub actor_type: ActorType,
    pub source_ip: Option<IpAddr>,
    pub user_agent: Option<String>,
    
    // Operation Details
    pub operation: OperationType,
    pub success: bool,
    pub error_code: Option<String>,
    
    // Request Context
    pub request_id: Uuid,
    pub trace_id: Option<String>,
    
    // Additional Data (flexible)
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    TokenIssued,
    TokenValidated,
    TokenRevoked,
    TokenRefreshed,
    TokenExpired,
    ValidationFailed,
    KeyRotated,
    PluginLoaded,
    PluginUnloaded,
    RateLimitExceeded,
    ConfigurationChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorType {
    User,
    Service,
    System,
    Unknown,
}
```

### Storage Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Audit Storage Architecture                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                     Event Flow                                         │  │
│  │                                                                      │  │
│  │   Token Service ──► Audit Middleware ──► Dual Write                  │  │
│  │                              │                                        │  │
│  │                    ┌─────────┴─────────┐                             │  │
│  │                    │                   │                             │  │
│  │                    ▼                   ▼                             │  │
│  │           ┌──────────────┐    ┌──────────────┐                      │  │
│  │           │ Redis Stream │    │  PostgreSQL  │                      │  │
│  │           │   (Fast)     │    │  (Durable)   │                      │  │
│  │           └──────┬───────┘    └──────┬───────┘                      │  │
│  │                  │                   │                               │  │
│  │                  └─────────┬─────────┘                               │  │
│  │                            ▼                                         │  │
│  │                   ┌──────────────┐                                  │  │
│  │                   │   SIEM       │                                  │  │
│  │                   │   Export     │                                  │  │
│  │                   │   (Kafka)    │                                  │  │
│  │                   └──────────────┘                                  │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  Redis Stream: XREADGROUP BLOCK 2000 COUNT 100 STREAMS audit >              │
│                                                                              │
│  PostgreSQL: INSERT INTO audit_events VALUES ($1, $2, ...) ON CONFLICT      │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Structured Log Format (JSON)

```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "event_type": "TokenIssued",
  "timestamp": "2026-04-02T15:04:05.123Z",
  "jti": "12345678-1234-1234-1234-123456789012",
  "subject": "user_12345",
  "tenant_id": "tenant_abc",
  "actor_id": "service_token_issuer",
  "actor_type": "Service",
  "source_ip": "192.168.1.100",
  "operation": "IssueToken",
  "success": true,
  "request_id": "req_abc123",
  "trace_id": "trace_xyz789",
  "metadata": {
    "token_format": "JwtEd25519",
    "ttl_seconds": 3600,
    "scopes": ["read", "write"],
    "issuer": "https://tokn.example.com"
  }
}
```

### Query Patterns

| Query | Redis Command | PostgreSQL Query |
|-------|---------------|------------------|
| By JTI | XRANGE audit JTI_START JTI_END | SELECT * FROM audit WHERE jti = $1 |
| By Subject | XREADGROUP filtered | SELECT * FROM audit WHERE subject = $1 |
| By Time Range | XREADGROUP with timestamp | SELECT * WHERE timestamp BETWEEN $1 AND $2 |
| By Actor | XRANGE + filter | SELECT * WHERE actor_id = $1 |
| Failure Only | XREADGROUP + filter | SELECT * WHERE success = false |
| Recent Events | XREADGROUP last N | SELECT * ORDER BY timestamp DESC LIMIT N |

### Rationale

| Aspect | Redis Streams | PostgreSQL | Kafka | Cloud Logging |
|--------|--------------|-----------|-------|---------------|
| **Write Performance** | ✅ Very High | ⚠️ Medium | ✅ High | ✅ High |
| **Durability** | ⚠️ Configurable | ✅ Strong | ✅ Strong | ✅ Strong |
| **Query Capability** | ⚠️ Limited | ✅ Full SQL | ❌ None | ⚠️ Limited |
| **Export to SIEM** | ✅ Via consumer | ✅ Direct | ✅ Direct | ✅ Native |
| **Cost** | Low | Medium | Medium | High |
| **Complexity** | Medium | Low | High | Low |

---

## Consequences

### Positive
- Dual-write ensures both performance and durability
- Redis Streams provides high-throughput event capture
- PostgreSQL enables rich querying for investigations
- Standard JSON format enables SIEM integration
- Immutable audit trail for compliance

### Negative
- Additional infrastructure complexity
- Dual-write latency (~1-2ms overhead)
- Storage costs for both systems
- Schema evolution requires migration

### Mitigation
- Async dual-write to minimize latency impact
- Automatic expiration of Redis events after PostgreSQL write
- Regular archival of old audit data
- Versioned schema with backward compatibility

---

## References

- [Redis Streams Documentation](https://redis.io/docs/data-streams/)
- [PostgreSQL INSERT ON CONFLICT](https://www.postgresql.org/docs/current/sql-insert.html)
- [OWASP Security Logging](https://owasp.org/www-project-application-security-verification-standard/)
