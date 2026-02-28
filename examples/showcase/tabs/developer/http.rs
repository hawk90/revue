//! HTTP widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{HttpClient, HttpMethod, JsonViewer};

pub fn render() -> impl View {
    let (primary, success, _warning, error, _info, muted, text, _) = theme_colors();

    vstack()
        .gap(1)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" HTTP Client ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("REST API client:").fg(primary))
                            .child(Text::new(""))
                            .child(
                                HttpClient::new()
                                    .url("https://api.example.com/users")
                                    .method(HttpMethod::GET),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• URL input").fg(muted))
                            .child(Text::new("• Method selector").fg(muted))
                            .child(Text::new("• Send button").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" GET Request ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Request:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("GET /api/users HTTP/1.1").fg(text))
                            .child(Text::new("Host: api.example.com").fg(muted))
                            .child(Text::new("Accept: application/json").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Response (200 OK):").fg(primary))
                            .child(Text::new(""))
                            .child(JsonViewer::new().json(
                                r#"{
    "users": [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]
}"#,
                            ))
                            .child(Text::new(""))
                            .child(Text::new("• Fetch data").fg(muted))
                            .child(Text::new("• Query params").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" POST Request ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Request:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("POST /api/users HTTP/1.1").fg(text))
                            .child(Text::new("Host: api.example.com").fg(muted))
                            .child(Text::new("Content-Type: application/json").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Body:").fg(primary))
                            .child(
                                Text::new(
                                    r#"{
    "name": "Charlie",
    "email": "charlie@example.com"
}"#,
                                )
                                .fg(text),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Create resources").fg(muted))
                            .child(Text::new("• JSON body").fg(muted))
                            .child(Text::new("• Form data").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Response Viewer ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Status: 200 OK").fg(success))
                            .child(Text::new(""))
                            .child(Text::new("Headers:").fg(primary))
                            .child(Text::new("  Content-Type: application/json").fg(muted))
                            .child(Text::new("  X-Rate-Limit: 100").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Body:").fg(primary))
                            .child(Text::new(r#"  {"success": true, "data": [...]}"#).fg(text))
                            .child(Text::new(""))
                            .child(Text::new("• Status code").fg(muted))
                            .child(Text::new("• Response headers").fg(muted))
                            .child(Text::new("• Body preview").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Error Response ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Status: 404 Not Found").fg(error))
                            .child(Text::new(""))
                            .child(Text::new("Headers:").fg(primary))
                            .child(Text::new("  Content-Type: application/json").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Body:").fg(primary))
                            .child(
                                Text::new(
                                    r#"{
    "error": "User not found",
    "code": "USER_404"
}"#,
                                )
                                .fg(text),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Error handling").fg(muted))
                            .child(Text::new("• Status colors").fg(muted))
                            .child(Text::new("• Error messages").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Request History ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("[200] GET /api/users (2ms)").fg(success))
                            .child(Text::new("[201] POST /api/users (5ms)").fg(success))
                            .child(Text::new("[200] GET /api/users/1 (1ms)").fg(success))
                            .child(Text::new("[204] DELETE /api/users/1 (3ms)").fg(text))
                            .child(Text::new("[200] GET /api/users (2ms)").fg(success))
                            .child(Text::new(""))
                            .child(Text::new("• Request log").fg(muted))
                            .child(Text::new("• Response times").fg(muted))
                            .child(Text::new("• Replay option").fg(muted)),
                    ),
                ),
        )
}
