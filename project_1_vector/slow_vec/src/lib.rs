use std::fmt::{Display, Formatter};
use fixed::FixedSizeArray;

// A SlowVec contains a fixed number of elements.
// The elements are of type "T"
// This is Rust's way of saying that SlowVec can accept any type for the elements.
// E.g., SlowVec<i32> represents a SlowVec with integer elements,
//       SlowVec<bool> represents a SlowVec with bool elements,
//       etc.
// look at main.rs for an example.
pub struct SlowVec<T> {
    fixed: FixedSizeArray<T>,
}

// Functions inside SlowVec.
impl<T> SlowVec<T> {
    pub fn new() -> Self {
        return SlowVec {
            fixed: FixedSizeArray::allocate(0)
        };
    }
    
    // returns the length of the SlowVec.
    pub fn len(&self) -> usize {
        return self.fixed.len();
    }

    // Transforms an instance of SlowVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.fixed.len());
        for i in 0..self.fixed.len() {
            v.push(self.fixed.move_out(i));
        }
        v
    }

    // Transforms a vector to a SlowVec.
    pub fn from_vec(vec: Vec<T>) -> SlowVec<T> {
        let mut tmp = FixedSizeArray::allocate(vec.len());
        let mut index = 0;
        for element in vec {
            tmp.put(element, index);
            index = index + 1;
        }
        return SlowVec { fixed: tmp };
    }

    // Clear the content of this vector.
    pub fn clear(&mut self) {
        self.fixed = FixedSizeArray::allocate(0);
    }

    // Get a reference to the element at position i.
    // Think of a reference as a read-only "copy" of the element.
    // We will talk about what references are in class.
    // Note: the element remains stored in the SlowVec after get(). It is not removed.
    pub fn get(&self, i: usize) -> &T {
        self.fixed.get(i)
    }

    // Student 1: Provide your solution here.
    pub fn push(&mut self, t: T) { //grows the vector at the end, t element we wanna grow
        let mut tmp = FixedSizeArray::allocate(self.len() + 1); //adds one more space for new element to kinda 'grow' the vector bc fixedsizearray cant be changed
       
        for i in 0..self.len() { //0 to current len
            tmp.put(self.fixed.move_out(i),i); //takes element and removes, then puts new element at same position i (transfer from old to new)
        }
        tmp.put(t, self.len()); //puts t at the last index, so like our "push"

        self.fixed = tmp; //get rid of the old fixed field and replace with tmp
    }

    // Student 2: Provide your solution here
    pub fn remove(&mut self, i: usize) {
        todo!("Student 2 should implement this");
    }
}


// This allows us to print the SlowVec using println!().
impl<T: Display> Display for SlowVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlowVec({})", self.fixed)
    }
}
