use rand::seq::SliceRandom;

static OPTIONS: &[&str] = &[
    "Dude looks like a lady.",
    "I couldn't find my keys.",
    "I had temporary amnesia.",
    "I had to shampoo my cat.",
    "I have temporary insanity most of the time.",
    "I locked my keys in my car.",
    "I need to take my tarantula for a walk.",
    "I was abducted by aliens.",
    "I was captured by terrorists.",
    "I'm schizophrenic and my other personality is a flake.",
    "It's only me and not my mind that is confusing things.",
    "My dog caught fire.",
    "My evil twin did that.",
    "My iguana threw up on my homework.",
    "My mother always liked my evil twin best.",
    "My wife wouldn't let me out of the house.",
    "She looked 18 to me.",
    "Thanks, Obama!",
    "That wasn't me, that was my parrot who said that.",
    "The devil made me do it.",
];

fn main() {
    println!("{}", message());
}

fn message() -> &'static str {
    OPTIONS.choose(&mut rand::thread_rng()).expect("Unreachable")
}
