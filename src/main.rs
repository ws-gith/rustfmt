fn main() {
    let value = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    let vec_value = value.to_vec();

    let result = vec_value
        .iter()
        .map(|value| value * 2)
        .take(10)
        .map(|some_lang_ass_name_variables_argument| some_lang_ass_name_variables_argument * 2)
        .collect::<Vec<i32>>();

    result
        .iter()
        .for_each(|value| println!("{}", value));

    match result.get(0) {
        | Some(_) => todo!(),
        | None => todo!(),
    }
}
