use regex::RegexBuilder;

#[test]
fn block_comment() {
    let re = RegexBuilder::new(r"\/\*([^*]|\*+[^\*\/])*\*+\/")
        .multi_line(true)
        .build()
        .unwrap();

    let hey = r#"as/ *casaaaaa*/dbz
zaha/q*eg
acsaz/*asomcaoccasz
acscz//dc
sazcef aa*ada
sacsacz**//axsz
sa/cz*das/da*/aczc
sacpm /*sca/asc
asco//maslm**a/cas
sacca
"#;
    let res = r#"/*asomcaoccasz
acscz//dc
sazcef aa*ada
sacsacz**/"#;

    let matches: Vec<_> = re.find_iter(hey).map(|m| m.as_str()).collect();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0], res);
}
