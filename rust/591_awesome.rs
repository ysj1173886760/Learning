use std::any::type_name;
impl Solution {
    fn type_of<T>(_: &T) -> &'static str {
        type_name::<T>()
    }
    pub fn is_valid(code: String) -> bool {
        code.chars().try_fold(Validator::new(), |mut validator, c| {
            validator.consume_one(c).and_then(|_| Ok(validator))
        })
        .and_then(|validator| 
            match validator.is_end() {
            true => Ok(()),
            false => Err("failed"),
        })
        .is_ok()
    }
}

enum State {
    Init,
    TagName {
        name: String,
        is_end_tag: bool,
    },
    TagContent,
    CDataTag {
        name: String,
    },
    CDataContent {
        // matching ]]>
        prefix: (bool, bool),
    },
    End,
}

struct Validator {
    state: State,
    stack: Vec<String>,
}

impl Validator {
    fn new() -> Self {
        Self {
            state: State::Init,
            stack: Vec::new(),
        }
    }

    fn is_end(&self) -> bool {
        match self.state {
            State::End => true,
            _ => false
        }
    }

    // consume one char, dispatch to the handler corresponding to the state
    fn consume_one(&mut self, c: char) -> Result<(), &'static str> {
        match self.state {
            State::Init => self.handle_init(c),
            State::TagName { .. } => self.handle_tag_name(c),
            State::TagContent => self.handle_tag_content(c),
            State::CDataTag { .. } => self.handle_cdata_tag(c),
            State::CDataContent { .. } => self.handle_cdata_content(c),
            State::End => self.handle_end(c),
        }
    }

    fn handle_init(&mut self, c: char) -> Result<(), &'static str> {
        match c {
            '<' => {
                self.state = State::TagName {
                    name: String::new(),
                    is_end_tag: false,
                };
                Ok(())
            }
            _ => {
                Err("Expect <")
            }
        }
    }

    fn handle_tag_name(&mut self, c: char) -> Result<(), &'static str> {
        match self.state {
            // structure binding
            State::TagName { ref mut name, ref mut is_end_tag } => {
                // println!("{} {} {}", name, is_end_tag, c);
                match (c, name.len(), &is_end_tag) {
                    // @ stands for pattern matching
                    // char for tag name, append it to current tag name
                    (c @ 'A'..='Z', 0..=8, _) => {
                        name.push(c);
                        Ok(())
                    }
                    // tag name should have length in range [1, 9]
                    // then we successfully match a begin tag
                    ('>', 1..=9, false) => {
                        // println!("{}", name.to_string());
                        self.stack.push(name.to_string());
                        // now start to matching content
                        self.state = State::TagContent;
                        Ok(())
                    }
                    // matching an end tag
                    ('>', 1..=9, true) => {
                        // println!("{}", name.to_string());
                        match self.stack.pop() {
                            Some(top) if top == *name => {
                                if self.stack.len() > 0 {
                                    // we are still at others tag content
                                    self.state = State::TagContent;
                                } else {
                                    self.state = State::End;
                                }
                                Ok(())
                            }
                            _ => {
                                Err("failed to match end tag")
                            }
                        }
                    }
                    ('/', 0, false) => {
                        // this is end tag
                        *is_end_tag = true;
                        Ok(())
                    }
                    ('!', 0, false) if !self.stack.is_empty() => {
                        // cdata content
                        self.state = State::CDataTag { name: String::new() };
                        Ok(())
                    }
                    _ => {
                        Err("")
                    }
                }
            }
            _ => {
                panic!("fatal error at handle tag name");
            }
        }
    }

    fn handle_tag_content(&mut self, c: char) -> Result<(), &'static str> {
        match c {
            '<' => {
                self.state = State::TagName {
                    name: String::new(),
                    is_end_tag: false
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_cdata_tag(&mut self, c: char) -> Result<(), &'static str> {
        match self.state {
            State::CDataTag { ref mut name } => {
                match (c, name.len()) {
                    (c, 0..=6) => {
                        // append the string
                        name.push(c);
                        if "[CDATA[" == name.as_str() {
                            // matching success
                            self.state = State::CDataContent {
                                prefix: (false, false)
                            };
                        }
                        // we could apply more fine-grained checking here
                        // i.e. use state machine to match [CDATA[
                        Ok(())
                    }
                    _ => {
                        Err("failed to match cdata tag")
                    }
                }
            }
            _ => {
                panic!("fatal error at handle cdata tag")
            }
        }
    }

    fn handle_cdata_content(&mut self, c: char) -> Result<(), &'static str> {
        match self.state {
            State::CDataContent { ref mut prefix } => {
                match (&prefix.0, &prefix.1, c) {
                    (true, true, '>') => {
                        self.state = State::TagContent;
                        Ok(())
                    }
                    (true, true, ']') => {
                        Ok(())
                    }
                    (true, false, ']') => {
                        *prefix = (true, true);
                        Ok(())
                    }
                    (false, false, ']') => {
                        *prefix = (true, false);
                        Ok(())
                    }
                    _ => {
                        *prefix = (false, false);
                        Ok(())
                    }
                }
            }
            _ => {
                panic!("fatal error at handle cdata content")
            }
        }
    }

    fn handle_end(&mut self, c: char) -> Result<(), &'static str> {
        Err("end of code")
    }
}
