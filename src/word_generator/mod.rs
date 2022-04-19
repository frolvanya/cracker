use itertools::Itertools;

pub fn generate(preferences: Vec<bool>) -> impl Iterator<Item = String> {
    let mut charset = String::new();

    if preferences[0] {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }

    if preferences[1] {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    if preferences[2] {
        charset.push_str("0123456789");
    }

    if preferences[3] {
        charset.push_str("!\"#$%&'()*+,-./:;<=>?@^_`{|}~")
    }

    (1..=20)
        .flat_map(move |len| {
            charset
                .clone()
                .chars()
                .combinations_with_replacement(len)
                .map(move |combos| (combos, len))
                .collect::<Vec<_>>()
        })
        .flat_map(|(combos, len)| combos.into_iter().permutations(len))
        .dedup()
        .map(|chars| chars.into_iter().collect())
}
