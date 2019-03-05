use std::collections::BTreeMap;

fn parse_kv(data: &str) -> Option<BTreeMap<String, String>> {
    data.split('&')
        .map(|kv| kv.split('='))
        .map(|mut kv| Some((kv.next()?.into(), kv.next()?.into())))
        .collect()
}

#[test]
fn parse_kv_test() {
    let good_result = parse_kv("test1=1&test2=2").unwrap();
    assert_eq!(good_result["test1"], "1");
    assert_eq!(good_result["test2"], "2");

    let bad_result = parse_kv("test1=1&test2");
    assert_eq!(bad_result, None);
}

fn escape_nasty_chars(data: &str) -> String {
    data.chars()
        .map(|ch| match ch {
            '&' => "%26".to_string(),
            '=' => "%3D".to_string(),
            _ => ch.to_string(),
        })
        .collect()
}

#[test]
fn escape_nasty_chars_test() {
    assert_eq!(escape_nasty_chars("rock&roll"), "rock%26roll");
    assert_eq!(escape_nasty_chars("2+2=4"), "2+2%3D4");
    assert_eq!(escape_nasty_chars("can't touch this"), "can't touch this");
}

fn profile_for(mail: &str) -> String {
    format!("email={}&uid=10&role=user", escape_nasty_chars(mail))
}

#[test]
fn profile_for_good_input() {
    assert_eq!(
        profile_for("foo@bar.com"),
        "email=foo@bar.com&uid=10&role=user"
    );
}

#[test]
fn profile_for_nasty_input() {
    assert_eq!(
        profile_for("foo@bar.com&role=admin"),
        "email=foo@bar.com%26role%3Dadmin&uid=10&role=user"
    );
}
