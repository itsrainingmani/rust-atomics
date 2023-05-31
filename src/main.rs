#[allow(unused)]
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::{
        atomic::{Ordering::*, *},
        *,
    },
    thread::{self, Thread},
};

fn main() {
    // let numbers = vec![1, 2, 3];
    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();

        sum / len
    });

    println!("Hello from the main thread");

    // Waits until the thread has finished executing and returns a std::thread::Result
    let avg = t.join().unwrap();
    println!("avg = {avg}");
    // t2.join().unwrap();
}

fn _f() {
    println!("Hello from other thread");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
