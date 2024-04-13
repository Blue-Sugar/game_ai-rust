use rand::*;

use crate::time_keeper::TimeKeeper;

#[allow(unused)]
#[derive(Clone, Copy)]
struct Player {
    // player の状態を表す。
}
#[allow(unused)]
impl Player {
    pub fn new(seed: u64) -> Self {
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
        // 初期化された player を返す関数
        Self {

        }
    }
}

#[derive(Clone, Copy)]
#[allow(unused)]
struct Board {
    // board の状態を表す。
}
#[allow(unused)]
impl Board {
    pub fn new(seed: u64) -> Self {
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
        // 初期化された盤面を返す関数。
        Self {

        }
    }
}

#[allow(unused)]
#[derive(Clone, Copy)]
enum Action {
    // player がし得る action 全体。
}
// Action のなかで player が取り得る action　全体。
#[allow(unused)]
const ACTIONS: [Action; 0] = [];
#[allow(unused)]
const INF: isize = std::isize::MAX / 2;

#[derive(Clone, Copy)]
#[allow(unused)]
struct GameState {
    board: Board,
    turn: usize,
    player: Player,
    game_score: isize,
    evaliated_score: isize,
    first_action: Option<Action>,
    end_turn: usize,
}
#[allow(unused)]
impl GameState {
    pub fn new(seed: u64) -> Self {
        // GameState をシード値を用いて作成する。
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
        let end_turn = rng.gen::<usize>();
        Self {
            board: Board::new(0),
            turn: 0,
            player: Player::new(0),
            game_score: 0,
            evaliated_score: 0,
            first_action: None,
            end_turn: end_turn,
        }
    }
    pub fn build(board: Board, player: Player, end_turn: usize) -> Self {
        // GameState を情報が与えられた前提で作成する。
        Self {
            board: board,
            turn: 0,
            player: player,
            game_score: 0,
            evaliated_score: 0,
            first_action: None,
            end_turn: end_turn,
        }
    }

    fn is_done(&self) -> bool {
        // game が終了しているかどうかを返す。
        self.turn == self.end_turn
    }

    fn advance(&mut self, action: Action) {
        // game が player が action することで一手進む関数。
    }

    fn legal_actions(&self) -> Vec<Action> {
        // ACTIONS 全体の中で許容される action の全体を返す関数        
        // 簡易実装では、true を返す。
        ACTIONS.into_iter().filter(|action| true).collect()
    }

    fn evaliated_score(&mut self) {
        // self.evaluated_score を更新す処理をする関数。
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.evaliated_score == other.evaliated_score
    }
}
impl Eq for GameState {
    fn assert_receiver_is_total_eq(&self) {}
}
impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.evaliated_score.cmp(&other.evaliated_score))
    }
}
impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// ここから先は、action を決定する方法を定める関数を作る。
#[allow(unused)]
fn random_action(state: &GameState) -> Option<Action> {
    let mut rng = rand::thread_rng();
    let legal_actions = state.legal_actions();
    if legal_actions.is_empty() {
        return None;
    }
    Some(legal_actions[rng.gen::<usize>() % legal_actions.len()])
}
#[allow(unused)]
fn greedy_action(state: &GameState) -> Option<Action> {
    let mut best_action = None;
    let mut best_score = -INF;
    for &action in &state.legal_actions() {
        let mut now_state = state.clone();
        now_state.advance(action);
        now_state.evaliated_score();
        if now_state.evaliated_score > best_score {
            best_action = Some(action);
            best_score = now_state.evaliated_score;
        }
    }
    best_action
}
#[allow(unused)]
fn beam_search_action(state: &GameState, beam_width: usize, beam_depth: usize) -> Option<Action> {
    let mut now_beam = std::collections::BinaryHeap::new();
    let mut best_state = state.clone();
    now_beam.push(*state);
    for t in 0..beam_depth {
        let mut next_beam = std::collections::BinaryHeap::new();
        for now_state in now_beam.iter().take(beam_width) {
            for &action in now_state.legal_actions().iter() {
                let mut next_state = now_state.clone();
                next_state.advance(action);
                next_state.evaliated_score();
                if t == 0 {
                    next_state.first_action = Some(action);
                }
                next_beam.push(next_state);
            }
        }
        now_beam = next_beam;
        best_state = *now_beam.peek().unwrap();
        if best_state.is_done() {
            break;
        }
    }
    best_state.first_action
}
#[allow(unused)]
fn chokudai_search_action(state: &GameState, beam_width: usize, beam_depth: usize, beam_number: usize) -> Option<Action> {
    let mut beams = vec![std::collections::BinaryHeap::new(); beam_depth + 1];
    beams[0].push(*state);
    for _ in 0..beam_number {
        for t in 0..beam_depth {
            let mut next_beam = std::mem::take(&mut beams[t + 1]);
            for now_state in beams[t].iter().take(beam_width) {
                if now_state.is_done() {
                    break;
                }
                for &action in now_state.legal_actions().iter() {
                    let mut next_state = now_state.clone();
                    next_state.advance(action);
                    next_state.evaliated_score();
                    if t == 0 {
                        next_state.first_action = Some(action);
                    }
                    next_beam.push(next_state);
                }
            }
            std::mem::swap(&mut beams[t + 1], &mut next_beam);
        }
    }
    for beam in beams.iter().rev() {
        if !beam.is_empty() {
            return beam.peek().unwrap().first_action;
        }
    }
    None
}
#[allow(unused)]
fn beam_search_action_with_time_threshold(state: &GameState, beam_width: usize, time_threshold: f64) -> Option<Action> {
    let time_keeper = TimeKeeper::build(time_threshold);
    let mut now_beam = std::collections::BinaryHeap::new();
    let mut best_state = state.clone();
    now_beam.push(*state);
    for t in 0.. {
        let mut next_beam = std::collections::BinaryHeap::new();
        for now_state in now_beam.iter().take(beam_width) {
            if time_keeper.is_time_over() {
                return best_state.first_action;
            }
            for &action in now_state.legal_actions().iter() {
                let mut next_state = now_state.clone();
                next_state.advance(action);
                next_state.evaliated_score();
                if t == 0 {
                    next_state.first_action = Some(action);
                }
                next_beam.push(next_state);
            }
        }
        now_beam = next_beam;
        best_state = *now_beam.peek().unwrap();
        if best_state.is_done() {
            break;
        }
    }
    best_state.first_action
}
#[allow(unused)]
fn chokudai_search_action_with_time_threshold(state: &GameState, beam_width: usize, beam_depth: usize, time_threshold: f64) -> Option<Action> {
    let time_keeper = TimeKeeper::build(time_threshold);
    let mut beams = vec![std::collections::BinaryHeap::new(); beam_depth + 1];
    beams[0].push(*state);
    for _ in 0.. {
        for t in 0..beam_depth {
            let mut next_beam = std::mem::take(&mut beams[t + 1]);
            for now_state in beams[t].iter().take(beam_width) {
                if now_state.is_done() {
                    break;
                }
                for &action in now_state.legal_actions().iter() {
                    let mut next_state = now_state.clone();
                    next_state.advance(action);
                    next_state.evaliated_score();
                    if t == 0 {
                        next_state.first_action = Some(action);
                    }
                    next_beam.push(next_state);
                }
            }
            beams[t + 1] = next_beam;
        }
        if time_keeper.is_time_over() {
            break;
        }
    }
    for beam in beams.iter().rev() {
        if !beam.is_empty() {
            return beam.peek().unwrap().first_action;
        }
    }
    None
}

#[allow(unused)]
pub fn play_game(seed: u64) -> isize {
    let mut state = GameState::new(seed);
    while !state.is_done() {
        if let Some(action) = beam_search_action(&state, 2, 5) {
            state.advance(action);
        } else {
            return -INF;
        }
    }
    state.game_score
}
#[allow(unused)]
pub fn test_ai_score(game_number: usize, seed: u64) -> f64 {
    let mut rngs: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
    let mut score_sum = 0;
    for _ in 0..game_number {
        score_sum += play_game(rngs.gen::<u64>()) as i128;
    }
    score_sum as f64 / game_number as f64
}