use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

type Men<'a> = HashMap<&'a str, Vec<&'a str>>;
type Women<'a> = HashMap<&'a str, Vec<&'a str>>;
type State<'a> = HashMap<&'a str, &'a str>;
type Score<'a> = HashMap<&'a str, HashMap<&'a str, i32>>;

fn stable_match<'a>(men: &'a mut Men, women: &'a mut Women) -> State<'a> {
    let mut state: State = HashMap::new();
    let score: Score = calculate_score(women.clone());
    let result = _stable_match(men, women, &mut state, score, false);
    result.clone()
}

fn _stable_match<'a>(
    men: &'a mut Men,
    women: &'a mut Women,
    state: &'a mut State,
    score: Score,
    stable: bool,
) -> State<'a> {
    if stable {
        return state.clone();
    }
    let (mut new_men, mut new_state) =
        men.iter()
            .fold((men.clone(), state.clone()), |(men, state), (m1, prefs)| {
                let m1_is_single = !state.contains_key(m1);
                if m1_is_single {
                    let w = prefs.first().unwrap();
                    let w_is_single = !state.contains_key(w);
                    let m2 = if w_is_single { None } else { state.get(w) };
                    let is_m1_preferred_to_m2 = if m2.is_some() {
                        score[w.clone()][m1.clone()] < score[w.clone()][m2.unwrap().clone()]
                    } else {
                        true
                    };

                    match (w_is_single, is_m1_preferred_to_m2) {
                        (true, true) => add_couple(&mut state.clone(), m1, w),
                        (false, true) => {
                            delete_couple(&mut state.clone(), m2.unwrap(), w);
                            add_couple(&mut state.clone(), m1, w);
                        }
                        _ => (),
                    };
                    let prefs = prefs[1..].to_vec();
                    men.clone().insert(m1, prefs);
                    (men.clone(), state.clone())
                } else {
                    (men.clone(), state.clone())
                }
            });

    let stable = is_stable(&new_men, &women, &new_state);
    _stable_match(&mut new_men, women, &mut new_state, score, stable)
}

fn calculate_score(women: Women) -> Score {
    let mut score_table: Score = HashMap::new();
    for (w, prefs) in women {
        let mut prefs_with_score: HashMap<&str, i32> = HashMap::new();
        let mut score = 0;
        for m in prefs {
            prefs_with_score.insert(m, score);
            score += 1;
        }
        score_table.insert(w, prefs_with_score);
    }

    score_table
}

fn is_m1_preferred_to_m2(w: &str, m1: &str, m2: Option<&&str>, score: Score) -> bool {
    unimplemented!()
}

fn add_couple<'a>(state: &mut State<'a>, m: &'a str, w: &'a str) {
    state.insert(m, w);
    state.insert(w, m);
}

fn delete_couple<'a>(state: &mut State<'a>, m: &'a str, w: &'a str) {
    state.remove(m);
    state.remove(w);
}

fn is_stable(men: &Men, women: &Women, state: &State) -> bool {
    men.keys().len() + women.keys().len() == state.keys().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_marriage_test() {
        let mut men = HashMap::new();
        men.insert("a", vec!["o", "m", "n", "l", "p"]);
        men.insert("b", vec!["p", "n", "m", "l", "o"]);
        men.insert("c", vec!["m", "p", "l", "o", "n"]);
        men.insert("d", vec!["p", "m", "o", "n", "l"]);
        men.insert("e", vec!["o", "l", "m", "n", "p"]);

        let mut women = HashMap::new();
        women.insert("l", vec!["d", "b", "e", "c", "a"]);
        women.insert("m", vec!["b", "a", "d", "c", "e"]);
        women.insert("n", vec!["a", "c", "e", "d", "b"]);
        women.insert("o", vec!["d", "a", "c", "b", "e"]);
        women.insert("p", vec!["b", "e", "a", "c", "d"]);

        let result = stable_match(&mut men, &mut women);
        println!("{:?}", result);
    }
}
