// TODO: turn it into a macro that:
// in debug mode runs all arguments and validates that there's only one `true`
// in release mode runs all arguments until it fonds `true`
//
pub(crate) fn at_most_one_is_true<const N: usize>(values: [bool; N]) -> bool {
    let mut idxs = vec![];

    for (idx, v) in values.iter().enumerate() {
        if *v {
            idxs.push(idx)
        }
    }

    match idxs.len() {
        0 => false,
        1 => true,
        _ => {
            let formatted_idxs = idxs
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("/");
            panic!("Multiple rules match, indexes: {}", formatted_idxs)
        }
    }
}
