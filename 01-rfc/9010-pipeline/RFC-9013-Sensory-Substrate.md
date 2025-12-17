RFC-9013: SX9 Sensory Substrate: Haptics, Voice, Tone & L2 Ops

The “bidirectional sensory substrate” you described.

Sections


Purpose


Define SX9 sensory substrate as bidirectional context fabric


Applicable to: iPad haptics, robot ops, voice control, ops terminals




Sensory Layer Overview


Mapping between:


Haptics


Visual cues


Audio/tone


DSL & tails


L2 operational layer






Haptic Semantics


Angle-based semantics: attraction/repulsion/intensity


Examples:


“Boots before pants” = strong repulsion


“Left vs right boot order” = mild preference




Haptic profiles for: warning, error, success, subtle nudge




Voice Prompt Continuum


Axes like:


ask → tell → make → reason → inspire → educate → remediate




How each maps to:


tail flags


suppression tier


response tone/tenor


required explanation level






SX9 Haptic + Voice to DSL Mapping


How sensory signals annotate DSL operations


Tail fields for sensory context (haptic profile, voice intent)


Versioning: sensory scheme version stored in hash/meta




Robot Ops & ROS Integration Hooks


How SX9 sensory layer feeds ROS / robot controllers


Semantics for:


“soft stop” vs “hard stop”


“nudge” vs “override”




Use in remote operator + AI shared control




L2 Execution Integration


How sensory substrate affects:


tool selection


escalation decisions


gating operations based on risk/intent






Safety & Human Factors


Preventing overload or misinterpretation


Guidelines for combining haptics, tone, and risk levels


Minimum explanation requirements for high-risk actions




Conformance & Extensibility


How new sensory channels plug in (AR, BCI, etc.)


Version negotiation and backward compatibility





🔚 How to Use This Right Now


Drop these section maps into rfcs/ as markdown stubs


RFC-9000-Core.md


RFC-9010-Enterprise-Extraction.md (replacing the old version)


RFC-9011-Content-Ingestion-YAML-DSL-Playbooks.md


RFC-9012-Embeddings-GNN.md


RFC-9013-Sensory-Substrate.md




Then point your SX9 RFC Engine at them to:


Slot them into the LaTeX RFC template you pasted


Fill in boilerplate


Start drafting details section-by-section




When you’re ready, we can take any one of these and fully draft it in RFC voice, then let the engine convert to LaTeX.