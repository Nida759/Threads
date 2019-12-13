// use std::thread;
// use std::time::Duration;

// fn main() {
//     thread::spawn(|| {
//         for o in 1..10 {
//             println!("hi number {} from the spawned thread!", o);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for e in 1..5 {
//         println!("hi number {} from the main thread!", e);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for o in 1..10 {
//             println!("hi number {} from the spawned thread!", o);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for e in 1..5 {
//         println!("hi number {} from the main thread!", e);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap();
// }

// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for o in 1..10 {
//             println!("hi number {} from the spawned thread!", o);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     handle.join().unwrap();

//     for e in 1..5 {
//         println!("hi number {} from the main thread!", e);
//         thread::sleep(Duration::from_millis(1));
//     }
// }
