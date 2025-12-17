# RFC-9022: OODA as Vertical Escalation Trigger

**Status:** Canonical
**Author:** Charlie Payne
**Version:** 7.3.1
**Related:** RFC-9020 (HD4), RFC-9021 (Convergence)

---

## Abstract

OODA loops serve as **escalatory triggers** for vertical context shifts in the HD4 framework. When an OODA loop completes at one level, it may trigger initiation of an OODA loop at the next vertical level. This creates nested, synchronized decision cycles from tactical to national scope.

---

## 1. The Vertical Problem

HD4 phases (Hunt, Detect, Disable, Disrupt, Dominate) apply at every level:

| Level | Scope | Example |
|-------|-------|---------|
| Tactical | Device/Node | This endpoint is compromised |
| Operational | Network/Org | This campaign targets our network |
| Strategic | Sector/Region | This APT targets critical infrastructure |
| National | Coalition/Global | This is a nation-state operation |

**Problem:** When does a tactical situation require operational attention? When does operational require strategic?

**Answer:** OODA completion triggers evaluation for escalation.

---

## 2. OODA is the Trigger Mechanism

### 2.1 Single OODA Loop

```
    ┌──────────────────────────────────────┐
    │                                      │
    ▼                                      │
┌───────┐    ┌────────┐    ┌────────┐    ┌─────┐
│Observe│ →  │ Orient │ →  │ Decide │ →  │ Act │
└───────┘    └────────┘    └────────┘    └─────┘
    │             │             │           │
    │             │             │           │
Collection   Convergence    Above line?   Execute OR
             Calculation    Choose action  Escalate
```

### 2.2 Act Phase Outcomes

The "Act" phase has three possible outcomes:

1. **Execute** - Take action at current level (HD4 phase)
2. **Hunt** - Need more information, cycle back to Observe
3. **Escalate** - Situation exceeds current level scope

---

## 3. Escalation Criteria

### 3.1 Tactical → Operational

| Indicator | Threshold | Escalation |
|-----------|-----------|------------|
| Affected nodes | > 1 | Yes |
| Attack type | Lateral movement detected | Yes |
| Asset criticality | Crown jewel touched | Yes |
| Dwell time | > 24 hours | Yes |

### 3.2 Operational → Strategic

| Indicator | Threshold | Escalation |
|-----------|-----------|------------|
| Campaign scope | Multiple organizations | Yes |
| TTP match | Known APT playbook | Yes |
| Sector targeting | Critical infrastructure | Yes |
| Attribution confidence | > 70% nation-state | Yes |

### 3.3 Strategic → National

| Indicator | Threshold | Escalation |
|-----------|-----------|------------|
| Sector impact | Multiple CI sectors | Yes |
| Attack type | Destructive/ICS | Yes |
| Political timing | Election/summit | Yes |
| Allied involvement | Coalition partner hit | Yes |

---

## 4. Nested OODA Implementation

### 4.1 Structure

```
NATIONAL OODA
    │
    ├── STRATEGIC OODA
    │       │
    │       ├── OPERATIONAL OODA
    │       │       │
    │       │       ├── TACTICAL OODA (Device A)
    │       │       ├── TACTICAL OODA (Device B)
    │       │       └── TACTICAL OODA (Device C)
    │       │
    │       └── OPERATIONAL OODA (Another org)
    │
    └── STRATEGIC OODA (Another sector)
```

### 4.2 Synchronization

Each level's OODA runs at different speeds:

| Level | Cycle Time | Collection Sources |
|-------|------------|-------------------|
| Tactical | Seconds-minutes | Endpoint telemetry, SIEM alerts |
| Operational | Hours | Threat intel, campaign analysis |
| Strategic | Days | Sector reports, ISAC feeds |
| National | Weeks | IC products, diplomatic channels |

### 4.3 Upward Flow

When tactical OODA escalates:

```rust
fn tactical_act(decision: TacticalDecision) -> TacticalOutcome {
    match decision {
        TacticalDecision::Contain => isolate_endpoint(),
        TacticalDecision::Hunt => return TacticalOutcome::CycleBack,
        TacticalDecision::Escalate(reason) => {
            // Push to operational OODA's Observe queue
            operational_queue.push(EscalationPacket {
                source_level: Level::Tactical,
                reason,
                context: gather_tactical_context(),
                timestamp: now(),
            });
            return TacticalOutcome::Escalated;
        }
    }
}
```

### 4.4 Downward Flow

Higher levels can task lower levels:

```rust
fn strategic_act(decision: StrategicDecision) -> StrategicOutcome {
    match decision {
        StrategicDecision::TaskCollection(targets) => {
            // Push requirements to operational level
            for target in targets {
                operational_queue.push(CollectionTasking {
                    source_level: Level::Strategic,
                    target,
                    priority: Priority::High,
                    deadline: now() + Duration::hours(6),
                });
            }
        }
        // ...
    }
}
```

---

## 5. OODA + HD4 Integration

### 5.1 OODA Phases Map to HD4

| OODA Phase | HD4 Phase | Activity |
|------------|-----------|----------|
| Observe | Hunt | Collection, monitoring |
| Orient | Detect | Convergence calculation, pattern match |
| Decide | - | Above threshold? Which HD4 action? |
| Act | Disable/Disrupt/Dominate | Execute offensive action |

### 5.2 The Orient Phase IS Convergence

```
ORIENT = Calculate H1 + H2 convergence scores

If H1 + H2 > threshold:
    DECIDE → Which HD4 action?
Else:
    DECIDE → Hunt more (back to Observe)
```

### 5.3 Phase Transitions

```rust
fn ooda_orient(observations: &[Observation]) -> OrientResult {
    let h1 = calculate_operational_convergence(observations);
    let h2 = calculate_semantic_convergence(observations);

    OrientResult {
        h1_score: h1,
        h2_score: h2,
        above_threshold: h1 >= threshold && h2 >= threshold,
        hd4_recommendation: if h1 >= threshold && h2 >= threshold {
            recommend_hd4_phase(h1, h2)
        } else {
            HD4Phase::Hunt // Need more
        },
        escalation_indicators: check_escalation_criteria(observations),
    }
}
```

---

## 6. 1n/2n Perspective in OODA

### 6.1 Dual OODA Tracks

At every level, run BOTH perspectives:

**1n Track (Defender):**
```
Observe: What is the adversary doing to us?
Orient:  How does this fit known attack patterns?
Decide:  Defend, hunt, or escalate?
Act:     Execute defensive HD4 phase
```

**2n Track (Adversary Emulation):**
```
Observe: What would I see if I were the attacker?
Orient:  What would my next move be?
Decide:  Where are my defensive gaps?
Act:     Preemptive hardening or deception
```

### 6.2 The Weave

```
1n OODA: Observe → Orient → Decide → Act
              ↕        ↕        ↕        ↕
2n OODA: Observe → Orient → Decide → Act

Constant cross-reference between perspectives
```

---

## 7. Relative Superiority

### 7.1 McRaven's Principle

Relative superiority = operating inside the adversary's decision cycle

### 7.2 Applied to Nested OODA

You achieve relative superiority when:

```
Your_OODA_Speed > Their_OODA_Speed

At EVERY level simultaneously:
  Your_Tactical   > Their_Tactical
  Your_Operational > Their_Operational
  Your_Strategic   > Their_Strategic
```

### 7.3 The Window

When YOUR convergence exceeds THEIR convergence = **action window**

```rust
fn detect_relative_superiority(
    our_state: &OODAState,
    estimated_adversary_state: &OODAState,
) -> Option<ActionWindow> {
    let our_convergence = our_state.h1 + our_state.h2;
    let their_convergence = estimated_adversary_state.h1 + estimated_adversary_state.h2;

    if our_convergence > their_convergence {
        Some(ActionWindow {
            advantage: our_convergence - their_convergence,
            recommended_action: HD4Phase::Dominate,
            window_estimate: estimate_adversary_catchup_time(their_convergence),
        })
    } else {
        None // They're inside our cycle - defensive posture
    }
}
```

---

## 8. Implementation

### 8.1 Data Structure

```rust
pub struct OODALoop {
    level: VerticalLevel,
    phase: OODAPhase,
    observations: Vec<Observation>,
    convergence: ConvergenceState,
    hd4_phase: HD4Phase,
    perspective: Perspective, // 1n or 2n
    parent: Option<Arc<OODALoop>>, // Higher level
    children: Vec<Arc<OODALoop>>, // Lower levels
}

pub enum OODAPhase {
    Observe,
    Orient,
    Decide,
    Act,
}

pub enum VerticalLevel {
    Tactical,
    Operational,
    Strategic,
    National,
}
```

### 8.2 Cycle Execution

```rust
impl OODALoop {
    pub async fn cycle(&mut self) -> OODAOutcome {
        // OBSERVE
        self.phase = OODAPhase::Observe;
        let observations = self.collect_observations().await;
        self.observations.extend(observations);

        // ORIENT
        self.phase = OODAPhase::Orient;
        self.convergence = self.calculate_convergence(&self.observations);

        // DECIDE
        self.phase = OODAPhase::Decide;
        let decision = if self.convergence.above_threshold() {
            self.decide_action()
        } else {
            Decision::HuntMore
        };

        // Check escalation
        if self.should_escalate(&self.observations) {
            return OODAOutcome::Escalate(self.build_escalation_packet());
        }

        // ACT
        self.phase = OODAPhase::Act;
        match decision {
            Decision::Execute(hd4_phase) => {
                self.hd4_phase = hd4_phase;
                self.execute_hd4_phase().await
            }
            Decision::HuntMore => OODAOutcome::CycleBack,
            Decision::Escalate(reason) => OODAOutcome::Escalate(reason),
        }
    }
}
```

---

## 9. References

- RFC-9020: HD4 Framework
- RFC-9021: Graph Convergence Theory
- Boyd, J. "Patterns of Conflict" (1986)
- McRaven, W. "The Theory of Special Operations" (NPS, 1993)
- Joint Publication 3-0, Joint Operations

---

## 10. Related RFCs

- RFC-9020: HD4 Framework
- RFC-9021: Graph Convergence Theory
- RFC-9023: MITRE Integration Map
