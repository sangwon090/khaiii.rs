use khaiii::KhaiiiApi;

#[test]
fn test_version() {
    let api = KhaiiiApi::new();
    assert_eq!(api.version(), "0.4");
}

#[test]
#[should_panic(expected = "resource directory not found: /not/existing/dir")]
fn test_open() {
    let mut api = KhaiiiApi::new();

    api.open("".into(), "".into()).unwrap();
    api.open("/not/existing/dir".into(), "".into()).unwrap();
}

#[test]
fn test_analyze() {
    let mut api = KhaiiiApi::new();
    api.open("".into(), "".into()).unwrap();

    let words = api.analyze("안녕? 반가워!".into(), "".into()).unwrap();

    assert_eq!(words[0].morphs.len(), 2);
    assert_eq!(words[0].morphs[0].lex, "안녕");
    assert_eq!(words[0].morphs[0].tag, "IC");
    assert_eq!(words[0].morphs[1].lex, "?");
    assert_eq!(words[0].morphs[1].tag, "SF");
    assert_eq!(words[1].morphs.len(), 3);
    assert_eq!(words[1].morphs[0].lex, "반갑");
    assert_eq!(words[1].morphs[0].tag, "VA");
    assert_eq!(words[1].morphs[1].lex, "어");
    assert_eq!(words[1].morphs[1].tag, "EF");
    assert_eq!(words[1].morphs[2].lex, "!");
    assert_eq!(words[1].morphs[2].tag, "SF");
}

#[test]
fn test_analyze_bfr_errpatch() {
    let mut api = KhaiiiApi::new();
    api.open("".into(), "".into()).unwrap();

    for foo in api.analyze_bfr_errpatch("테스트".into(), "".into()).unwrap() {
        println!("{}", foo);
    }

    assert_eq!(api.analyze_bfr_errpatch("테스트".into(), "".into()).unwrap().len(), "테스트".chars().count() + 2)
}

#[test]
#[should_panic(expected = "invalid log level: not_existing_level")]
fn test_set_log_level() {
    let api = KhaiiiApi::new();

    api.set_log_level("all".into(), "info".into()).unwrap();
    api.set_log_level("all".into(), "not_existing_level".into()).unwrap();
}