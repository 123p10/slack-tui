use reqwest::Client;
use crate::app;

static USERS_ID_URL : &str = "https://slack.com/api/users.identity";


pub async fn get_profile_info(client: &mut Client, token: String) -> app::ProfileInfo
{
//    let res = client.get(USERS_ID_URL).header("Authorization", "Bearer ".to_owned() + &token).header("Cookie", String::from("d=xoxd-FNVd93OzsG4uFaYfzpf3Bzy4cFe%2F9HrXALNIjKrAX98mx2qcX7SWJ%2FjFK%2FDwsJKhba89W1J%2B574QDhploCq5oR04LNzxyZwOqS49VQKNYL%2BYFGUGSN%2FxI0raJNu3jKwuCCbQo3Yds8KTSs98RxKkOkqBZ7rTsc2IABXQH8tY2feqWuVE6WCShUkD")).send().await.unwrap();
   let res = client.get("https://slack.com/api/team.info?token=xoxc-433343262982-759574558630-746415958595-60bdeaec9788d11832b04530e641f09f09586221b85c35b1cfa0aef8c1ab1810").header("Authorization", "Bearer ".to_owned() + &token).header("Cookie", String::from("d=xoxd-FNVd93OzsG4uFaYfzpf3Bzy4cFe%2F9HrXALNIjKrAX98mx2qcX7SWJ%2FjFK%2FDwsJKhba89W1J%2B574QDhploCq5oR04LNzxyZwOqS49VQKNYL%2BYFGUGSN%2FxI0raJNu3jKwuCCbQo3Yds8KTSs98RxKkOkqBZ7rTsc2IABXQH8tY2feqWuVE6WCShUkD")).send().await.unwrap();
    let txt = res.text().await.unwrap();
    println!("{}", txt);
    log::info!("{}", txt);
    app::ProfileInfo {
        name: String::from(""),
        token: String::from("")
    }
}

