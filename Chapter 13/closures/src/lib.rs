mod application;
mod domain;
mod helpers;

pub fn run_giveaway() {
    for shirt in helpers::setup::shirts() {
        application::giveaway::run(&shirt, &helpers::setup::store());
    }
}

pub fn run_closures(numbers: Vec<u64>) {
    for num in numbers {
        application::closure_examples::expensive_closure()(num);
    }
}
