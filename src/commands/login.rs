use super::Command;
use crate::cli::login::Login;
use crate::config::{AppConfig, SystemUser};
use crate::emoji::SPARKLES;
use crate::provider::Provider;
use async_trait::async_trait;

#[async_trait]
impl Command for Login {
    async fn execute(&self, cfg: crate::config::AppConfig) -> Result<(), crate::error::Error> {
        let (display_name, email) = match self.provider {
            Provider::AzureDevOps => {
                let azdo_client = crate::azdo::Client::new(&self.token);
                let profile = azdo_client.get_profile().await?;

                (
                    profile.authenticated_user.provider_display_name,
                    profile.authenticated_user.properties.account.value,
                )
            }
            Provider::GitHub => {
                todo!()
            }
        };

        let sys_user = SystemUser::from_env();
        let encrypted_token = crate::config::Token::encrypt(&self.token, &sys_user.user)?;
        let encoded = crate::config::Token::encode(&encrypted_token);

        let new_cfg = AppConfig {
            token: Some(encoded),
            email: Some(email.clone()),
            ..cfg
        };

        confy::store(crate::APP_NAME, None, new_cfg)?;

        println!(
            "{} Successfully logged in as: {} ({})",
            SPARKLES, display_name, email
        );

        Ok(())
    }
}
