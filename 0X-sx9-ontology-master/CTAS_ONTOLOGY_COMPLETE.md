# CTAS Complete Tactical Ontology

## Domain Primitives (5-Tuple Structure)

### Adversarial Domain
**Actor**: Terrorist Cell Leader, Bomb Maker, Courier, Financier, Recruiter, Cyber Operative, Sleeper Agent, Handler, Scout, Weapons Dealer

**Object**: IED Components, Vehicle, Communications Device, Weapon Cache, Safe House, Financial Assets, Identity Documents, Surveillance Equipment, Encrypted Drive

**Event**: Bomb Deployment, Reconnaissance Mission, Fund Transfer, Cell Meeting, Training Session, Cyber Attack, Dead Drop Exchange, Border Crossing, Target Surveillance

**Concept**: Attack Plan, Operational Security, Martyrdom Ideology, Cell Structure, Escape Route, Target Selection, Timing Strategy, Resource Allocation, Risk Assessment

**Attribute**: Explosives Expertise, Language Skills, Technical Capability, Geographic Knowledge, Security Clearance, Financial Resources, Operational Experience, Physical Fitness

### Manufacturing Domain
**Actor**: Production Manager, Quality Inspector, Machine Operator, Maintenance Tech, Safety Officer, Supply Chain Coordinator, Process Engineer, Automation Specialist, Logistics Manager

**Object**: Assembly Line, Raw Materials, Quality Sensors, Robotic Arm, Conveyor System, Inventory Database, Production Schedule, Safety Equipment, Testing Apparatus

**Event**: Production Start, Quality Check, Material Delivery, Equipment Maintenance, Shift Change, Batch Processing, Defect Detection, Order Fulfillment, Safety Inspection

**Concept**: Lean Manufacturing, Six Sigma, Just-in-Time, Quality Control, Process Optimization, Waste Reduction, Efficiency Metrics, Safety Protocols, Cost Management

**Attribute**: Production Capacity, Error Rate, Throughput Speed, Energy Efficiency, Safety Rating, Maintenance Schedule, Skill Level, Certification Status, Experience Years

### Medical Domain
**Actor**: Attending Physician, Nurse Practitioner, Pharmacist, Lab Technician, Emergency Responder, Patient, Radiologist, Surgeon, Medical Administrator, Infection Control Specialist

**Object**: Medical Device, Patient Record, Medication, Diagnostic Equipment, Treatment Protocol, Laboratory Sample, Surgical Instrument, Patient Monitor, Emergency Kit

**Event**: Patient Admission, Diagnosis, Treatment Administration, Surgery, Discharge, Lab Test, Medication Dosing, Vital Signs Check, Emergency Response

**Concept**: Patient Safety, Treatment Efficacy, Infection Control, Pain Management, Recovery Protocol, Diagnostic Accuracy, Medication Adherence, Risk Assessment, Care Coordination

**Attribute**: Medical Expertise, Patient Condition, Treatment Response, Risk Factor, Compliance Level, Vital Signs, Lab Values, Symptom Severity, Recovery Rate

## HD4 Counter-Primitives

### Hunt
- **Description**: Active search and discovery operations
- **Tactics**: Reconnaissance, Intelligence Gathering, Pattern Analysis, Behavioral Tracking, Network Mapping
- **Color**: Red (bg-red-500/20 text-red-400 border-red-500/30)

### Detect
- **Description**: Monitoring and anomaly recognition
- **Tactics**: Sensor Monitoring, Signature Analysis, Threshold Detection, Behavioral Analysis, Alert Generation
- **Color**: Yellow (bg-yellow-500/20 text-yellow-400 border-yellow-500/30)

### Disrupt
- **Description**: Active interference and path collapse
- **Tactics**: Signal Jamming, Resource Denial, Pathway Blocking, Communication Interference, Process Interruption
- **Color**: Orange (bg-orange-500/20 text-orange-400 border-orange-500/30)

### Disable
- **Description**: Neutralization and capability removal
- **Tactics**: System Shutdown, Resource Exhaustion, Access Revocation, Function Termination, Capability Nullification
- **Color**: Purple (bg-purple-500/20 text-purple-400 border-purple-500/30)

### Dominate
- **Description**: Control establishment and maintenance
- **Tactics**: System Control, Authority Assertion, Structure Locking, Behavioral Override, Operational Superiority
- **Color**: Green (bg-green-500/20 text-green-400 border-green-500/30)

## Counter-Mapping Examples
- **Bomb Deployment**: Hunt → Detect → Disable
- **Cyber Attack**: Detect → Disrupt → Dominate
- **Reconnaissance Mission**: Hunt → Detect → Disrupt
- **Cell Meeting**: Hunt → Detect → Disrupt
- **Fund Transfer**: Detect → Disrupt → Disable
- **Training Session**: Hunt → Disrupt → Disable
- **Target Surveillance**: Hunt → Detect → Disrupt

## Position Definitions (Base96 Encoding)

### Position 1: Primitive Type
- **Operational**: A=Actor, O=Object, E=Event, C=Concept, T=Attribute
- **Code**: F=Function, M=Module, H=Header, G=Footer, N=Comment
- **Counter**: U=Hunt, D=Detect, R=Disrupt, B=Disable, G=Dominate

### Position 2: TTL Decay Class
- 0=Immediate (seconds), 1=Short (minutes), 2=Medium (hours)
- 3=Long (days), 4=Extended (weeks), 5=Persistent (months)

### Position 3: Entropy Class
- 0=Static (0.0-0.1), 1=Stable (0.1-0.3), 2=Moderate (0.3-0.5)
- 3=Dynamic (0.5-0.7), 4=Volatile (0.7-0.9), 5=Chaotic (0.9-1.0)

### Position 8: Graph Role Code
- S=Source (Origin point), K=Sink (Terminal point), R=Relay (Pass-through node)
- H=Hub (Connection center), B=Bridge (Cross-domain link)

This ontology provides the foundation for 190+ tactical task nodes with complete relationship mapping and cross-domain applicability.