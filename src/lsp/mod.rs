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
        // Zero Mocks: True LSP Hover Extraction
        let position = params.text_document_position_params.position;
        let uri = params.text_document_position_params.text_document.uri;
        
        let mut root_word = String::new();
        
        if let Ok(file_path) = uri.to_file_path() {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                let lines: Vec<&str> = content.lines().collect();
                if let Some(line) = lines.get(position.line as usize) {
                    let char_idx = position.character as usize;
                    
                    // Extract word surrounding the cursor
                    let mut start = char_idx;
                    let mut end = char_idx;
                    let chars: Vec<char> = line.chars().collect();
                    
                    if char_idx < chars.len() {
                        while start > 0 && chars[start - 1].is_alphanumeric() {
                            start -= 1;
                        }
                        while end < chars.len() && chars[end].is_alphanumeric() {
                            end += 1;
                        }
                        if start < end {
                            root_word = chars[start..end].iter().collect();
                        }
                    }
                }
            }
        }
        
        if root_word.is_empty() {
            return Ok(None);
        }
        
        if let Some(dhatu) = self.dhatu_registry.lookup(&root_word) {
            let markdown = format!(
                "**{}** (_{}_)\n\n**Meaning:** {}\n**Gana:** {:?}",
                root_word, dhatu.devanagari, dhatu.meaning, dhatu.gana
            );
            Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(markdown)),
                range: None,
            }))
        } else {
            Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(format!("Vedic Dhatu not recognized: {}", root_word))),
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
