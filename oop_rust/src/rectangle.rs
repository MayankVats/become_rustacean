
// Here pub keyword implements the idea of abstraction
pub struct Rectangle {
  pub width: u32,
  pub height: u32
}

// Polymorphism with traits
pub trait ShapeMethods {
  fn area(&self) -> u32;
  fn calc_perimeter(&self) -> u32;
}

// Encapsulation with methods
impl Rectangle {
  pub fn perimeter(&self) -> u32 {
    self.calc_perimeter()
  }

  // if we want to change the value of the struct fields
  // we can create mutable reference to the self (&mut self)
  pub fn set_width(&mut self, width: u32) {
      self.width = width;
  }

  pub fn set_height(&mut self, height: u32) {
      self.height = height;
  }
}

impl ShapeMethods for Rectangle {
  // here &self is immutable reference to the instance
  // we use this in case the function is read only
  fn area(&self) -> u32 {
    self.width * self.height
  }

  // This function is private and can only be used in this file only
  // It cannot be exported
  fn calc_perimeter(&self) -> u32 {
    2 * (self.width + self.height)
  }
}