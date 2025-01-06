use anyhow::Result;
use gitload::DownloaderBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let downloader = DownloaderBuilder::new("levinion", "dotfiles", "nvim")
        .local_path("./src")
        .build();
    downloader.download().await
}