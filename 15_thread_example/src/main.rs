use std::thread;
fn main() {
   let handle = thread::spawn(move || {
      // do stuff in child thread
   });
   // do simultaneously in the main thread
   
   // wait until thread has existed
   handle.join().unwrap();
}
