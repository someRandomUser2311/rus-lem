#[cfg(test)]
mod tests {
    use super::*;

    struct Book {
        title: String,
    }

    #[test]
    fn values_come_back_newest_first() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3, "after three pushes");
        assert_eq!(stack.pop(), Some(3), "the first pop takes the last push");
        assert_eq!(stack.peek(), Some(&2), "2 is on top after one pop");
        assert_eq!(stack.len(), 2, "after one pop");
        assert_eq!(stack.pop(), Some(2), "the second pop");
        assert_eq!(stack.pop(), Some(1), "the third pop");
        assert!(stack.is_empty(), "after three pops");
        assert_eq!(stack.pop(), None, "a fourth pop finds nothing");
    }

    #[test]
    fn an_empty_stack_has_nothing() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.len(), 0, "a new stack");
        assert!(stack.is_empty(), "a new stack is empty");
        assert_eq!(stack.peek(), None, "nothing to peek at");
        assert_eq!(stack.pop(), None, "nothing to pop");
    }

    #[test]
    fn pushes_keep_earlier_values() {
        let mut stack = Stack::new();
        stack.push('a');
        stack.push('b');
        assert_eq!(stack.len(), 2, "two pushes");
        assert!(!stack.is_empty(), "two values");
        assert_eq!(stack.peek(), Some(&'b'), "the newest is on top");
    }

    #[test]
    fn clear_removes_every_value() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        stack.push(30);
        stack.clear();
        assert_eq!(stack.len(), 0, "after clear");
        assert_eq!(stack.peek(), None, "nothing left to peek at");
        stack.push(40);
        assert_eq!(stack.pop(), Some(40), "the stack works after clear");
        assert_eq!(stack.pop(), None, "and holds only the new value");
    }

    #[test]
    fn require_top_borrows_the_top() {
        let mut stack = Stack::new();
        stack.push(String::from("bottom"));
        stack.push(String::from("top"));
        assert_eq!(stack.require_top(), "top");
    }

    #[test]
    #[should_panic]
    fn require_top_panics_when_empty() {
        let stack: Stack<u8> = Stack::new();
        stack.require_top();
    }

    #[test]
    fn top_index_is_the_last_position() {
        let mut stack = Stack::new();
        assert_eq!(stack.top_index(), None, "an empty stack has no top");
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.top_index(), Some(2), "three values: the top is at 2");
    }

    #[test]
    fn holds_values_that_cannot_be_copied() {
        let mut stack = Stack::new();
        stack.push(Book { title: String::from("Été") });
        stack.push(Book { title: String::from("東京") });
        let top = stack.peek().expect("a book was just pushed");
        assert_eq!(top.title, "東京", "peek shows the last book");
        let popped = stack.pop().expect("there are two books");
        assert_eq!(popped.title, "東京", "pop gives the last book back, owned");
        assert_eq!(stack.require_top().title, "Été", "the first book is on top again");
    }
}
