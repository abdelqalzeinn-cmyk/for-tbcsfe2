pub trait ExternalSaveSource {
    type Error;

    fn write_save(
        &mut self,
        data: Vec<u8>,
        pkg: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
    fn read_save(
        &mut self,
        pkg: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Self::Error>> + Send;

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
