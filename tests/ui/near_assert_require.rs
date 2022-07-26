#![warn(clippy::near_assert_require)]

fn main() {
    // test code goes here
    let x = 10;
    // assert!(x < 10 * 8);
    // foo()
}

pub struct StatusMessage {
    // records: HashMap<AccountId, String>,
}

impl StatusMessage {
    pub fn foo() {
        let y= 11;
        assert!(y < 10 * 8);
    }
}
