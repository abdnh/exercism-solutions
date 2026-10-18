use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    let mut reversed = input.to_string();
    reverse_grapheme_clusters_in_place(&mut reversed);

    reversed
}
