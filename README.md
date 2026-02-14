# rust-crud-api

## Description
A Rust-based CRUD API that connects to a MySQL database using Axum and SQLx.

## Features
- **Health Check Endpoint**: GET `/health` to check the API status.
- **User Management**: Create and retrieve users via the `/users` endpoint.

## Technologies Used
- Rust
- Axum
- SQLx
- MySQL
- Serde for serialization
- Argon2 for password hashing

## Project Structure
```
rust-crud-api/
    Cargo.toml
    LICENSE
    README.md
    src/
        app.rs
        main.rs
        routes.rs
        seed.rs
        config/
            mod.rs
        db/
            mod.rs
            mysql.rs
            user_repository.rs
        errors/
            api_error.rs
            mod.rs
        handlers/
            health.rs
            mod.rs
            user_handler.rs
        migrations/
            20260209165552_create_users.sql
            20260209165729_create_departments.sql
            20260209165754_create_user_departments.sql
            20260209165828_seed_data.sql
        models/
            mod.rs
            user.rs
```

## Setup
1. Clone the repository.
2. Run `cargo build` to build the project.
3. Set up your MySQL database and update the configuration in `config/mod.rs`.
4. Run migrations to set up the database schema.
5. Start the server with `cargo run`.

---

# 📊 Performance experiments — Axum API

Esta sección documenta lo observado durante las pruebas de latencia y tiempo de respuesta de la API, junto con las mejoras aplicadas y las que se implementarán posteriormente.

## 1) Metodología de medición

Las mediciones se realizaron utilizando `curl` mostrando los tiempos detallados de la request:

```bash
curl -w "\nDNS: %{time_namelookup}s\
TCP: %{time_connect}s\
TLS: %{time_appconnect}s\
Pretransfer: %{time_pretransfer}s\
TTFB: %{time_starttransfer}s\
Total: %{time_total}s\n" \
-o /dev/null -s https://xxxx.ngrok-free.app/users
```

Para pruebas con compresión:

```bash
curl --compressed -w "\nDNS: %{time_namelookup}s\
TCP: %{time_connect}s\
TLS: %{time_appconnect}s\
Pretransfer: %{time_pretransfer}s\
TTFB: %{time_starttransfer}s\
Total: %{time_total}s\n" \
-o /dev/null -s https://xxxx.ngrok-free.app/users
```

Para pruebas locales:

```bash
curl -w "\nTTFB: %{time_starttransfer}s\
Total: %{time_total}s\n" \
-o /dev/null -s http://localhost:3000/users
```

## 2) Resultados obtenidos

### A) Antes de habilitar compresión (gzip)

El mayor tiempo se concentraba en la transferencia del payload (transfer + download).

Ejemplo representativo (~50 registros):

```
DNS: 0.285098s
TCP: 0.697037s
TLS: 1.012869s
Pretransfer: 1.014400s
TTFB: 1.819795s
Total: 1.820846s
```

**Observación clave**

El TTFB era prácticamente igual al tiempo total →
el backend respondía rápido, pero el tamaño de la respuesta dominaba la latencia.

### B) Después de habilitar gzip

Al activar compresión HTTP:

* Reducción importante del tiempo total
* Reducción significativa del TTFB percibido
* El cuello de botella dejó de ser el tamaño de la respuesta

**Conclusión:**

> El problema principal no era el procesamiento sino el peso del JSON.

### C) Pruebas en local (sin red ni TLS)

En local el tiempo bajó drásticamente:

* La base de datos responde rápido
* La serialización es rápida
* El overhead principal proviene de red + túnel HTTPS

**Conclusión:**

> La API es rápida; el mayor costo está fuera del servidor (red + TLS + túnel).

## 3) Análisis técnico

Del desglose de tiempos:

| Etapa         | Impacto                      |
| ------------- | ---------------------------- |
| DNS           | Propio del túnel             |
| TCP           | Handshake externo            |
| TLS           | Muy costoso en ngrok         |
| Backend       | Rápido                       |
| Transferencia | Dependía del tamaño del JSON |

**Lo aprendido:**

1. El backend no era el problema principal
2. El tamaño de respuesta afectaba más que la query
3. TLS remoto domina el tiempo total
4. Compresión fue la mejora más efectiva hasta ahora

## 4) Mejoras ya implementadas

* ✔ Compresión gzip en respuestas
* ✔ Validación de tiempos por capas (red vs backend)
* ✔ Confirmación de performance real en entorno local
* ✔ Comparación directa local vs túnel HTTPS

## 5) Próximas optimizaciones (plan de mejora)

### A) Optimización de payload

Reducir el tamaño de respuesta:

* DTOs específicos por endpoint
* Evitar columnas innecesarias
* Paginación obligatoria
* Evitar `SELECT *`
* Posible uso de `serde(skip_serializing_if)`

### B) Optimización de consultas SQL

* Índices en filtros frecuentes
* Evitar OFFSET grandes
* Implementar paginación por cursor (keyset pagination)

**Objetivo:**

> evitar escaneo completo de tabla

### C) Mejora de serialización

Opciones a evaluar:

* JSON más compacto
* MessagePack (evaluación futura)
* Streaming response para listas grandes

### D) Mejora de conexión

* Pool tuning
* Prepared statements cache
* Mantener conexiones calientes

### E) Reducción del impacto TLS

El túnel HTTPS agrega gran parte de la latencia.

Por lo tanto las métricas válidas de rendimiento backend serán:

> siempre las pruebas locales

Las pruebas vía túnel solo se usarán para latencia real de usuario.

## 6) Conclusión general

Actualmente:

* El backend es rápido
* La DB responde rápido
* El mayor costo está en red + TLS + tamaño de respuesta

Después de gzip:

> La performance dejó de depender del peso del JSON y pasó a depender principalmente de la red.

Las siguientes optimizaciones apuntan a:

1. Reducir tamaño lógico del payload
2. Optimizar paginación
3. Evitar scans completos
4. Preparar la API para datasets grandes

## Estado actual

* ✔ Compresión implementada
* ✔ Performance local validada
* ✔ Cuello de botella identificado

## Pendiente

* ⬜ Cursor pagination
* ⬜ Índices SQL
* ⬜ DTO optimizados
* ⬜ Serialización optimizada
* ⬜ Pool tuning

---

## License
This project is licensed under the MIT License.