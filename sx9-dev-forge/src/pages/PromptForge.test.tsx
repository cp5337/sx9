import {
  render,
  screen,
  fireEvent,
  waitFor,
  within,
} from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import PromptForge from "./PromptForge";

// Mock Tauri invoke to prevent actual IPC calls
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("PromptForge Agentic Logic", () => {
  it("renders the main title", () => {
    render(<PromptForge />);
    expect(screen.getByText("PROMPT FORGE")).toBeInTheDocument();
  });

  it("swaps personas and updates role constraints", async () => {
    render(<PromptForge />);

    // Switch to AXIOM
    // Note: The Persona buttons are labelled by key in the PERSONAS map.
    // We need to check PromptForge.tsx to see what text is rendered.
    // It renders {k} (the key name).
    const personaButton = screen.getByText("AXIOM");
    fireEvent.click(personaButton);

    // Verify role update in YAML output (since role field might not be visible)
    const generateBtn = screen.getByText("GENERATE");
    fireEvent.click(generateBtn);

    await waitFor(() => {
      screen.getByTestId("prompt-type-grid").nextSibling; // Just verify output contains text
      // Or better, look for the text in the document
      expect(screen.getByText(/You are AXIOM/)).toBeInTheDocument();
    });
  });

  it("configures tools based on Harness selection", async () => {
    render(<PromptForge />);

    // Select SECURITY harness from the HARNESS section specifically
    const harnessSection = screen.getByTestId("harness-section");
    // Note: The text might be "Security Audit" or just "Security" depending on HARNESSES definition.
    // Let's use getByText loosely or verify what's in there.
    // Based on previous failure: "Security Audit" exists.
    const securityHarness = within(harnessSection).getByText("Security Audit");
    fireEvent.click(securityHarness);

    // Verify specific tool states
    // Switch to TOOLS tab first because they are conditionally rendered
    fireEvent.click(screen.getByText("TOOLS"));

    // "Memory" should be checked for Security Audit harness
    const memoryCheckbox = screen.getByLabelText(/Memory/i) as HTMLInputElement;
    expect(memoryCheckbox.checked).toBe(true);
  });

  it("generates valid YAML structure", async () => {
    render(<PromptForge />);

    // Click AXIOM
    fireEvent.click(screen.getByText("AXIOM"));

    // Enter a Title
    const titleInput = screen.getByPlaceholderText("Descriptive title");
    fireEvent.change(titleInput, { target: { value: "Test Mission" } });

    // Click Generate
    const generateBtn = screen.getByText("GENERATE");
    fireEvent.click(generateBtn);

    // Check output area
    await waitFor(() => {
      const output = screen.getByText((content) =>
        content.includes("# Type: Custom")
      );
      expect(output).toBeInTheDocument();
      // Check for persona in the text content
      expect(output.textContent).toContain("persona: AXIOM");
    });
  });
});
