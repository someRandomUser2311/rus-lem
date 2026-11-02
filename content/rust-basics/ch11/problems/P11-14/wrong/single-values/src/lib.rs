#[cfg(test)]
mod tests {
    use super::*;

    struct Book {
        title: String,
    }

    #[test]
    fn push_then_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.len(), 1, "one push");
        assert!(!stack.is_empty(), "one value");
        assert_eq!(stack.peek(), Some(&1), "the value is on top");
        assert_eq!(stack.pop(), Some(1), "pop gives it back");
        assert_eq!(stack.pop(), None, "then nothing is left");
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
    }

    #[test]
    fn clear_removes_the_value() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.clear();
        assert_eq!(stack.peek(), None, "nothing left after clear");
    }

    #[test]
    fn require_top_borrows_the_top() {
        let mut stack = Stack::new();
        stack.push(String::from("top"));
        assert_eq!(stack.require_top(), "top");
    }

    #[test]
    #[should_panic(expected = "stack is empty")]
    fn require_top_panics_when_empty() {
        let stack: Stack<u8> = Stack::new();
        stack.require_top();
    }

    #[test]
    fn top_index_is_the_last_position() {
        let mut stack = Stack::new();
        assert_eq!(stack.top_index(), None, "an empty stack has no top");
        stack.push(1);
        assert_eq!(stack.top_index(), Some(0), "one value: the top is at 0");
    }

    #[test]
    fn holds_values_that_cannot_be_copied() {
        let mut stack = Stack::new();
        stack.push(Book { title: String::from("Été") });
        let top = stack.peek().expect("a book was just pushed");
        assert_eq!(top.title, "Été", "peek shows the book");
    }
}
