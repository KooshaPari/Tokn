# ADR-011: gRPC Internal Communication Protocol

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

Tokn requires efficient internal communication between services. We need:
- Low-latency token operations
- Bidirectional streaming for events
- Strong typing and code generation
- Backward compatibility support

---

## Decision

We will use **gRPC with Protocol Buffers** for all internal service communication.

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    gRPC Service Communication                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                      Service Mesh                                     │  │
│  │                                                                      │  │
│  │   ┌─────────────┐         ┌─────────────┐         ┌─────────────┐  │  │
│  │   │   Gateway   │────────►│  Token Svc  │────────►│   Storage   │  │  │
│  │   │   (REST)    │         │  (gRPC)     │         │   Adapter   │  │  │
│  │   └─────────────┘         └─────────────┘         └─────────────┘  │  │
│  │         │                        │                        │           │  │
│  │         │                        │                        │           │  │
│  │         ▼                        ▼                        ▼           │  │
│  │   ┌─────────────┐         ┌─────────────┐         ┌─────────────┐  │  │
│  │   │   Audit     │         │    Rate     │         │    Cache    │  │  │
│  │   │   Service   │         │   Limiter   │         │   Service   │  │  │
│  │   └─────────────┘         └─────────────┘         └─────────────┘  │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  Protocol Stack:                                                             │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │   Layer 7: gRPC / Protocol Buffers                                   │  │
│  │   Layer 4: HTTP/2                                                    │  │
│  │   Layer 3: TLS 1.3                                                   │  │
│  │   Layer 2: TCP                                                       │  │
│  │   Layer 1: Physical Network                                          │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Service Definition

```protobuf
syntax = "proto3";

package tokn.v1;

service TokenService {
  rpc IssueToken(IssueTokenRequest) returns (IssueTokenResponse);
  rpc ValidateToken(ValidateTokenRequest) returns (ValidateTokenResponse);
  rpc RevokeToken(RevokeTokenRequest) returns (RevokeTokenResponse);
  rpc RefreshToken(RefreshTokenRequest) returns (RefreshTokenResponse);
  
  // Streaming for events
  rpc StreamTokenEvents(StreamEventsRequest) returns (stream TokenEvent);
}

message IssueTokenRequest {
  string subject = 1;
  repeated string audience = 2;
  repeated string scopes = 3;
  map<string, string> custom_claims = 4;
  int64 ttl_seconds = 5;
  TokenFormat format = 6;
  string tenant_id = 7;
}

message IssueTokenResponse {
  string token = 1;
  string jti = 2;
  int64 issued_at = 3;
  int64 expires_at = 4;
  string refresh_token = 5;
}

message ValidateTokenRequest {
  string token = 1;
  string expected_audience = 2;
  repeated string required_scopes = 3;
  bool check_revocation = 4;
}

message ValidateTokenResponse {
  bool valid = 1;
  Claims claims = 2;
  string error = 3;
}

message RevokeTokenRequest {
  oneof target {
    string jti = 1;
    string subject = 2;
  }
  string reason = 3;
}

message TokenEvent {
  string event_type = 1;
  string jti = 2;
  string subject = 3;
  int64 timestamp = 4;
  map<string, string> metadata = 5;
}
```

### Performance Comparison

| Aspect | REST/JSON | gRPC/Protobuf | Improvement |
|--------|-----------|---------------|-------------|
| **Serialization** | JSON text | Binary protobuf | 3-10x smaller |
| **Latency** | ~5ms | ~1ms | 5x faster |
| **Throughput** | 10K req/s | 100K req/s | 10x higher |
| **Code Gen** | None | Strong types | Type safety |
| **Streaming** | SSE/WebSocket | Native bidirectional | Native support |

---

## Consequences

### Positive
- High-performance binary protocol
- Strong typing with code generation
- Native streaming support
- Backward compatible schema evolution
- Wide language support

### Negative
- Requires HTTP/2 (usually supported)
- Debugging more complex than REST
- Browser support limited (requires gRPC-Web)
- Additional tooling required

### Mitigation
- Provide REST gateway for external access
- Use gRPC reflection for debugging
- Document gRPC-Web for browser clients
- Invest in observability tooling

---

## References

- [gRPC Documentation](https://grpc.io/docs/)
- [Protocol Buffers](https://developers.google.com/protocol-buffers)
- [gRPC Health Check Protocol](https://grpc.io/grpc/health/v1/)
