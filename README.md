# quack-api
axum + tonic backend API for Micro Duck

## Run (mock hardware)

```sh
cargo run -p quack-api
```

| Surface | Default address | Override | Contract |
| --- | --- | --- | --- |
| REST | 127.0.0.1:8080 | `QUACK_REST_ADDR` | `openapi/quack-api.yaml` |
| WebSocket telemetry (`/telemetry`, 50 Hz) | 127.0.0.1:8081 | `QUACK_WS_ADDR` | `TelemetryFrame` in the spec |
| gRPC control | 127.0.0.1:50051 | `QUACK_GRPC_ADDR` | `quack-proto/proto/quack/control/v1/control.proto` |
