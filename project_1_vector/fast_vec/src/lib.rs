use std::{fmt::{Display, Formatter}, ptr::{self, null_mut}};

use malloc::MALLOC;

pub struct FastVec<T> {
    ptr_to_data: *mut T,
    len: usize,
    capacity: usize,
}
impl<T> FastVec<T> {
    // Creating a new FastVec that is either empty or has capacity for some future elements.
    pub fn new() -> FastVec<T> {
        return FastVec::with_capacity(1);
    }
    pub fn with_capacity(capacity: usize) -> FastVec<T> {
        return FastVec {
            ptr_to_data: MALLOC.malloc(size_of::<T>() * capacity) as *mut T,
            len: 0,
            capacity: capacity,
        };
    }

    // Retrieve the FastVec's length and capacity
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    // Transforms an instance of SlowVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len);
        for i in 0..self.len {
            unsafe {
                let ptr = self.ptr_to_data.add(i);
                let element = ptr::read(ptr);
                v.push(element);
            }
        }
        MALLOC.free(self.ptr_to_data as *mut u8);
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
        return v;
    }

    // Transforms a vector to a SlowVec.
    pub fn from_vec(vec: Vec<T>) -> FastVec<T> {
        let mut fast_vec: FastVec<T> = FastVec::with_capacity(vec.len());
        for element in vec {
            unsafe {
                let ptr = fast_vec.ptr_to_data.add(fast_vec.len);
                ptr::write(ptr, element);
            }
            fast_vec.len = fast_vec.len + 1;
        }
        return fast_vec;
    }

    // Student 1 and Student 2 should implement this together
    // Use the project handout as a guide for this part!
    pub fn get(&self, i: usize) -> &T {
        if i >= self.len {
            panic!("FastVec: get out of bounds");
        }
        unsafe {
            &*self.ptr_to_data.add(i)
        }
    }

    // Student 2 should implement this.
    pub fn push(&mut self, t: T) {
        if self.len == self.capacity {  //check if vector has reached capacity
            let new_capacity = if self.capacity == 0 {1} else {self.capacity * 2}; //if empty, start vector as length of 1 or double to current capacity
            unsafe {  //unsafe block implemented since we are dealing with raw pointers
                let new_ptr = MALLOC.malloc(std::mem::size_of::<T>() * new_capacity) as *mut T;  //allocate new memory in the heap cast to *mut T
                
                for i in 0..self.len {  //copy existing elements from old memory to new memory
                    let element = std::ptr::read(self.ptr_to_data.add(i));  //read the element from old pointer starting at index i
                    std::ptr::write(new_ptr.add(i), element);  //write the element into new pointer at same index
                }
                if !self.ptr_to_data.is_null() {  //if pointer is not empty, free the old memory
                    MALLOC.free(self.ptr_to_data as *mut u8); 
                }
                
                std::ptr::write(new_ptr.add(self.len), t);  //write the new element into new memory at index 'len'

                self.ptr_to_data = new_ptr;  //update struct to point to new memory
                self.capacity = new_capacity;  //update capacity to reflect the updated size
            }
        } else {
            unsafe {  //if there is still capacity, write new element at index 'len', in the same collection of memory 
                std::ptr::write(self.ptr_to_data.add(self.len), t);
            }
        }
        self.len += 1;  //increase length since a new element was just added
    }

    // Student 1 should implement this.
    pub fn remove(&mut self, i: usize) {
        if i >= self.len{
            panic!("FastVec: remove out of bounds");
        }
        unsafe {
            let to_be_removed = ptr::read(self.ptr_to_data.add(i));
            for j in i + 1..self.len {
                let value = ptr::read(self.ptr_to_data.add(j));
                ptr::write(self.ptr_to_data.add(j-1), value);
            }
            self.len = self.len - 1;
        }
    }

    // This appears correct but with further testing, you will notice it has a bug!
    // Student 1 and 2 should attempt to find and fix this bug.
    // Hint: check out case 2 in memory.rs, which you can run using
    //       cargo run --bin memory
    pub fn clear(&mut self) {
        MALLOC.free(self.ptr_to_data as *mut u8);
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
    }
}

// Destructor should clear the fast_vec to avoid leaking memory.
impl<T> Drop for FastVec<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

// This allows printing FastVecs with println!.
impl<T: Display> Display for FastVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FastVec[")?;
        if self.len > 0 {
            for i in 0..self.len()-1 {
                write!(f, "{}, ", self.get(i))?;
            }
            write!(f, "{}", self.get(self.len - 1))?;
        }
        return write!(f, "]");
    }
}
