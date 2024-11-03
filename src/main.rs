use git_info::GitInfo;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let token = String::from("fake_token");

    let git_info = GitInfo::new(token);

    let user = git_info.user("juanjo");

    println!("User: {:?}", user);

    Ok(())
}
