use std::{panic, thread};
use tempera::custom::CUSTOM;
use tempera::{add_style, colorize, colorize_template, delete_styles, resolve_styles};

#[test]
fn add_style_returns_an_error_when_the_lock_is_poisoned() {
  let old_hook = panic::take_hook();
  panic::set_hook(Box::new(|_| {}));

  let handle = thread::spawn(move || {
    let _writer = CUSTOM.write().unwrap();
    panic!("Crash!");
  });

  let _ = handle.join();
  panic::set_hook(old_hook);

  assert!(matches!(
    add_style("my_style", &["red"]),
    Err(tempera::Error::LockPoisoned)
  ));
}

#[test]
fn delete_styles_returns_an_error_when_the_lock_is_poisoned() {
  let old_hook = panic::take_hook();
  panic::set_hook(Box::new(|_| {}));

  let handle = thread::spawn(move || {
    let _writer = CUSTOM.write().unwrap();
    panic!("Crash!");
  });

  let _ = handle.join();
  panic::set_hook(old_hook);

  assert!(matches!(
    delete_styles(&["my_style"]),
    Err(tempera::Error::LockPoisoned)
  ));
}

#[test]
fn resolve_styles_returns_an_error_when_the_lock_is_poisoned() {
  let old_hook = panic::take_hook();
  panic::set_hook(Box::new(|_| {}));

  let handle = thread::spawn(move || {
    let _writer = CUSTOM.write().unwrap();
    panic!("Crash!");
  });

  let _ = handle.join();
  panic::set_hook(old_hook);

  assert!(matches!(
    resolve_styles(&["my_style"]),
    Err(tempera::Error::LockPoisoned)
  ));
}

#[test]
fn colorize_becomes_a_noop_when_the_lock_is_poisoned() {
  let old_hook = panic::take_hook();
  panic::set_hook(Box::new(|_| {}));

  let handle = thread::spawn(move || {
    let _writer = CUSTOM.write().unwrap();
    panic!("Crash!");
  });

  let _ = handle.join();
  panic::set_hook(old_hook);

  assert_eq!(colorize("ABC", &["bg_Black", "red"]), "ABC");
}

#[test]
fn colorize_template_becomes_a_noop_when_the_lock_is_poisoned() {
  let old_hook = panic::take_hook();
  panic::set_hook(Box::new(|_| {}));

  let handle = thread::spawn(move || {
    let _writer = CUSTOM.write().unwrap();
    panic!("Crash!");
  });

  let _ = handle.join();
  panic::set_hook(old_hook);

  assert_eq!(colorize_template("{red}ABC{-}"), "ABC");
}
