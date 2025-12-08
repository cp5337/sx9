# Kali → CTAS Integration Plan

**Date:** December 7, 2025  
**Status:** 📋 **READY FOR IMPLEMENTATION**  
**Goal:** Integrate Kali Linux tools and capabilities into CTAS-7

---

## 🎯 **OBJECTIVE**

Enable CTAS-7 to manage Kali tools, build custom ISOs, run containers, and execute offensive security operations.

---

## 📊 **CURRENT KALI INFRASTRUCTURE**

### **Existing Systems**
- Kali Tools Inventory (600+ tools)
- Docker API integration
- CALDERA integration
- Exploit-DB integration
- Plasma integration (Wazuh + AXON + Legion)

### **Components to Build**
- ISO builder system
- Container management
- Tool execution framework
- Plasma integration UI

---

## 🔧 **INTEGRATION STEPS**

### **Step 1: Create Kali Service Layer** (2-3 hours)

**File:** `sx9-ops-main-platform/src/services/kaliService.ts`

```typescript
import { supabase } from '@/utils/supabaseClient';
import Docker from 'dockerode';

export class KaliService {
  private docker: Docker;
  
  constructor() {
    this.docker = new Docker();
  }
  
  // ISO Management
  async buildISO(config: ISOConfig): Promise<ISOBuildResult> {
    // Call ISO builder backend
  }
  
  // Docker Management
  async pullKaliImage(tag: string = 'latest'): Promise<void> {
    await this.docker.pull(`kalilinux/kali-rolling:${tag}`);
  }
  
  async createContainer(config: ContainerConfig): Promise<string> {
    const container = await this.docker.createContainer({
      Image: 'kalilinux/kali-rolling',
      ...config
    });
    return container.id;
  }
  
  async runContainer(containerId: string, command: string): Promise<ExecutionResult> {
    const container = this.docker.getContainer(containerId);
    const exec = await container.exec({
      Cmd: command.split(' '),
      AttachStdout: true,
      AttachStderr: true
    });
    // Execute and return results
  }
  
  // Tool Execution
  async executeTool(tool: string, args: string[]): Promise<ToolResult> {
    // Execute Kali tool in container
  }
  
  // Plasma Integration
  async enablePlasma(containerId: string): Promise<void> {
    // Enable Plasma (Wazuh + AXON + Legion) in container
  }
}
```

### **Step 2: Add Kali Components to Gallery** (2-3 hours)

**File:** `sx9-ops-main-platform/src/pages/Gallery.tsx`

Add 11 Kali components:
1. Kali Synaptix ISO Builder (Enterprise)
2. CTAS Operator ISO Builder (Enterprise)
3. Kali Tools Inventory (Basic)
4. Kali Purple Team Suite (Pro)
5. ISO Customization Engine (Pro)
6. Plasma Integration (Agnostic) (Enterprise)
7. CALDERA Integration (Pro)
8. Exploit-DB Integration (Basic)
9. Multi-Agent Multi-Terminal Prompt (Agnostic) (Enterprise)
10. Layer 2 Plasma ISO (Enterprise)
11. Kali Docker Container (Basic)

### **Step 3: Create Kali Dashboard** (3-4 hours)

**File:** `sx9-ops-main-platform/src/pages/Kali.tsx`

- Tool browser
- Container management
- ISO builder interface
- Execution monitoring

### **Step 4: Integrate with HD4PhaseContent** (1 hour)

Add Kali tab to horizon tabs.

---

## 📋 **IMPLEMENTATION CHECKLIST**

- [ ] Create `kaliService.ts`
- [ ] Integrate Docker API
- [ ] Add 11 Kali components to Gallery
- [ ] Create Kali dashboard page
- [ ] Add Kali tab to HD4PhaseContent
- [ ] Build ISO builder backend
- [ ] Integrate Plasma
- [ ] Test container management
- [ ] Test tool execution

---

## ⏱️ **ESTIMATED TIME: 12-15 hours**

---

## 📄 **See Also:**
- `kali_iso_integration_plan.plan.md` (detailed plan)


