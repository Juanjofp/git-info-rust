pub struct UserJsonMock;

impl UserJsonMock {
pub fn user() -> String {
  r#"{"login":"Juanjofp","id":446496,"node_id":"MDQ6VXNlcjQ0NjQ5Ng==","avatar_url":"https://avatars.githubusercontent.com/u/446496?v=4","gravatar_id":"","url":"https://api.github.com/users/Juanjofp","html_url":"https://github.com/Juanjofp","followers_url":"https://api.github.com/users/Juanjofp/followers","following_url":"https://api.github.com/users/Juanjofp/following{/other_user}","gists_url":"https://api.github.com/users/Juanjofp/gists{/gist_id}","starred_url":"https://api.github.com/users/Juanjofp/starred{/owner}{/repo}","subscriptions_url":"https://api.github.com/users/Juanjofp/subscriptions","organizations_url":"https://api.github.com/users/Juanjofp/orgs","repos_url":"https://api.github.com/users/Juanjofp/repos","events_url":"https://api.github.com/users/Juanjofp/events{/privacy}","received_events_url":"https://api.github.com/users/Juanjofp/received_events","type":"User","user_view_type":"public","site_admin":false,"name":"Juanjo","company":"Digio","blog":"http://juanjofp.com","location":"Murcia, Spain","email":"juanjo@juanjofp.com","hireable":null,"bio":null,"twitter_username":null,"notification_email":"juanjo@juanjofp.com","public_repos":62,"public_gists":5,"followers":37,"following":75,"created_at":"2010-10-20T07:04:01Z","updated_at":"2024-10-14T10:54:05Z"}"#.to_string()
}

pub fn without_login() -> String {
  r#"{"id":446496,"node_id":"MDQ6VXNlcjQ0NjQ5Ng==","avatar_url":"https://avatars.githubusercontent.com/u/446496?v=4","gravatar_id":"","url":"https://api.github.com/users/Juanjofp"}"#.to_string()
}

// pub fn users_json() -> String {
//   format!("[{}]", mock_user_json())
// }
}