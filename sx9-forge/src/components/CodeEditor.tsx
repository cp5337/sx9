
import type React from "react"
import { useRef, useState } from "react"
import Editor, { type Monaco } from "@monaco-editor/react"
import type { editor } from "monaco-editor"
import { View, Text, StyleSheet, TouchableOpacity } from "react-native"
import { colors, spacing, typography } from "../tokens"

export interface CodeEditorProps {
  value: string
  onChange: (value: string) => void
  language?: string
  readOnly?: boolean
  height?: number | string
  showToolbar?: boolean
  onSave?: () => void
  onFormat?: () => void
}

export const CodeEditor: React.FC<CodeEditorProps> = ({
  value,
  onChange,
  language = "markdown",
  readOnly = false,
  height = "100%",
  showToolbar = true,
  onSave,
  onFormat,
}) => {
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null)
  const [wordCount, setWordCount] = useState(0)
  const [lineCount, setLineCount] = useState(1)

  const handleEditorDidMount = (editor: editor.IStandaloneCodeEditor, monaco: Monaco) => {
    editorRef.current = editor

    // Configure Monaco theme for dark mode
    monaco.editor.defineTheme("cx9-dark", {
      base: "vs-dark",
      inherit: true,
      rules: [],
      colors: {
        "editor.background": colors.background.secondary,
        "editor.foreground": colors.text.primary,
        "editor.lineHighlightBackground": colors.surface.secondary,
        "editorLineNumber.foreground": colors.text.tertiary,
        "editor.selectionBackground": colors.primary + "40",
        "editor.inactiveSelectionBackground": colors.primary + "20",
      },
    })
    monaco.editor.setTheme("cx9-dark")

    // Update stats on content change
    editor.onDidChangeModelContent(() => {
      const model = editor.getModel()
      if (model) {
        const content = model.getValue()
        setWordCount(content.split(/\s+/).filter(Boolean).length)
        setLineCount(model.getLineCount())
      }
    })
  }

  const handleEditorChange = (value: string | undefined) => {
    if (value !== undefined) {
      onChange(value)
    }
  }

  const handleFormat = () => {
    if (editorRef.current) {
      editorRef.current.getAction("editor.action.formatDocument")?.run()
    }
    if (onFormat) {
      onFormat()
    }
  }

  const handleSave = () => {
    if (onSave) {
      onSave()
    }
  }

  return (
    <View style={styles.container}>
      {showToolbar && (
        <View style={styles.toolbar}>
          <View style={styles.stats}>
            <Text style={styles.statText}>{lineCount} lines</Text>
            <Text style={styles.statDivider}>•</Text>
            <Text style={styles.statText}>{wordCount} words</Text>
            <Text style={styles.statDivider}>•</Text>
            <Text style={styles.statText}>{language}</Text>
          </View>
          <View style={styles.actions}>
            {onFormat && (
              <TouchableOpacity style={styles.toolbarButton} onPress={handleFormat}>
                <Text style={styles.toolbarButtonText}>Format</Text>
              </TouchableOpacity>
            )}
            {onSave && (
              <TouchableOpacity style={styles.toolbarButton} onPress={handleSave}>
                <Text style={styles.toolbarButtonText}>Save</Text>
              </TouchableOpacity>
            )}
          </View>
        </View>
      )}
      <View style={styles.editorContainer}>
        <Editor
          height={height}
          language={language}
          value={value}
          onChange={handleEditorChange}
          onMount={handleEditorDidMount}
          options={{
            readOnly,
            minimap: { enabled: false },
            fontSize: typography.fontSize.base,
            fontFamily: "monospace",
            lineNumbers: "on",
            rulers: [80, 120],
            wordWrap: "on",
            scrollBeyondLastLine: false,
            automaticLayout: true,
            tabSize: 2,
            insertSpaces: true,
            renderWhitespace: "selection",
            bracketPairColorization: {
              enabled: true,
            },
          }}
        />
      </View>
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background.secondary,
  },
  toolbar: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.md,
    backgroundColor: colors.surface.primary,
    borderBottomWidth: 1,
    borderBottomColor: colors.border.primary,
  },
  stats: {
    flexDirection: "row",
    alignItems: "center",
    gap: spacing.sm,
  },
  statText: {
    fontSize: typography.fontSize.xs,
    color: colors.text.secondary,
  },
  statDivider: {
    fontSize: typography.fontSize.xs,
    color: colors.text.tertiary,
  },
  actions: {
    flexDirection: "row",
    gap: spacing.sm,
  },
  toolbarButton: {
    paddingHorizontal: spacing.md,
    paddingVertical: spacing.sm,
    backgroundColor: colors.surface.secondary,
    borderRadius: 4,
  },
  toolbarButtonText: {
    fontSize: typography.fontSize.sm,
    color: colors.text.primary,
    fontWeight: typography.fontWeight.medium.toString(),
  },
  editorContainer: {
    flex: 1,
  },
})
