pub fn check_flags(flags: &[String]) -> bool {
    let lst_flags: Vec<&str> = vec![
        "-m", "--map",
        "-c", "--captured",
        "-r", "--range",
        "-a", "--alignement"
    ];
    match flags.iter().map(|x| {
        let flag: Vec<&str> = x.split('=').collect();
        if lst_flags.iter().any(|z| *z == flag[0]) {
            return true
        }
        false
    }).find(|x| *x == false) {
        Some(false) => false,
        _ => true
    }
}
