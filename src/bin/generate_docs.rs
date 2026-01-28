use std::fs;
use std::path::Path;
use utoipa::OpenApi;

use rust_teraform_backend::presentation::openapi::ApiDoc;

fn main() {
    let openapi = ApiDoc::openapi();

    // Create docs directory
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir).expect("Failed to create docs directory");
    }

    // Generate OpenAPI JSON
    let json = serde_json::to_string_pretty(&openapi).expect("Failed to serialize to JSON");
    fs::write(docs_dir.join("openapi.json"), &json).expect("Failed to write openapi.json");
    println!("✓ Generated: docs/openapi.json");

    // Generate OpenAPI YAML
    let yaml = serde_yaml::to_string(&openapi).expect("Failed to serialize to YAML");
    fs::write(docs_dir.join("openapi.yaml"), &yaml).expect("Failed to write openapi.yaml");
    println!("✓ Generated: docs/openapi.yaml");

    // Generate Markdown documentation
    let markdown = generate_markdown(&openapi);
    fs::write(docs_dir.join("API.md"), &markdown).expect("Failed to write API.md");
    println!("✓ Generated: docs/API.md");

    println!("\n🎉 API documentation generated successfully!");
}

fn generate_markdown(openapi: &utoipa::openapi::OpenApi) -> String {
    let mut md = String::new();

    // Title and description
    md.push_str(&format!("# {}\n\n", openapi.info.title));
    md.push_str(&format!("**Version**: {}\n\n", openapi.info.version));

    if let Some(desc) = &openapi.info.description {
        md.push_str(&format!("{}\n\n", desc));
    }

    // Table of contents
    md.push_str("## 目次\n\n");
    md.push_str("- [認証](#認証)\n");
    md.push_str("- [エンドポイント](#エンドポイント)\n");
    md.push_str("  - [Auth API](#auth-api)\n");
    md.push_str("  - [Todos API](#todos-api)\n");
    md.push_str("- [スキーマ](#スキーマ)\n\n");

    // Authentication
    md.push_str("## 認証\n\n");
    md.push_str("このAPIはJWT Bearer認証を使用します。\n\n");
    md.push_str("認証が必要なエンドポイントでは、以下のヘッダーを含めてください:\n\n");
    md.push_str("```\nAuthorization: Bearer <access_token>\n```\n\n");

    // Endpoints
    md.push_str("## エンドポイント\n\n");

    // Group by tag
    let mut auth_endpoints = Vec::new();
    let mut todo_endpoints = Vec::new();

    for (path, item) in openapi.paths.paths.iter() {
        if let Some(ops) = get_operations(item) {
            for (method, op) in ops {
                let tags = op.tags.clone().unwrap_or_default();
                if tags.iter().any(|t| t == "auth") {
                    auth_endpoints.push((path.clone(), method, op.clone()));
                } else if tags.iter().any(|t| t == "todos") {
                    todo_endpoints.push((path.clone(), method, op.clone()));
                }
            }
        }
    }

    // Auth API
    md.push_str("### Auth API\n\n");
    for (path, method, op) in &auth_endpoints {
        md.push_str(&format_endpoint(path, method, op));
    }

    // Todos API
    md.push_str("### Todos API\n\n");
    for (path, method, op) in &todo_endpoints {
        md.push_str(&format_endpoint(path, method, op));
    }

    // Schemas
    md.push_str("## スキーマ\n\n");
    if let Some(components) = &openapi.components {
        for (name, schema) in &components.schemas {
            md.push_str(&format!("### {}\n\n", name));
            md.push_str(&format_schema(schema));
            md.push('\n');
        }
    }

    md
}

fn get_operations(
    item: &utoipa::openapi::path::PathItem,
) -> Option<Vec<(String, &utoipa::openapi::path::Operation)>> {
    let mut ops = Vec::new();

    if let Some(op) = &item.get {
        ops.push(("GET".to_string(), op));
    }
    if let Some(op) = &item.post {
        ops.push(("POST".to_string(), op));
    }
    if let Some(op) = &item.put {
        ops.push(("PUT".to_string(), op));
    }
    if let Some(op) = &item.delete {
        ops.push(("DELETE".to_string(), op));
    }
    if let Some(op) = &item.patch {
        ops.push(("PATCH".to_string(), op));
    }

    if ops.is_empty() {
        None
    } else {
        Some(ops)
    }
}

fn format_endpoint(path: &str, method: &str, op: &utoipa::openapi::path::Operation) -> String {
    let mut md = String::new();

    // Title with method badge
    let summary = op.summary.as_deref().unwrap_or("No description");
    md.push_str(&format!("#### `{}` {}\n\n", method, path));
    md.push_str(&format!("{}\n\n", summary));

    // Security
    if let Some(security) = &op.security {
        if !security.is_empty() {
            md.push_str("🔒 **認証必須**\n\n");
        }
    }

    // Parameters
    if let Some(params) = &op.parameters {
        if !params.is_empty() {
            md.push_str("**パラメータ**\n\n");
            md.push_str("| 名前 | 位置 | 型 | 必須 | 説明 |\n");
            md.push_str("|------|------|-----|------|------|\n");
            for param in params {
                let required = matches!(param.required, utoipa::openapi::Required::True);
                let required_mark = if required { "✓" } else { "" };
                let desc = param.description.as_deref().unwrap_or("");
                let param_in = match param.parameter_in {
                    utoipa::openapi::path::ParameterIn::Path => "path",
                    utoipa::openapi::path::ParameterIn::Query => "query",
                    utoipa::openapi::path::ParameterIn::Header => "header",
                    utoipa::openapi::path::ParameterIn::Cookie => "cookie",
                };
                md.push_str(&format!(
                    "| {} | {} | {} | {} | {} |\n",
                    param.name,
                    param_in,
                    get_param_type(&param.schema),
                    required_mark,
                    desc
                ));
            }
            md.push('\n');
        }
    }

    // Request body
    if let Some(body) = &op.request_body {
        md.push_str("**リクエストボディ**\n\n");
        if let Some(content) = body.content.get("application/json") {
            if let Some(schema) = &content.schema {
                md.push_str(&format!(
                    "```json\n{}\n```\n\n",
                    format_schema_example(schema)
                ));
            }
        }
    }

    // Responses
    md.push_str("**レスポンス**\n\n");
    md.push_str("| ステータス | 説明 |\n");
    md.push_str("|------------|------|\n");
    for (status, response) in &op.responses.responses {
        let desc = get_response_description(response);
        md.push_str(&format!("| {} | {} |\n", status, desc));
    }
    md.push_str("\n---\n\n");

    md
}

fn get_response_description(
    response: &utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
) -> String {
    match response {
        utoipa::openapi::RefOr::T(r) => r.description.clone(),
        utoipa::openapi::RefOr::Ref(r) => r.ref_location.clone(),
    }
}

fn get_param_type(schema: &Option<utoipa::openapi::RefOr<utoipa::openapi::Schema>>) -> String {
    match schema {
        Some(utoipa::openapi::RefOr::T(s)) => get_schema_type_str(s),
        Some(utoipa::openapi::RefOr::Ref(r)) => r
            .ref_location
            .split('/')
            .next_back()
            .unwrap_or("ref")
            .to_string(),
        None => "string".to_string(),
    }
}

fn get_schema_type_str(schema: &utoipa::openapi::Schema) -> String {
    match schema {
        utoipa::openapi::Schema::Object(obj) => format_schema_type(&obj.schema_type),
        utoipa::openapi::Schema::Array(arr) => {
            let item_type = match &arr.items {
                utoipa::openapi::schema::ArrayItems::RefOrSchema(item) => get_schema_type(item),
                utoipa::openapi::schema::ArrayItems::False => "any".to_string(),
            };
            format!("array<{}>", item_type)
        }
        _ => "unknown".to_string(),
    }
}

fn format_schema_type(schema_type: &utoipa::openapi::schema::SchemaType) -> String {
    match schema_type {
        utoipa::openapi::schema::SchemaType::Type(t) => type_to_string(t),
        utoipa::openapi::schema::SchemaType::AnyValue => "any".to_string(),
        utoipa::openapi::schema::SchemaType::Array(types) => types
            .iter()
            .map(type_to_string)
            .collect::<Vec<_>>()
            .join(" | "),
    }
}

fn type_to_string(t: &utoipa::openapi::Type) -> String {
    match t {
        utoipa::openapi::Type::String => "string".to_string(),
        utoipa::openapi::Type::Integer => "integer".to_string(),
        utoipa::openapi::Type::Number => "number".to_string(),
        utoipa::openapi::Type::Boolean => "boolean".to_string(),
        utoipa::openapi::Type::Object => "object".to_string(),
        utoipa::openapi::Type::Array => "array".to_string(),
        utoipa::openapi::Type::Null => "null".to_string(),
    }
}

fn format_schema(schema: &utoipa::openapi::RefOr<utoipa::openapi::Schema>) -> String {
    let mut md = String::new();

    match schema {
        utoipa::openapi::RefOr::T(s) => match s {
            utoipa::openapi::Schema::Object(obj) => {
                if !obj.properties.is_empty() {
                    md.push_str("| フィールド | 型 | 説明 |\n");
                    md.push_str("|------------|-----|------|\n");
                    for (name, prop) in &obj.properties {
                        let type_str = get_schema_type(prop);
                        let desc = get_schema_description(prop);
                        md.push_str(&format!("| {} | {} | {} |\n", name, type_str, desc));
                    }
                }
            }
            _ => {
                md.push_str("(詳細はOpenAPIスキーマを参照)\n");
            }
        },
        utoipa::openapi::RefOr::Ref(r) => {
            md.push_str(&format!("参照: `{}`\n", r.ref_location));
        }
    }

    md
}

fn get_schema_type(schema: &utoipa::openapi::RefOr<utoipa::openapi::Schema>) -> String {
    match schema {
        utoipa::openapi::RefOr::T(s) => get_schema_type_str(s),
        utoipa::openapi::RefOr::Ref(r) => r
            .ref_location
            .split('/')
            .next_back()
            .unwrap_or("ref")
            .to_string(),
    }
}

fn get_schema_description(schema: &utoipa::openapi::RefOr<utoipa::openapi::Schema>) -> String {
    match schema {
        utoipa::openapi::RefOr::T(utoipa::openapi::Schema::Object(obj)) => {
            obj.description.clone().unwrap_or_default()
        }
        _ => String::new(),
    }
}

fn format_schema_example(schema: &utoipa::openapi::RefOr<utoipa::openapi::Schema>) -> String {
    match schema {
        utoipa::openapi::RefOr::Ref(r) => {
            let name = r.ref_location.split('/').next_back().unwrap_or("Object");
            format!("// See {} schema", name)
        }
        utoipa::openapi::RefOr::T(s) => match s {
            utoipa::openapi::Schema::Object(obj) => {
                let mut example = String::from("{\n");
                for (name, prop) in &obj.properties {
                    let value = get_example_value(prop);
                    example.push_str(&format!("  \"{}\": {}\n", name, value));
                }
                example.push('}');
                example
            }
            _ => "{}".to_string(),
        },
    }
}

fn get_example_value(schema: &utoipa::openapi::RefOr<utoipa::openapi::Schema>) -> String {
    match schema {
        utoipa::openapi::RefOr::T(s) => match s {
            utoipa::openapi::Schema::Object(obj) => {
                let type_str = format_schema_type(&obj.schema_type);
                match type_str.as_str() {
                    "string" => "\"string\"".to_string(),
                    "integer" => "0".to_string(),
                    "number" => "0.0".to_string(),
                    "boolean" => "false".to_string(),
                    _ => "{}".to_string(),
                }
            }
            utoipa::openapi::Schema::Array(_) => "[]".to_string(),
            _ => "null".to_string(),
        },
        utoipa::openapi::RefOr::Ref(r) => {
            let name = r.ref_location.split('/').next_back().unwrap_or("Object");
            format!("{{ /* {} */ }}", name)
        }
    }
}
