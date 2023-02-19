use image::guess_format;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::errors::CustomError;

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

pub async fn upload_image_v2(body: &[u8]) -> Result<String, CustomError> {
    if let Some(file_extension) = get_file_extension(body) {
        let file_name = Uuid::new_v4().to_string();
        let rel_file_path = format!("./static/{}.{}", file_name, file_extension);
        let served_file_path: String = format!("/static/{}.{}", file_name, file_extension);
        let mut file = File::create(&rel_file_path).await.unwrap();
        file.write_all(&body).await.unwrap();
        Ok(served_file_path)
    } else {
        Err(CustomError::WrongFileExtension)
    }
}

fn get_file_extension(data: &[u8]) -> Option<&'static str> {
    if let Ok(format) = guess_format(data) {
        match format {
            image::ImageFormat::Png => Some("png"),
            image::ImageFormat::Jpeg => Some("jpg"),
            image::ImageFormat::Gif => Some("gif"),
            image::ImageFormat::WebP => Some("webp"),
            image::ImageFormat::Bmp => Some("bmp"),
            _ => None,
        }
    } else {
        None
    }
}
