use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use crate::dhatu::roots::DhatuRegistry;

pub struct KasturiLsp {
    client: Client,
    dhatu_registry: DhatuRegistry,
}

#[tower_lsp::async_trait]
impl LanguageServer for KasturiLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["√".to_string(), "+".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "ॐ Kasturi LSP Server Initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        // MVP: Provide generic hover or check the dhatu registry for specific roots.
        // In a full implementation, we'd extract the word under the cursor.
        
        let _position = params.text_document_position_params.position;
        // Mock lookup for demonstration
        let root = "sṛj"; // Hypothetical extracted word
        
        if let Some(dhatu) = self.dhatu_registry.lookup(root) {
            let markdown = format!(
                "**{}** (_{}_)\n\n**Meaning:** {}\n**Gana:** {:?}",
                root, dhatu.devanagari, dhatu.meaning, dhatu.gana
            );
            Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(markdown)),
                range: None,
            }))
        } else {
            Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String("Vedic Dhatu not recognized.".into())),
                range: None,
            }))
        }
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let mut items = Vec::new();

        let basic_roots = vec![
            ("√वच्", "vac", "Speak/Print"),
            ("√सृज्", "sṛj", "Create/Assign"),
            ("√दृश्", "dṛś", "See/Read Storage"),
            ("√स्मृ", "smṛ", "Remember/Write Storage"),
            ("√स्था", "sthā", "Establish Server"),
            ("√सेतु", "setu", "Bridge/Interop"),
            ("√काल", "kāla", "Time/Timestamp"),
            ("√कुञ्च्", "kuñc", "Generate Keypair"),
            ("√चिह्न", "cihna", "Sign Data"),
            ("√परीक्ष्", "parīkṣ", "Verify Signature"),
        ];

        for (devanagari, latin, detail) in basic_roots {
            items.push(CompletionItem {
                label: devanagari.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(detail.to_string()),
                documentation: Some(Documentation::String(format!("Sanskrit Root: {}", latin))),
                insert_text: Some(format!("{}+ति·", devanagari)),
                ..Default::default()
            });
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // Run diagnostics (Parser checks)
        let uri = params.text_document.uri;
        let text = if let Some(change) = params.content_changes.first() {
            change.text.clone()
        } else {
            return;
        };

        // If it was a real parser we'd send diagnostics back
        // self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

pub async fn start_lsp() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| KasturiLsp {
        client,
        dhatu_registry: DhatuRegistry::new(),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
