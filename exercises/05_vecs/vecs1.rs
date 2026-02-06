fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;
    let v:Vec<i32> = vec![10, 20, 30, 40];
    (a, v)
}

fn main() {
    // You can optionally experiment here.
    let a = [1, 2, 3];
    // Method 1: .to_vec() (requires elements to be Clone)
    let v1 = a.to_vec();
    
    // Method 2: Vec::from()
    let v2 = Vec::from(a);
    
    // Method 3: .into()
    let v3: Vec<i32> = a.into();

    // Method 4: Slice (View) - ZERO COPY!
    // This just creates a "window" viewing the stack array `a`
    let s: &[i32] = &a;

    println!("Original array (STACK): {:?}", a);
    
    println!("\nMemory comparison:");
    // `a` lives on the STACK
    println!("a (Stack Array)       : len={}, ptr={:p} (High memory address)", a.len(), a.as_ptr());
    
    // `s` points to `a` on the STACK (same pointer!)
    println!("s (Slice View)        : len={}, ptr={:p} (Same as `a`!)", s.len(), s.as_ptr());

    // `v1`, `v2`, `v3` allocate new memory on the HEAP
    println!("v1 (.to_vec() - HEAP) : len={}, cap={}, ptr={:p} (Low memory address)", v1.len(), v1.capacity(), v1.as_ptr());
    println!("v2 (Vec::from - HEAP) : len={}, cap={}, ptr={:p} (Distinct allocation)", v2.len(), v2.capacity(), v2.as_ptr());
    println!("v3 (.into()   - HEAP) : len={}, cap={}, ptr={:p} (Distinct allocation)", v3.len(), v3.capacity(), v3.as_ptr());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
