use std::path::{Path, PathBuf};

pub trait ExternalSaveSource {
    type Error;

    fn get_save_path(pkg: &str) -> PathBuf {
        PathBuf::from("/data")
            .join("data")
            .join(pkg)
            .join("files")
            .join("SAVE_DATA")
    }
    fn get_account_info_path(pkg: &str, inquiry_code: &str) -> PathBuf {
        PathBuf::from("/data")
            .join("data")
            .join(pkg)
            .join("cache")
            .join(format!("{inquiry_code}.json"))
    }

    fn write_save(
        &mut self,
        data: Vec<u8>,
        pkg: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send
    where
        Self: Send,
    {
        async {
            let path = Self::get_save_path(pkg);

            self.write_path(data, &path).await
        }
    }
    fn read_save(
        &mut self,
        pkg: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>> + Send
    where
        Self: Send,
    {
        async {
            let path = Self::get_save_path(pkg);

            self.read_path(&path).await
        }
    }

    fn read_account_info(
        &mut self,
        pkg: &str,
        inquiry_code: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>> + Send
    where
        Self: Send,
    {
        async {
            self.read_path(&Self::get_account_info_path(pkg, inquiry_code))
                .await
        }
    }

    fn read_path(
        &mut self,
        path: &Path,
    ) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    fn write_path(
        &mut self,
        data: Vec<u8>,
        path: &Path,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    fn close_game(
        &mut self,
        pkg: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
    fn run_game(
        &mut self,
        pkg: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    fn rerun_game(
        &mut self,
        pkg: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send
    where
        Self: Send,
    {
        async {
            self.close_game(pkg).await?;
            self.run_game(pkg).await?;

            Ok(())
        }
    }

    fn get_all_game_packages(
        &mut self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Self::Error>> + Send;
}
