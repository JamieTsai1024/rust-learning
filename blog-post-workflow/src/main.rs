// Version 1: No compile time errors 
// use blog::Post;

// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }

//     pub fn add_text(&mut self, text: &str) { // mutable reference to self since we're changing the Post instance 
//         self.content.push_str(text);
//     }
    
//     pub fn content(&self) -> &str {
//         ""
//     }
    
//     pub fn request_review(&mut self) { 
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review()) // consumes current state and returns a new state 
//         }
//     }
    
//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve())
//         }
//     }

//     pub fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)
//     }
// }

// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     // default implementation for content 
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
// }

// struct Draft {}

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }

// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {}) // Return a new, boxed instance of the Published struct
//     }
// }

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }

// fn main() {
//     let mut post = Post::new();

//     post.add_text("I ate a salad for lunch today");
//     assert_eq!("", post.content());

//     post.request_review();
//     assert_eq!("", post.content());

//     post.approve();
//     assert_eq!("I ate a salad for lunch today", post.content());
// }

// Version 2: Compile time errors for invalid states 
// Note: won't compile since we haven't implemented Blog crate 
use blog::Post;

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(), // new() now returns an instance of DraftPost
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}