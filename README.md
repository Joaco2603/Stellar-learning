# Roadmap: OpenZeppelin, arquitectura de smart contracts y experto en Stellar / Soroban

Programa práctico basado en **proyectos**, no en leer documentación de punta a punta. Objetivo: diseñar contratos seguros y modulares (EVM + principios) y protocolos reales sobre Stellar.

## Stack principal

| Área | Herramientas |
|------|----------------|
| Soroban | Rust + Soroban SDK |
| Cliente Stellar | Stellar SDK (TypeScript) |
| Red | Stellar RPC + Horizon (según operación) |
| Patrones EVM | OpenZeppelin Contracts 5.x, Foundry + Solidity |
| Apps | Next.js + NestJS |

**Distinción importante:** OpenZeppelin está pensado para **EVM**. En Soroban aplicas los **principios** (acceso, invariantes, separación de responsabilidades), no copias proxies ni APIs de Solidity tal cual.

---

## Parte A — OpenZeppelin + arquitectura EVM (6 semanas)

### Objetivo

Diseñar contratos seguros, modulares y auditables con patrones probados.

### Fase 1 — Fundamentos (semana 1)

**Estudiar:** ERC-20, ERC-721, ERC-1155 (EIPs).

**Practicar:**

1. Crear un ERC-20 con [OpenZeppelin Wizard](https://wizard.openzeppelin.com).
2. Leer el código generado: herencia, hooks internos (en OZ 5.x revisar APIs actuales; evitar tutoriales obsoletos con `_beforeTokenTransfer` si ya no aplica).

### Fase 2 — Seguridad esencial (semana 2)

| Módulo | Para qué |
|--------|----------|
| **Ownable** | Control de acceso básico |
| **AccessControl** | RBAC granular |
| **ReentrancyGuard** | Anti reentrancy (contexto: The DAO, 2016) |
| **Pausable** | Parada de emergencia |

**Ejercicio:** contrato donde solo admin pausa, usuarios depositan y hay protección contra reentrancy.

### Fase 3 — Arquitectura profesional (semanas 3–4)

**Capas (referencia proxy / upgradeable):**

1. **Storage** — solo estado  
2. **Logic** — funciones de negocio  
3. **Proxy** — upgrades (Transparent Proxy, OpenZeppelin)

**Estructura modular (EVM):**

```text
contracts/
 ├── core/          # Token.sol, Treasury.sol
 ├── access/        # Roles.sol
 ├── security/      # Guards.sol
 └── interfaces/    # IToken.sol
```

**Patrones:**

- Checks-Effects-Interactions  
- Pull over Push  
- Contratos upgradeables (proxies)

### Fase 4 — Upgradeables (semana 5)

- `@openzeppelin/contracts-upgradeable`  
- Transparent vs UUPS (gas, seguridad, uso)

| Proxy | Gas | Seguridad | Uso |
|-------|-----|-----------|-----|
| Transparent | Alto | Muy alta | Estándar |
| UUPS | Bajo | Alta | Moderno |

### Fase 5 — Testing y auditoría (semana 6)

**Herramientas:** Hardhat, Foundry, Slither, Mythril.

**Testear:** roles, reentrancy, límites, pausable, upgradeability.

**Proyecto integrador EVM:** mini protocolo DeFi (ERC20, Treasury, roles, Pausable, ReentrancyGuard, proxy upgradeable).

### Niveles resumidos (EVM)

| Nivel | Contenido |
|-------|-----------|
| 1 — Básico | ERC, Ownable, Pausable |
| 2 — Intermedio | AccessControl, ReentrancyGuard, patrones de seguridad |
| 3 — Avanzado | Proxies, arquitectura modular, auditoría |

Referencias de industria: Uniswap, Aave, Compound (mismas ideas de capas, permisos y upgrades).

---

## Parte B — Experto Stellar / Soroban (12 semanas, por proyectos)

### Tabla de proyectos

| Semana | Proyecto | Conceptos | Dificultad |
|--------|----------|-----------|------------|
| 1 | Token Factory | Assets, SAC, emisión, permisos | 3/10 |
| 2 | Multisig Treasury | Auth, firmas, roles | 5/10 |
| 3 | Escrow Marketplace | Estado, almacenamiento, seguridad | 6/10 |
| 4 | Subscription Protocol | TTL, estados, off-chain | 6/10 |
| 5 | Staking Protocol | Recompensas, invariantes | 7/10 |
| 6 | DEX AMM | Liquidez, swaps, slippage, \(x \cdot y = k\) | 8/10 |
| 7 | Lending Protocol | Colateral, liquidaciones, oráculo | 9/10 |
| 8 | Payment Router | Payments, path payments, trustlines | 7/10 |
| 9 | Cross-Contract Protocol | Composición, interfaces | 8/10 |
| 10 | Security Lab | Fuzzing, pruebas adversariales | 9/10 |
| 11 | Upgrade & Migration | Versionado, migraciones | 9/10 |
| 12 | DeFi completo | Arquitectura de producción | 10/10 |

### Secuencia recomendada si ya programas en Stellar

1. Multisig Treasury → 2. Escrow → 3. DEX → 4. Lending → 5. Auditoría.

**Regla:** cada proyecto debe incluir al menos una vulnerabilidad que descubras con pruebas y corrijas antes de darlo por cerrado.

---

### Proyectos (detalle)

#### 1 — Token Factory

- Activos clásicos, SAC, mint/burn, permisos, transferencias, freeze/revoke cuando aplique.  
- **Reto:** comparar activo vía SAC vs token Soroban puro y explicar por qué no son equivalentes.

#### 2 — Multisig Treasury

- 5 admins, 3 firmas, propuestas con ID, anti doble ejecución, caducidad, eventos.  
- **Reto:** versión con multisig nativo Stellar vs tesorería Soroban con `require_auth`.

#### 3 — Escrow Marketplace

```text
Marketplace (Next.js) → NestJS → Stellar RPC → Escrow → Storage / Token
```

Acuerdos, depósito, liberación, cancelación, disputas, reembolsos.  
**Reto:** intentar romperlo (llamadas no autorizadas, estados inválidos, replay).

#### 4 — Subscription Protocol

Planes, cobros recurrentes, cancelación, gracia, registro; renovaciones vía **servicio externo** (Soroban no corre solo con el tiempo).  
**Reto:** autorización de cobros limitada sin retiros arbitrarios del proveedor.

#### 5 — Staking Protocol

Depósitos, retiros, recompensas, lock, proporcionalidad, fondo de rewards.  
**Reto:** muchos usuarios entrando/saliendo en distintos momentos sin exceder fondos.

#### 6 — DEX AMM

Pools, swaps, fees, retiro de liquidez, slippage; invariante \(x \cdot y = k\).  
**Reto:** pruebas que ningún swap extrae más de lo permitido (fees y redondeo).

#### 7 — Lending Protocol

Colateral, préstamos, interés, health factor, liquidaciones, oráculo.  
**Reto:** caída ~40 % del colateral. Mantener en testnet/local hasta revisión de seguridad seria.

---

### Arquitectura Soroban (referencia)

```text
stellar-defi/
├── contracts/
│   ├── treasury/   # lib, storage, errors, events
│   ├── vault/
│   ├── lending/
│   └── oracle/
├── packages/         # interfaces, shared-types, test-utils
├── apps/             # frontend, indexer
└── tests/            # integration, security
```

| Aspecto | Qué dominar |
|---------|-------------|
| Contract architecture | Responsabilidades, interfaces, composición |
| Storage | Instance, Persistent, Temporary, **TTL** |
| Authorization | `require_auth`, `require_auth_for_args`, auth entre contratos |
| Resources | CPU, memoria, lecturas/escrituras, tamaño WASM |
| Security | Invariantes, validación, errores, permisos, pruebas adversariales |

---

### OpenZeppelin en paralelo (1 sesión/semana en Solidity)

| Proyecto Stellar | Equivalente OZ | Aprendizaje |
|------------------|----------------|-------------|
| Token Factory | ERC20, AccessControl | Tokens y permisos |
| Treasury | AccessControl, Ownable | Admin |
| Escrow | ReentrancyGuard, Pausable | Fondos |
| Staking | SafeERC20, Math | Transferencias y math |
| Vault | UUPSUpgradeable | Upgrades |
| (opcional) | Governor, Timelock | Gobernanza |

Usar **OpenZeppelin Contracts 5.x**.

---

### Más allá de Soroban (6 áreas)

| Área | Proyecto mini |
|------|----------------|
| A. Ledger | Explorador básico: cuentas, ops, assets, trustlines, entries |
| B. SCP | Simulador simplificado de federated voting / quórum |
| C. Payments | Router: pagos directos, path payments, simulación |
| D. Runtime | Benchmark WASM: CPU, memoria, storage, invocaciones |
| E. Seguridad | Lab de contratos vulnerables + test que reproduce cada fallo |
| F. Infra | Indexador de eventos → PostgreSQL → API |

---

### Metodología por proyecto (7 días)

| Día | Actividad |
|-----|-----------|
| 1 | Diseño: problema, estados, permisos, invariantes |
| 2–3 | Implementación sin depender de tutoriales completos |
| 4 | Tests unitarios, integración, adversariales |
| 5 | Buscar vulnerabilidades y corregir |
| 6 | Cliente TypeScript + RPC |
| 7 | Documentar decisiones y riesgos |

### Criterios de “terminado”

- Pruebas en operaciones críticas  
- Permisos explícitos  
- Invariantes comprobadas  
- Errores con comportamiento definido  
- Ejecución desde cliente real  
- Local y/o testnet  
- Puedes explicar riesgos principales  

---

### Proyecto final — Stellar DeFi Protocol

```text
Next.js → NestJS → Stellar RPC → Protocol Router
                    ├── DEX
                    ├── Lending
                    └── Treasury
                              ↓
                        Token layer / Stellar Assets
```

Exigir: propiedades/invariantes, escenarios extremos, métricas de recursos, migración, modelo de amenazas, arquitectura documentada, testnet.

---

### Recursos (consulta bajo demanda)

| Recurso | Uso |
|---------|-----|
| [Stellar Developers](https://developers.stellar.org/) | SDK, RPC, arquitectura |
| Soroban Examples | Referencia |
| Soroban SDK (Rust) | API de contratos |
| [OpenZeppelin Contracts](https://docs.openzeppelin.com/contracts/) | Patrones EVM |
| [OpenZeppelin Wizard](https://wizard.openzeppelin.com) | Scaffold ERC |
| Foundry | Solidity + tests |

---

## Hoy — mini proyectos (una sesión)

Si quieres **avanzar hoy** sin abrir otro curso, elige una pista (2–4 h cada una) o combina **mañana EVM + tarde Stellar**.

### Pista 1 — EVM en 2 h: “Vault con guardas”

**Objetivo:** Ownable + Pausable + ReentrancyGuard + depósito/retiro (pull).

1. `forge init oz-mini-vault && cd oz-mini-vault`  
2. Instalar OZ 5.x.  
3. Contrato: `deposit()`, `withdraw()` con CEI y `nonReentrant`.  
4. Tests: usuario retira; no retira más del balance; pausado bloquea; solo owner pausa.

**Entregable:** repo con tests verdes y 3 líneas en un `NOTES.md` explicando CEI en tu `withdraw`.

### Pista 2 — Soroban en 2 h: “Caja con auth”

**Objetivo:** `require_auth`, storage mínimo, invariante de balance.

1. `soroban contract init` (o clonar un ejemplo mínimo del repo oficial).  
2. Funciones: `deposit(amount)`, `withdraw(amount)` con auth del caller.  
3. Test en Rust: depositar, retirar de más debe fallar, retirar sin firma debe fallar.

**Entregable:** test que demuestra un intento de robo (withdraw de otro usuario) fallando.

### Pista 3 — Stellar clásico en 1,5 h: “Multisig en papel”

**Objetivo:** entender thresholds antes del Proyecto 2.

1. En testnet: cuenta con 3 signers, threshold 2.  
2. Script TS: crear propuesta (pago), firmar con 1 → falla; con 2 → pasa.  
3. Anota diferencia vs “multisig en contrato Soroban”.

**Entregable:** diagrama en `NOTES.md` (3 cajas: signers, threshold, operación).

### Pista 4 — Seguridad en 1 h: “Un bug deliberado”

1. Copia tu contrato más simple (EVM o Soroban).  
2. Introduce **un** bug (p. ej. no validar caller, o redondeo que favorece al atacante).  
3. Escribe **un** test que lo explota.  
4. Corrige y deja el test como regresión.

**Entregable:** commit o nota con “bug → test rojo → fix → test verde”.

### Qué aprender **hoy** (checklist)

- [ ] **CEI** en una función que mueve valor (EVM o lógica equivalente en Soroban).  
- [ ] **`require_auth`** (o equivalente) en cada función que cambia balance de un usuario.  
- [ ] **Una invariante** escrita en prosa: “la suma de balances de usuarios ≤ balance del contrato”.  
- [ ] **Un test adversarial** que falle antes del fix.

### Mañana (siguiente sesión)

Empieza **Multisig Treasury** (semana 2 del roadmap): primero versión Soroban mínima (propuesta + 3 approvals), en paralelo opcional Treasury con `AccessControl` en Foundry el mismo fin de semana.

---

## Cierre

El objetivo no es “saber el SDK”. Es **diseñar un protocolo**, justificar arquitectura, anticipar fallos y **demostrar con pruebas** que las propiedades críticas se mantienen. La experiencia de producción suma: revisar código ajeno, incidentes reales, operar en testnet/mainnet y participar en auditorías.

Este repositorio (`stellar-portfolio`) puede alojar cada mini proyecto en carpetas `week-01-token-factory`, `mini/oz-vault`, etc., según vayas avanzando.
# Stellar-learning
