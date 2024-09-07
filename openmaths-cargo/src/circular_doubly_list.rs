use std::rc::Rc;
use std::cell::RefCell;

// type Link<T> = Rc<<Node<T>>;
// This above it still possible, which does not require RefCell because we are not mutating the data
// But we are using RefCell because we are mutating the data
// RefCell is a runtime check, so it is slower than Rc
// Rc is a compile time check, so it is faster than RefCell
// RefCell is used when we are not sure about the ownership of the data

type Link<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
  pub data: T,
  next: Link<T>,
  prev: Link<T>,
}

pub struct CircularDoubly<T> {
  head: Option<Rc<RefCell<Node<T>>>>,
  tail: Option<Rc<RefCell<Node<T>>>>,
  length: u32,
}

impl<T> CircularDoubly<T> {
  pub fn new() -> Self {
    CircularDoubly {
      head: None,
      tail: None,
      length: 0,
    }
  }

  pub fn push(&mut self, data: T) {
    let new_node = Rc::new(RefCell::new(Node {
      data,
      next: Rc::clone(&self.head.as_ref().unwrap()),
      prev: Rc::clone(&self.tail.as_ref().unwrap()),
    }));

    match self.head.take() {
      Some(old_head) => {
        old_head.borrow_mut().prev = Rc::clone(&new_node);
        self.tail = Some(new_node.clone());
      }
      None => {
        self.tail = Some(new_node.clone());
      }
    }

    self.head = Some(new_node);
    self.length += 1;
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|old_head| {
      if self.length == 1 {
        self.tail.take();
      } else {
        self.head = Some(Rc::clone(&old_head.borrow().next));
        self.head.as_ref().unwrap().borrow_mut().prev = Rc::clone(&self.tail.as_ref().unwrap());
        self.tail.as_ref().unwrap().borrow_mut().next = Rc::clone(&self.head.as_ref().unwrap());
      }

      self.length -= 1;
      Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
    })
  }

  pub fn next(&self) -> Link<T> {
    Rc::clone(&self.head.as_ref().unwrap().borrow().next)
  }

  pub fn length(&self) -> u32 {
    self.length
  }
}
