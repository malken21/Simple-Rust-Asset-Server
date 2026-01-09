use axum::{
    extract::{Multipart, DefaultBodyLimit},
    response::{Json, IntoResponse},
    routing::post,
    Router,
    http::StatusCode,
};
use serde_json::json;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use std::path::Path;
use tokio::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use tokio::io::AsyncWriteExt;
use sha2::{Sha256, Digest};

#[tokio::main]
async fn main() {
    // public/uploadsディレクトリの作成
    if let Err(e) = fs::create_dir_all("public/uploads").await {
        eprintln!("Failed to create public/uploads: {}", e);
        return;
    }

    // ルーターの設定
    let app = Router::new()
        // アップロード用エンドポイント
        .route("/upload", post(upload_handler))
        // 静的ファイル配信 (http://localhost:3000/uploads/xxx.glb)
        .nest_service("/uploads", ServeDir::new("public/uploads"))
        // CORS設定 (すべてのオリジンを許可)
        .layer(CorsLayer::permissive())
        // アップロードサイズ制限を解除 (必要に応じて設定)
        .layer(DefaultBodyLimit::disable());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Asset Server running on port 3000");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn upload_handler(mut multipart: Multipart) -> impl IntoResponse {
    // マルチパートデータを解析してファイルを保存
    while let Some(mut field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        
        // 'file' というフィールド名を探す
        if name == "file" {
            let file_name = field.file_name().unwrap_or("unknown").to_string();
            
            // ファイル名の拡張子を取得
            let ext = Path::new(&file_name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            let ext_str = if ext.is_empty() { "".to_string() } else { format!(".{}", ext) };

            // 一時ファイル名を生成
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let random_part: u32 = rand::thread_rng().gen_range(0..1_000_000_000);
            let temp_name = format!("temp-{}-{}", timestamp, random_part);
            let temp_filepath = format!("public/uploads/{}", temp_name);

            // ファイルをストリーム書き込み
            let mut file = match fs::File::create(&temp_filepath).await {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("File create error: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "File save error").into_response();
                }
            };

            let mut hasher = Sha256::new();

            while let Ok(Some(chunk)) = field.chunk().await {
                hasher.update(&chunk);
                if let Err(e) = file.write_all(&chunk).await {
                    eprintln!("File write error: {}", e);
                    let _ = fs::remove_file(&temp_filepath).await;
                    return (StatusCode::INTERNAL_SERVER_ERROR, "File write error").into_response();
                }
            }

            if let Err(e) = file.flush().await {
                eprintln!("File flush error: {}", e);
                let _ = fs::remove_file(&temp_filepath).await;
                return (StatusCode::INTERNAL_SERVER_ERROR, "File flush error").into_response();
            }
            // ロック解除をするためファイルを明示的にドロップ
            drop(file);

            // SHA-256ハッシュを計算してファイル名を決定
            let hash = hasher.finalize();
            let hash_hex = hex::encode(hash);
            let unique_name = format!("{}{}", hash_hex, ext_str);
            let final_filepath = format!("public/uploads/{}", unique_name);

            if Path::new(&final_filepath).exists() {
                // 同一ファイルが存在する場合は一時ファイルを削除
                let _ = fs::remove_file(&temp_filepath).await;
            } else {
                // 名前を変更
                if let Err(e) = fs::rename(&temp_filepath, &final_filepath).await {
                    eprintln!("File rename error: {}", e);
                    let _ = fs::remove_file(&temp_filepath).await;
                    return (StatusCode::INTERNAL_SERVER_ERROR, "File rename error").into_response();
                }
            }

            // URLを生成して返す
            let file_url = format!("http://localhost:3000/uploads/{}", unique_name);
            println!("[Asset Server] File Uploaded: {}", file_url);
            return Json(json!({ "url": file_url })).into_response();
        }
    }

    // ファイルが見つからなかった場合
    (StatusCode::BAD_REQUEST, "No file uploaded.").into_response()
}
