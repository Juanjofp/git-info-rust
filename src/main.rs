use git_info::GitInfo;

fn main() -> anyhow::Result<()> {
    println!("Hello, Github!");

    let token = String::from("fake_token");

    let git_info = GitInfo::new(token);

    let user = git_info.user("octokit");

    let Ok(user) = user else {
        println!("Error: {:?}", user.unwrap_err());
        return Err(anyhow::anyhow!("Error user request"));
    };

    println!("User: {}", user.name);

    let me = git_info.me();
    let Ok(me) = me else {
        println!("Error: {:?}", me.unwrap_err());
        return Err(anyhow::anyhow!("Error me request"));
    };

    println!("Me: {}", me.name);

    let repos = git_info.repositories("juanjofp");
    let Ok(repos) = repos else {
        println!("Error: {:?}", repos.unwrap_err());
        return Err(anyhow::anyhow!("Error repos request"));
    };

    repos.iter().for_each(|repo| {
        println!("Repo: {} {}", repo.name, repo.description);
    });

    let repo = git_info.repository("juanjofp", "git-info-rust");
    let Ok(repo) = repo else {
        println!("Error: {:?}", repo.unwrap_err());
        return Err(anyhow::anyhow!("Error repo request"));
    };

    println!("Repo: {} {}", repo.name, repo.description);

    let events = git_info.events("juanjofp");
    let Ok(events) = events else {
        println!("Error: {:?}", events.unwrap_err());
        return Err(anyhow::anyhow!("Error events request"));
    };

    events.iter().for_each(|event| {
        println!("Event: {} {}", event.kind, event.created_at);
    });


    Ok(())
}
