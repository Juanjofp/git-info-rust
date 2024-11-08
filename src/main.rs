use git_info::GitInfo;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let token = String::from("fake_token");

    let git_info = GitInfo::new(token);

    let user = git_info.user("octokit");

    println!("User: {:?}", user);

    let me = git_info.me();

    println!("Me: {:?}", me);

    Ok(())
}
