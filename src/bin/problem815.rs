// [0, 0] / 0 : 1
//
// [1, 0] / 1 : 8
//
// [2, 0] / 1 : 8 * 3  = 24
// [1, 1] / 2 : 8 * 4 = 32
//
// [3, 0] / 1 : 8 * 3 * 2 = 48
// [2, 1] / 2 : 8 * 3 * 4 + 8 * 4 * 6 = 288
//
// [4, 0] / 1 : 8 * 3 * 2 * 1 = 48
// [3, 1] / 2 : 8 * 3 * 2 * 4 + 288 * 2 = 768
// [2, 2] / 2 : 288 * 3 = 864
//
// [4, 1] / 1 : 48 * 4 = 192
// [4, 1] / 2 : 768
// [3, 2] / 2 : 768 * 3 + 864 * 4 = 5760
//
// [4, 2] / 1 : 192 * 3 = 576
// [4, 2] / 2 : 768 * 3 + 5760 * 1 = 8064
// [3, 3] / 2 : 5760 * 2 = 11520
//
// [4, 3] / 1 : 576 * 2 = 1152
// [4, 3] / 2 : 8064 * 2 + 11520 * 2 = 39168
//
// [4, 4] / 1 : 1152
// [4, 4] / 2 : 39168

use std::collections::{HashMap, LinkedList};

fn main() {
    println!("{:.8}", solve(60));
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct State {
    heaps: [u32; 5],
    max: u32,
}

fn solve(n: u32) -> f64 {
    let mut state_prob: HashMap<State, f64> = HashMap::new();
    state_prob.insert(
        State {
            heaps: [n, 0, 0, 0, 0],
            max: 0,
        },
        1.0,
    );
    let mut queue: LinkedList<State> = LinkedList::new();
    queue.push_back(State {
        heaps: [n, 0, 0, 0, 0],
        max: 0,
    });

    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        let total: u32 = (0..=3).map(|i| (4 - i as u32) * state.heaps[i]).sum();
        for idx in 0..=3 {
            if state.heaps[idx] == 0 {
                continue;
            }
            let mut new_state_heaps = state.heaps.clone();
            new_state_heaps[idx] -= 1;
            new_state_heaps[idx + 1] += 1;

            let new_state_max = u32::max(
                state.max,
                new_state_heaps[1] + new_state_heaps[2] + new_state_heaps[3],
            );
            let new_state = State {
                heaps: new_state_heaps,
                max: new_state_max,
            };

            let new_state_prob = state_prob.get(&state).unwrap()
                * (((4 - idx as u32) * state.heaps[idx]) as f64)
                / (total as f64);

            // println!("{:?} {:?} {:?}", state, new_state, new_state_cnt);
            if state_prob.contains_key(&new_state) {
                state_prob
                    .entry(new_state.clone())
                    .and_modify(|e| *e += new_state_prob);
            } else {
                state_prob.insert(new_state.clone(), new_state_prob);
                queue.push_back(new_state);
            }
        }
    }

    return (0..=n)
        .map(|max| {
            (max as f64)
                * state_prob
                    .get(&State {
                        heaps: [0, 0, 0, 0, n],
                        max: max,
                    })
                    .unwrap_or(&0_f64)
        })
        .sum();
}
