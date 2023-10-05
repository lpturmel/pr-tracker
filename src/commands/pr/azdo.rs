use crate::cli::pr::Azdo;
use crate::commands::Command;
use crate::config::SystemUser;
use async_trait::async_trait;
use console::style;

#[async_trait]
impl Command for Azdo {
    async fn execute(&self, cfg: crate::config::AppConfig) -> Result<(), crate::error::Error> {
        let token = cfg.token.ok_or(crate::error::Error::Unauthorized)?;

        let sys_user = SystemUser::from_env();
        let decode = crate::config::Token::decode(&token)?;
        let decrypted = crate::config::Token::decrypt(&decode, &sys_user.user)?;

        let azdo_client = crate::azdo::Client::new(&decrypted);

        let prs = self
            .repos
            .iter()
            .map(|repo| azdo_client.get_prs(&self.organization, &self.project, repo))
            .collect::<Vec<_>>();
        let prs = futures::future::join_all(prs).await;

        let prs = prs
            .iter()
            .flat_map(|pr| pr.as_ref().unwrap().value.iter())
            .collect::<Vec<_>>();

        for (i, pr) in prs.iter().enumerate() {
            if i > 0 {
                println!("\n");
            }
            let assigned_to_you = pr
                .reviewers
                .iter()
                .map(|r| r.unique_name.clone())
                .any(|email| email == cfg.email.clone().unwrap_or_default());
            let url = format!(
                "https://dev.azure.com/{}/{}/_git/{}/pullrequest/{}",
                self.organization, self.project, pr.repository.name, pr.pull_request_id,
            );
            let date = chrono::DateTime::parse_from_rfc3339(&pr.creation_date)?;
            println!(
                "({}) {} --- {} --- {} [{}]",
                date.format("%Y-%m-%d %H:%M:%S"),
                pr.repository.name,
                style(&pr.title).bold(),
                pr.pull_request_id,
                style(&url).underlined()
            );
            let description = pr.description.chars().take(60).collect::<String>();
            let description = format!(
                "{}{}",
                description,
                if description.len() > 20 { "..." } else { "" }
            );
            println!("{}", style(description).dim());
            if assigned_to_you {
                println!("{}", style("Assigned to you").black().on_green());
            }

            if i == prs.len() - 1 {
                println!(
                    "{}",
                    style("──────────────────────────────────────────────────────────────────")
                );
            }
        }

        Ok(())
    }
}
