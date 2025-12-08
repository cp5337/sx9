# 🏗️ CTAS-7 System Architecture & Data Flow Diagrams

## 🎯 **System Overview Diagram**

```mermaid
graph TD
    subgraph "Frontend Layer"
        CC[CTAS Command Center<br/>Dioxus EA Console<br/>Port 21575]
        MO[CTAS Main Ops<br/>React/Vite Platform<br/>Port 5173]
    end

    subgraph "Service Discovery Layer"
        SD[Service Discovery<br/>Registry & Coordination<br/>Port 18650]
    end

    subgraph "Backend Infrastructure"
        BM[Backend MCP Server<br/>Data Integrity & Watchdog<br/>Port 18600]
        DV[Database Validator<br/>MurmurHash3 Integrity<br/>Port 18605]
        ER[Emergency Recovery<br/>CTAS Task Protection<br/>Port 18615]
    end

    subgraph "AI/Model Layer"
        PG[Phi-3 Guardian<br/>Model Drift Prevention<br/>Port 11434]
        NM[Neural Mux<br/>gRPC Coordination<br/>Port 50051]
    end

    subgraph "Foundation Services"
        PM[Port Manager<br/>Authoritative Registry<br/>Port 18103]
        HE[Hash Engine<br/>Trivariate Processing<br/>Port 18105]
        FD[Foundation Daemon<br/>PM2 Replacement<br/>Multi-Modal]
    end

    subgraph "Data Storage"
        SR[SurrealDB<br/>Graph Database<br/>Port 8000]
        SC[Sledis Cache<br/>Memory Fabric<br/>Port 19014]
        AC[Atomic Clipboard<br/>Cross-System Memory<br/>Port 19012]
    end

    subgraph "Monitoring & Security"
        WD[Watchdog Dashboard<br/>Grafana Monitoring<br/>Port 18610]
        PL[PLASMA Security<br/>Threat Detection<br/>Integrated]
    end

    %% Frontend Connections
    CC --> SD
    MO --> SD
    CC --> BM
    MO --> BM

    %% Service Discovery Orchestration
    SD --> BM
    SD --> DV
    SD --> ER
    SD --> PG
    SD --> PM
    SD --> HE
    SD --> FD

    %% Backend Infrastructure Flow
    BM --> DV
    BM --> ER
    BM --> PG
    BM --> WD

    %% AI/Model Coordination
    PG --> NM
    NM --> HE
    NM --> FD

    %% Foundation Services Integration
    FD --> PM
    FD --> HE
    HE --> DV

    %% Data Storage Access
    BM --> SR
    BM --> SC
    DV --> AC
    ER --> AC

    %% Security Integration
    BM --> PL
    WD --> PL
    FD --> PL

    style CC fill:#e1f5fe
    style MO fill:#e8f5e8
    style SD fill:#fff3e0
    style BM fill:#fce4ec
    style PG fill:#f3e5f5
    style FD fill:#e0f2f1
```

---

## 🔄 **Data Flow Diagram - CTAS Command Center**

```mermaid
flowchart TD
    subgraph "CTAS Command Center (Port 21575)"
        UI[Dioxus UI Components]
        EAC[Enterprise Architecture Console]
        SC[Satellite Control Panel]
        HC[Hashing Engine Console]
    end

    subgraph "Data Processing Pipeline"
        DR[Data Requests]
        DV[Data Validation]
        DT[Data Transformation]
        DS[Data Storage]
    end

    subgraph "Backend Integration"
        BM[Backend MCP<br/>Port 18600]
        SD[Service Discovery<br/>Port 18650]
        HE[Hash Engine<br/>Port 18105]
    end

    subgraph "External Systems"
        FG[Figma Integration]
        LN[Linear Integration]
        OS[OrbStack Containers]
    end

    %% User Interaction Flow
    UI --> DR
    EAC --> DR
    SC --> DR
    HC --> DR

    %% Data Processing Flow
    DR --> DV
    DV --> DT
    DT --> DS

    %% Backend Communication Flow
    DR --> SD
    SD --> BM
    BM --> HE

    %% Response Flow
    HE --> BM
    BM --> SD
    SD --> DT
    DT --> UI

    %% External Integration Flow
    EAC --> FG
    EAC --> LN
    SC --> OS

    %% Data Types
    DR --> |"Commands, Tasks, Configs"| DV
    DV --> |"Integrity Validated"| DT
    DT --> |"Hash-Verified Data"| DS
    HE --> |"T1/T2 Hashes"| BM
    BM --> |"Secure Responses"| SD

    style UI fill:#e1f5fe
    style BM fill:#fce4ec
    style HE fill:#f3e5f5
    style SD fill:#fff3e0
```

---

## 🔄 **Data Flow Diagram - CTAS Main Ops**

```mermaid
flowchart TD
    subgraph "CTAS Main Ops (Port 5173)"
        OP[Operations Dashboard]
        SM[System Monitoring]
        TM[Task Management]
        RM[Resource Management]
    end

    subgraph "Operations Data Pipeline"
        OR[Operations Requests]
        OV[Operations Validation]
        OE[Operations Execution]
        OR2[Operations Response]
    end

    subgraph "Service Coordination"
        SD[Service Discovery<br/>Port 18650]
        FD[Foundation Daemon<br/>Multi-Modal]
        PM[Port Manager<br/>Port 18103]
    end

    subgraph "Monitoring & Control"
        WD[Watchdog System]
        ER[Emergency Recovery<br/>Port 18615]
        NM[Neural Mux<br/>Port 50051]
    end

    %% Operations Flow
    OP --> OR
    SM --> OR
    TM --> OR
    RM --> OR

    %% Processing Pipeline
    OR --> OV
    OV --> OE
    OE --> OR2
    OR2 --> OP

    %% Service Coordination Flow
    OR --> SD
    SD --> FD
    FD --> PM

    %% Monitoring Integration
    SM --> WD
    WD --> ER
    TM --> NM

    %% Response Flow
    PM --> FD
    FD --> SD
    SD --> OV
    NM --> OE
    ER --> OR2

    %% Data Classifications
    OR --> |"Operational Tasks"| OV
    OV --> |"Namespace Isolated"| OE
    OE --> |"Executed Commands"| OR2
    WD --> |"Health Status"| SM
    ER --> |"Recovery Actions"| OR2

    style OP fill:#e8f5e8
    style FD fill:#e0f2f1
    style WD fill:#fce4ec
    style SD fill:#fff3e0
```

---

## 🛡️ **Data Isolation & Security Flow**

```mermaid
flowchart TD
    subgraph "Data Namespaces"
        CN[CTAS Operational<br/>Namespace]
        TN[Threat Intel<br/>Namespace]
        NN[Neural Model<br/>Namespace]
        FN[Foundation<br/>Namespace]
    end

    subgraph "Isolation Barriers"
        IB[Isolation Barriers<br/>Namespace Protection]
        DV[Data Validator<br/>Pattern Detection]
        IH[Integrity Hashing<br/>MurmurHash3]
    end

    subgraph "Watchdog System"
        WM[Watchdog Monitor]
        VD[Violation Detection]
        AR[Auto Remediation]
        ER[Emergency Recovery]
    end

    subgraph "Model Protection"
        PG[Phi-3 Guardian]
        MD[Model Drift Detection]
        BC[Baseline Comparison]
    end

    %% Data Flow Through Barriers
    CN --> IB
    TN --> IB
    NN --> IB
    FN --> IB

    %% Validation Process
    IB --> DV
    DV --> IH

    %% Watchdog Monitoring
    IH --> WM
    WM --> VD
    VD --> AR
    AR --> ER

    %% Model Protection Flow
    NN --> PG
    PG --> MD
    MD --> BC
    BC --> VD

    %% Critical Protection Paths
    CN -.->|"BLOCKED"| TN
    TN -.->|"BLOCKED"| CN
    VD --> |"Contamination Alert"| ER
    MD --> |"Drift Alert"| AR

    %% Data Types
    CN --> |"Tasks, Commands, Status"| IB
    TN --> |"Indicators, Intelligence"| IB
    IB --> |"Validated Data"| DV
    DV --> |"Clean Data"| IH
    VD --> |"Security Events"| AR

    style CN fill:#e8f5e8
    style TN fill:#ffebee
    style IB fill:#fff3e0
    style WM fill:#fce4ec
    style PG fill:#f3e5f5
```

---

## 🔄 **Service Discovery & Coordination Flow**

```mermaid
flowchart TD
    subgraph "Service Registration"
        SR[Service Registration<br/>Port 18650]
        SS[Singleton Check]
        PC[Port Conflict Check]
        RG[Registry Update]
    end

    subgraph "Health Monitoring"
        HB[Heartbeat Monitor]
        TO[Timeout Detection]
        SC[Service Cleanup]
        ST[Status Tracking]
    end

    subgraph "Service Types"
        CC[Command Center<br/>Frontend]
        MO[Main Ops<br/>Frontend]
        BM[Backend MCP<br/>Infrastructure]
        PG[Phi-3 Guardian<br/>AI Model]
        FD[Foundation Daemon<br/>Process Manager]
    end

    subgraph "Coordination Bridges"
        SB[Singleton Bridge]
        LB[Load Balance Bridge]
        FB[Failover Bridge]
        SEM[Semantic Bridge<br/>Future]
    end

    %% Registration Flow
    CC --> SR
    MO --> SR
    BM --> SR
    PG --> SR
    FD --> SR

    %% Validation Process
    SR --> SS
    SS --> PC
    PC --> RG

    %% Health Monitoring Flow
    RG --> HB
    HB --> TO
    TO --> SC
    SC --> ST

    %% Bridge Coordination
    SS --> SB
    RG --> LB
    ST --> FB
    FB --> SEM

    %% Data Classifications
    SR --> |"Service Metadata"| SS
    SS --> |"Singleton Rules"| PC
    PC --> |"Port Allocations"| RG
    HB --> |"Health Status"| TO
    SB --> |"Single Instance"| PG

    style SR fill:#fff3e0
    style SS fill:#fce4ec
    style SB fill:#e0f2f1
    style SEM fill:#f3e5f5
```

---

## 📊 **Port Allocation & Network Architecture**

```mermaid
graph TD
    subgraph "Frontend Ports (5000-8999)"
        P1[Command Center: 21575]
        P2[Main Ops: 5173]
        P3[SurrealDB: 8000]
    end

    subgraph "Service Discovery (18650)"
        P4[Service Registry: 18650]
    end

    subgraph "Backend Services (18600-18699)"
        P5[Backend MCP: 18600]
        P6[Database Validator: 18605]
        P7[Watchdog Dashboard: 18610]
        P8[Emergency Recovery: 18615]
    end

    subgraph "Foundation Services (18100-18199)"
        P9[Port Manager: 18103]
        P10[Hash Engine: 18105]
    end

    subgraph "AI/Neural (11434, 50051)"
        P11[Phi-3 Guardian: 11434]
        P12[Neural Mux: 50051]
    end

    subgraph "Memory Fabric (19000-19999)"
        P13[Sledis Cache: 19014]
        P14[Atomic Clipboard: 19012]
    end

    %% Network Flow
    P1 --> P4
    P2 --> P4
    P4 --> P5
    P5 --> P6
    P5 --> P8
    P6 --> P10
    P5 --> P11
    P11 --> P12
    P5 --> P13
    P6 --> P14

    style P1 fill:#e1f5fe
    style P2 fill:#e8f5e8
    style P4 fill:#fff3e0
    style P5 fill:#fce4ec
    style P11 fill:#f3e5f5
```

---

## 🔍 **Emergency Recovery Data Flow**

```mermaid
flowchart TD
    subgraph "Detection Phase"
        CC[Contamination Detection<br/>IED TTL in CTAS Space]
        VL[Violation Logging]
        AS[Alert System]
    end

    subgraph "Isolation Phase"
        NS[Namespace Shutdown]
        DI[Data Isolation]
        SI[Service Isolation]
    end

    subgraph "Recovery Phase"
        SR[Snapshot Retrieval]
        DV[Data Validation]
        IV[Integrity Verification]
        SS[Service Restart]
    end

    subgraph "Verification Phase"
        HV[Hash Verification]
        CT[Contamination Test]
        MV[Model Validation]
        GO[Go/No-Go Decision]
    end

    %% Detection Flow
    CC --> VL
    VL --> AS

    %% Isolation Flow
    AS --> NS
    NS --> DI
    DI --> SI

    %% Recovery Flow
    SI --> SR
    SR --> DV
    DV --> IV
    IV --> SS

    %% Verification Flow
    SS --> HV
    HV --> CT
    CT --> MV
    MV --> GO

    %% Critical Paths
    CC --> |"IMMEDIATE"| AS
    AS --> |"EMERGENCY"| NS
    GO --> |"SAFE TO PROCEED"| SS
    CT -.->|"CONTAMINATION FOUND"| NS

    style CC fill:#ffebee
    style AS fill:#fce4ec
    style GO fill:#e8f5e8
```

---

## 🎯 **Key Integration Points**

### **CTAS Command Center Integration:**
- **Service Discovery Registration** → Prevents duplicate EA consoles
- **Backend MCP Communication** → Secure model context management
- **Hash Engine Integration** → T1/T2 trivariate hash generation
- **Emergency Recovery** → Protects satellite control configurations

### **CTAS Main Ops Integration:**
- **Foundation Daemon Coordination** → Replaces PM2 with enterprise orchestration
- **Watchdog Monitoring** → Real-time operational health tracking
- **Neural Mux Communication** → AI agent coordination
- **Port Manager Integration** → Authoritative port allocation

### **Critical Data Protection:**
- **Namespace Isolation** → CTAS operational vs threat intelligence separation
- **Model Drift Prevention** → Phi-3 baseline protection
- **Emergency Recovery** → Automatic CTAS task restoration
- **Semantic Classification** → Future ontological routing capability

This architecture ensures **no service doubling**, **proper network bridging**, and **critical data protection** while maintaining **high-performance operation** and **enterprise-grade reliability**.