use shared::{
    domain::image::ImageKind,
    media::{FileKind, MediaLibrary, PngImageFile},
};
use sqlx::PgPool;

use crate::error;

pub async fn watch_image(db: &PgPool, s3: &crate::s3::Client) -> anyhow::Result<bool> {
    let mut txn = db.begin().await?;

    let row = sqlx::query!(
        r#"
select id, kind as "kind: ImageKind"
from image_metadata
inner join image_upload on image_metadata.id = image_upload.image_id
where uploaded_at is not null and processed_at >= uploaded_at is not true
for no key update of image_upload
for share of image_metadata
skip locked
"#
    )
    .fetch_optional(&mut txn)
    .await?;

    let row = match row {
        Some(row) => row,
        None => {
            txn.rollback().await?;
            return Ok(false);
        }
    };

    let file = s3
        .download_media_file(
            MediaLibrary::Global,
            row.id,
            FileKind::ImagePng(PngImageFile::Original),
        )
        .await?;

    let file = match file {
        Some(it) => it,
        None => {
            sqlx::query!("update image_upload set processed_at = now(), processing_result = false where image_id = $1", row.id)
                .execute(&mut txn)
                .await?;

            log::warn!("Image wasn't uploaded properly before processing?");
            txn.commit().await?;
            return Ok(true);
        }
    };

    let kind = row.kind;

    let processed = tokio::task::spawn_blocking(move || -> Result<_, error::Upload> {
        let original = image::load_from_memory(&file).map_err(|_| error::Upload::InvalidMedia)?;
        Ok(crate::image_ops::regenerate_images(&original, kind)?)
    })
    .await
    .unwrap();

    let (resized, thumbnail) = match processed {
        Ok(it) => it,
        Err(error::Upload::InvalidMedia) => {
            sqlx::query!("update image_upload set processed_at = now(), processing_result = false where image_id = $1", row.id)
                .execute(&mut txn)
                .await?;

            txn.commit().await?;
            return Ok(true);
        }
        Err(error::Upload::InternalServerError(e)) => return Err(e),
        Err(_) => unreachable!(),
    };

    s3.upload_png_images_resized_thumb(MediaLibrary::Global, row.id, resized, thumbnail)
        .await?;

    sqlx::query!("update image_upload set processed_at = now(), processing_result = true where image_id = $1", row.id).execute(&mut txn).await?;

    txn.commit().await?;

    Ok(true)
}