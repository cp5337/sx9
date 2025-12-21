# RFC-9308
## CONVERGE Integration with GLAF and Smart Crates

Status: Draft

Smart Crate:
class = analytic.sensor.convergence
storage = none
deterministic = true

GLAF:
ConvergeSignal -> graph node

Convergence:
rank_delta feeds RFC-9021 logic

Prohibitions:
No direct actuation, no bypass.
