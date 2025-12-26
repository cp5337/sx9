//! Server Capabilities Declaration

use tower_lsp::lsp_types::*;

/// Return server capabilities
pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        // Text document sync
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::FULL),
                will_save: None,
                will_save_wait_until: None,
                save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                    include_text: Some(true),
                })),
            },
        )),

        // Completion
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![
                ".".to_string(),
                ":".to_string(),
                "@".to_string(),
                "/".to_string(),
            ]),
            all_commit_characters: None,
            work_done_progress_options: WorkDoneProgressOptions::default(),
            completion_item: None,
        }),

        // Hover
        hover_provider: Some(HoverProviderCapability::Simple(true)),

        // Go to definition
        definition_provider: Some(OneOf::Left(true)),

        // Document symbols
        document_symbol_provider: Some(OneOf::Left(true)),

        // Workspace symbols
        workspace_symbol_provider: Some(OneOf::Left(true)),

        // Code actions
        code_action_provider: Some(CodeActionProviderCapability::Simple(true)),

        // Code lens
        code_lens_provider: None,

        // Diagnostics
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            identifier: Some("sx9".to_string()),
            inter_file_dependencies: true,
            workspace_diagnostics: true,
            work_done_progress_options: WorkDoneProgressOptions::default(),
        })),

        // Others (disabled)
        signature_help_provider: None,
        references_provider: None,
        document_highlight_provider: None,
        document_formatting_provider: None,
        document_range_formatting_provider: None,
        document_on_type_formatting_provider: None,
        rename_provider: None,
        document_link_provider: None,
        color_provider: None,
        folding_range_provider: None,
        execute_command_provider: None,
        workspace: None,
        call_hierarchy_provider: None,
        semantic_tokens_provider: None,
        moniker_provider: None,
        linked_editing_range_provider: None,
        inline_value_provider: None,
        inlay_hint_provider: None,
        experimental: None,
        position_encoding: None,
        selection_range_provider: None,
        implementation_provider: None,
        type_definition_provider: None,
        declaration_provider: None,
    }
}
