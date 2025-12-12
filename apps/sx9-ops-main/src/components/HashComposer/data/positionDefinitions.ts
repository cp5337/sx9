import { PositionDefinition } from '../types';

export const positionDefinitions: PositionDefinition[] = [
  {
    id: 'hunt-operational',
    name: 'Hunt Operational',
    operational: {
      'H': 'Hunt - Active threat hunting',
      'U': 'Uncover - Discovery operations',
      'N': 'Navigate - Intelligence gathering',
      'T': 'Track - Target monitoring'
    },
    code: {
      'H': 'Hunt - Active threat hunting',
      'U': 'Uncover - Discovery operations', 
      'N': 'Navigate - Intelligence gathering',
      'T': 'Track - Target monitoring'
    },
    counter: {
      'H': 'Hunt - Active threat hunting',
      'U': 'Uncover - Discovery operations',
      'N': 'Navigate - Intelligence gathering', 
      'T': 'Track - Target monitoring'
    }
  },
  {
    id: 'detect-operational',
    name: 'Detect Operational',
    operational: {
      'D': 'Detect - Threat detection',
      'E': 'Examine - Analysis operations',
      'C': 'Capture - Data collection',
      'T': 'Trace - Evidence tracking'
    },
    code: {
      'D': 'Detect - Threat detection',
      'E': 'Examine - Analysis operations',
      'C': 'Capture - Data collection',
      'T': 'Trace - Evidence tracking'
    },
    counter: {
      'D': 'Detect - Threat detection',
      'E': 'Examine - Analysis operations',
      'C': 'Capture - Data collection',
      'T': 'Trace - Evidence tracking'
    }
  },
  {
    id: 'disrupt-operational',
    name: 'Disrupt Operational',
    operational: {
      'D': 'Disrupt - Operational disruption',
      'I': 'Interrupt - Service interruption',
      'S': 'Stop - Process termination',
      'R': 'Recover - System restoration'
    },
    code: {
      'D': 'Disrupt - Operational disruption',
      'I': 'Interrupt - Service interruption',
      'S': 'Stop - Process termination',
      'R': 'Recover - System restoration'
    },
    counter: {
      'D': 'Disrupt - Operational disruption',
      'I': 'Interrupt - Service interruption',
      'S': 'Stop - Process termination',
      'R': 'Recover - System restoration'
    }
  },
  {
    id: 'disable-operational',
    name: 'Disable Operational',
    operational: {
      'D': 'Disable - System disablement',
      'I': 'Isolate - Network isolation',
      'S': 'Shutdown - Service shutdown',
      'B': 'Block - Access blocking'
    },
    code: {
      'D': 'Disable - System disablement',
      'I': 'Isolate - Network isolation',
      'S': 'Shutdown - Service shutdown',
      'B': 'Block - Access blocking'
    },
    counter: {
      'D': 'Disable - System disablement',
      'I': 'Isolate - Network isolation',
      'S': 'Shutdown - Service shutdown',
      'B': 'Block - Access blocking'
    }
  },
  {
    id: 'dominate-operational',
    name: 'Dominate Operational',
    operational: {
      'D': 'Dominate - System domination',
      'O': 'Own - System ownership',
      'M': 'Maintain - Persistent access',
      'A': 'Advance - Privilege escalation'
    },
    code: {
      'D': 'Dominate - System domination',
      'O': 'Own - System ownership',
      'M': 'Maintain - Persistent access',
      'A': 'Advance - Privilege escalation'
    },
    counter: {
      'D': 'Dominate - System domination',
      'O': 'Own - System ownership',
      'M': 'Maintain - Persistent access',
      'A': 'Advance - Privilege escalation'
    }
  },
  {
    id: 'hunt-code',
    name: 'Hunt Code',
    operational: {
      'H': 'Hunt - Code analysis',
      'U': 'Understand - Code comprehension',
      'N': 'Navigate - Code exploration',
      'T': 'Trace - Code execution'
    },
    code: {
      'H': 'Hunt - Code analysis',
      'U': 'Understand - Code comprehension',
      'N': 'Navigate - Code exploration',
      'T': 'Trace - Code execution'
    },
    counter: {
      'H': 'Hunt - Code analysis',
      'U': 'Understand - Code comprehension',
      'N': 'Navigate - Code exploration',
      'T': 'Trace - Code execution'
    }
  },
  {
    id: 'detect-code',
    name: 'Detect Code',
    operational: {
      'D': 'Detect - Vulnerability detection',
      'E': 'Examine - Code examination',
      'C': 'Capture - Code capture',
      'T': 'Trace - Execution tracing'
    },
    code: {
      'D': 'Detect - Vulnerability detection',
      'E': 'Examine - Code examination',
      'C': 'Capture - Code capture',
      'T': 'Trace - Execution tracing'
    },
    counter: {
      'D': 'Detect - Vulnerability detection',
      'E': 'Examine - Code examination',
      'C': 'Capture - Code capture',
      'T': 'Trace - Execution tracing'
    }
  },
  {
    id: 'disrupt-code',
    name: 'Disrupt Code',
    operational: {
      'D': 'Disrupt - Code disruption',
      'I': 'Inject - Code injection',
      'S': 'Stop - Execution stopping',
      'R': 'Replace - Code replacement'
    },
    code: {
      'D': 'Disrupt - Code disruption',
      'I': 'Inject - Code injection',
      'S': 'Stop - Execution stopping',
      'R': 'Replace - Code replacement'
    },
    counter: {
      'D': 'Disrupt - Code disruption',
      'I': 'Inject - Code injection',
      'S': 'Stop - Execution stopping',
      'R': 'Replace - Code replacement'
    }
  },
  {
    id: 'disable-code',
    name: 'Disable Code',
    operational: {
      'D': 'Disable - Code disablement',
      'I': 'Isolate - Code isolation',
      'S': 'Shutdown - Code shutdown',
      'B': 'Block - Code blocking'
    },
    code: {
      'D': 'Disable - Code disablement',
      'I': 'Isolate - Code isolation',
      'S': 'Shutdown - Code shutdown',
      'B': 'Block - Code blocking'
    },
    counter: {
      'D': 'Disable - Code disablement',
      'I': 'Isolate - Code isolation',
      'S': 'Shutdown - Code shutdown',
      'B': 'Block - Code blocking'
    }
  },
  {
    id: 'dominate-code',
    name: 'Dominate Code',
    operational: {
      'D': 'Dominate - Code domination',
      'O': 'Own - Code ownership',
      'M': 'Maintain - Code maintenance',
      'A': 'Advance - Code advancement'
    },
    code: {
      'D': 'Dominate - Code domination',
      'O': 'Own - Code ownership',
      'M': 'Maintain - Code maintenance',
      'A': 'Advance - Code advancement'
    },
    counter: {
      'D': 'Dominate - Code domination',
      'O': 'Own - Code ownership',
      'M': 'Maintain - Code maintenance',
      'A': 'Advance - Code advancement'
    }
  },
  {
    id: 'hunt-counter',
    name: 'Hunt Counter',
    operational: {
      'H': 'Hunt - Counter-hunting',
      'U': 'Uncover - Counter-discovery',
      'N': 'Navigate - Counter-navigation',
      'T': 'Track - Counter-tracking'
    },
    code: {
      'H': 'Hunt - Counter-hunting',
      'U': 'Uncover - Counter-discovery',
      'N': 'Navigate - Counter-navigation',
      'T': 'Track - Counter-tracking'
    },
    counter: {
      'H': 'Hunt - Counter-hunting',
      'U': 'Uncover - Counter-discovery',
      'N': 'Navigate - Counter-navigation',
      'T': 'Track - Counter-tracking'
    }
  },
  {
    id: 'detect-counter',
    name: 'Detect Counter',
    operational: {
      'D': 'Detect - Counter-detection',
      'E': 'Examine - Counter-examination',
      'C': 'Capture - Counter-capture',
      'T': 'Trace - Counter-tracing'
    },
    code: {
      'D': 'Detect - Counter-detection',
      'E': 'Examine - Counter-examination',
      'C': 'Capture - Counter-capture',
      'T': 'Trace - Counter-tracing'
    },
    counter: {
      'D': 'Detect - Counter-detection',
      'E': 'Examine - Counter-examination',
      'C': 'Capture - Counter-capture',
      'T': 'Trace - Counter-tracing'
    }
  },
  {
    id: 'disrupt-counter',
    name: 'Disrupt Counter',
    operational: {
      'D': 'Disrupt - Counter-disruption',
      'I': 'Interrupt - Counter-interruption',
      'S': 'Stop - Counter-stopping',
      'R': 'Recover - Counter-recovery'
    },
    code: {
      'D': 'Disrupt - Counter-disruption',
      'I': 'Interrupt - Counter-interruption',
      'S': 'Stop - Counter-stopping',
      'R': 'Recover - Counter-recovery'
    },
    counter: {
      'D': 'Disrupt - Counter-disruption',
      'I': 'Interrupt - Counter-interruption',
      'S': 'Stop - Counter-stopping',
      'R': 'Recover - Counter-recovery'
    }
  },
  {
    id: 'disable-counter',
    name: 'Disable Counter',
    operational: {
      'D': 'Disable - Counter-disablement',
      'I': 'Isolate - Counter-isolation',
      'S': 'Shutdown - Counter-shutdown',
      'B': 'Block - Counter-blocking'
    },
    code: {
      'D': 'Disable - Counter-disablement',
      'I': 'Isolate - Counter-isolation',
      'S': 'Shutdown - Counter-shutdown',
      'B': 'Block - Counter-blocking'
    },
    counter: {
      'D': 'Disable - Counter-disablement',
      'I': 'Isolate - Counter-isolation',
      'S': 'Shutdown - Counter-shutdown',
      'B': 'Block - Counter-blocking'
    }
  },
  {
    id: 'dominate-counter',
    name: 'Dominate Counter',
    operational: {
      'D': 'Dominate - Counter-domination',
      'O': 'Own - Counter-ownership',
      'M': 'Maintain - Counter-maintenance',
      'A': 'Advance - Counter-advancement'
    },
    code: {
      'D': 'Dominate - Counter-domination',
      'O': 'Own - Counter-ownership',
      'M': 'Maintain - Counter-maintenance',
      'A': 'Advance - Counter-advancement'
    },
    counter: {
      'D': 'Dominate - Counter-domination',
      'O': 'Own - Counter-ownership',
      'M': 'Maintain - Counter-maintenance',
      'A': 'Advance - Counter-advancement'
    }
  }
];
