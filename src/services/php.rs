use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

/// サーバーの静的ディレクトリに画像をアップロードする。
///
/// 画像の名前は、衝突を避けるためにランダムに変更されます。
///
/// # 引数
///
/// - body : 画像の内容。
pub async fn upload_image(body: &[u8], image_extension: String) -> String {
    let file_name = Uuid::new_v4().to_string();
    let rel_file_path = format!("./static/{}.{}", file_name, image_extension);
    let served_file_path: String = format!("/static/{}.{}", file_name, image_extension);
    let mut file = File::create(&rel_file_path).await.unwrap();
    file.write_all(&body).await.unwrap();
    served_file_path
}
