use self::List::{Cons, Nil};
use std::iter::FromIterator;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;

pub enum List<T> {
  Cons(T, Box<List<T>>),
  Nil,
}

impl<T> List<T> {
  /// Create a new list from the given collection.
  ///
  /// # Examples
  ///
  /// ```
  /// let list = smart_pointers::List::new(vec![1, 2, 3]);
  ///
  /// assert_eq!(3, list.iter().count());
  /// ```
  pub fn new<I>(items: I) -> List<T>
  where
    I: IntoIterator<Item = T>,
  {
    items.into_iter().collect()
  }

  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Iter { list: self }
  }

  /// Retrieve the first item of the list, or `None` if it is empty.
  ///
  /// # Examples
  ///
  /// ```
  /// let list = smart_pointers::List::new(vec![1, 2]);
  ///
  /// assert_eq!(Some(&1), list.head());
  /// assert_eq!(None, smart_pointers::List::<u32>::Nil.head());
  /// ```
  pub fn head(&self) -> Option<&T> {
    match self {
      Cons(item, _) => Some(item),
      Nil => None,
    }
  }

  /// Add a new item to the end of the list
  ///
  /// # Examples
  ///
  /// ```
  /// let mut list = smart_pointers::List::Nil;
  /// list.push(1);
  ///
  /// assert_eq!(Some(&1), list.head());
  /// ```
  pub fn push(&mut self, item: T) {
    match self {
      Cons(_, tail) => {
        tail.push(item);
      }
      Nil => {
        mem::replace(self, Cons(item, Box::new(Nil)));
      }
    }
  }

  /// Remove the first item of the list and return it.
  ///
  /// # Examples
  ///
  /// ```
  /// use smart_pointers::List;
  /// let mut list = smart_pointers::List::new(vec![1, 2]);
  ///
  /// assert_eq!(Some(1), list.shift());
  /// assert_eq!(Some(2), list.shift());
  /// assert_eq!(None, list.shift());
  /// ```
  pub fn shift(&mut self) -> Option<T> {
    match mem::replace(self, Nil) {
      Cons(result, tail) => {
        mem::replace(self, *tail);
        Some(result)
      }
      Nil => None,
    }
  }
}

impl<T> FromIterator<T> for List<T> {
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = T>,
  {
    let mut list = Nil;
    let mut list_end = &mut list;
    for item in iter {
      mem::replace(list_end, Cons(item, Box::new(Nil)));
      if let Cons(_, new_end) = list_end {
        list_end = new_end.deref_mut()
      }
    }
    list
  }
}

impl<T> IntoIterator for List<T> {
  type Item = T;
  type IntoIter = IntoIter<T>;

  /// Creates a consuming iterator. Allows lists to be used in for loops.
  fn into_iter(self) -> Self::IntoIter {
    IntoIter { list: self }
  }
}

pub struct Iter<'a, T> {
  list: &'a List<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.list {
      Cons(result, tail) => {
        self.list = tail.deref();
        Some(result)
      }
      Nil => None,
    }
  }
}

pub struct IntoIter<T> {
  list: List<T>,
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.list.shift()
  }
}
