extern crate gdk;
extern crate gtk;


pub struct Stack {
    pub stack: gtk::Stack,
    pub windows: Vec<String>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: gtk::Stack::new(),
            windows: Vec::new(),
        }
    }

    pub fn create_windows(&mut self, title: &str) {
        let label = gtk::Label::new(Some(title));
        self.stack.add_named(&label, &title);

        self.windows.push(title.to_string());
    }

    pub fn get_window_index(&self, title: &str) -> Option<usize> {
        let index = &self.windows.iter().position(|w| w == title);
        *index
    }

    fn current_window(&self) -> Option<String> {
        self.stack.get_visible_child_name()
    }

    pub fn next_window(&self) {
        let mut iter = self.windows.iter().cycle();
        let current_window = self.get_window_index(&self.current_window().unwrap()).unwrap();
        let next_window = iter.nth(current_window+1).unwrap();
        self.stack.set_visible_child_full(next_window, gtk::StackTransitionType::SlideLeftRight)
    }

    pub fn prev_window(&self) {
        let mut iter = self.windows.iter().cycle();
        let current_window = self.get_window_index(&self.current_window().unwrap()).unwrap();
        let next_window = match current_window {
            0 => self.windows.last(),
            _ => iter.nth(current_window-1),
        };
        self.stack.set_visible_child_full(next_window.unwrap(), gtk::StackTransitionType::SlideLeftRight)
    }


}

