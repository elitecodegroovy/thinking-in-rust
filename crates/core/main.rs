use std::error;
use std::io::{self, Write};
use std::process;
use std::sync::Mutex;
use std::time::Instant;

use ignore::WalkState;

use args::Args;
use subject::Subject;

#[macro_use]
mod messages;

mod app;
mod args;
mod config;
mod logger;
mod path_printer;
mod search;
mod subject;
mod thinking;
mod home; // Import home module
mod utils;
use crate::List::{Nil, Cons};

#[macro_use]
extern crate log;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn do_linked_list() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list 长度= {}", list.len());
    println!("{}", list.stringify());
}
// Since Rust no longer uses jemalloc by default, ripgrep will, by default,
// use the system allocator. On Linux, this would normally be glibc's
// allocator, which is pretty good. In particular, ripgrep does not have a
// particularly allocation heavy workload, so there really isn't much
// difference (for ripgrep's purposes) between glibc's allocator and jemalloc.
//
// However, when ripgrep is built with musl, this means ripgrep will use musl's
// allocator, which appears to be substantially worse. (musl's goal is not to
// have the fastest version of everything. Its goal is to be small and amenable
// to static compilation.) Even though ripgrep isn't particularly allocation
// heavy, musl's allocator appears to slow down ripgrep quite a bit. Therefore,
// when building with musl, we use jemalloc.
//
// We don't unconditionally use jemalloc because it can be nice to use the
// system's default allocator by default. Moreover, jemalloc seems to increase
// compilation times by a bit.
//
// Moreover, we only do this on 64-bit systems since jemalloc doesn't support
// i686.
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    initLog();
    log::info!(">>> start up...");
    if let Err(err) = Args::parse().and_then(try_main) {
        // TODO ... remove the useless code
        thinking_in_rust();
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn initLog() {
    // RUST_LOG environment variable that corresponds with the log messages you want to show
    // RUST_LOG=info cargo run
    use log::LevelFilter;
    use std::io::Write;
    use chrono::Local;
    use env_logger::Builder;

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}]: {}",
                //Format like you want to: <-----------------
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}
fn thinking_in_rust() {
    home::do_layout();
    home::decorator::do_decorate();
    do_home();
    do_linked_list();
    thinking::main_thinking();
}

fn try_main(args: Args) -> Result<()> {
    use args::Command::*;

    let matched = match args.command()? {
        Search => search(&args),
        SearchParallel => search_parallel(&args),
        SearchNever => Ok(false),
        Files => files(&args),
        FilesParallel => files_parallel(&args),
        Types => types(&args),
        PCRE2Version => pcre2_version(&args),
    }?;
    if matched && (args.quiet() || !messages::errored()) {
        process::exit(0)
    } else if messages::errored() {
        process::exit(2)
    } else {
        process::exit(1)
    }
}

fn do_home() {
    utils::print_info();
    utils::filter_str();
    utils::print_msg()
}

/// The top-level entry point for single-threaded search. This recursively
/// steps through the file list (current directory by default) and searches
/// each file sequentially.
fn search(args: &Args) -> Result<bool> {
    let started_at = Instant::now();
    let quit_after_match = args.quit_after_match()?;
    let subject_builder = args.subject_builder();
    let mut stats = args.stats()?;
    let mut searcher = args.search_worker(args.stdout())?;
    let mut matched = false;

    for result in args.walker()? {
        let subject = match subject_builder.build_from_result(result) {
            Some(subject) => subject,
            None => continue,
        };
        let search_result = match searcher.search(&subject) {
            Ok(search_result) => search_result,
            Err(err) => {
                // A broken pipe means graceful termination.
                if err.kind() == io::ErrorKind::BrokenPipe {
                    break;
                }
                err_message!("{}: {}", subject.path().display(), err);
                continue;
            }
        };
        matched = matched || search_result.has_match();
        if let Some(ref mut stats) = stats {
            *stats += search_result.stats().unwrap();
        }
        if matched && quit_after_match {
            break;
        }
    }
    if let Some(ref stats) = stats {
        let elapsed = Instant::now().duration_since(started_at);
        // We don't care if we couldn't print this successfully.
        let _ = searcher.print_stats(elapsed, stats);
    }
    Ok(matched)
}

/// The top-level entry point for multi-threaded search. The parallelism is
/// itself achieved by the recursive directory traversal. All we need to do is
/// feed it a worker for performing a search on each file.
fn search_parallel(args: &Args) -> Result<bool> {
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering::SeqCst;

    let quit_after_match = args.quit_after_match()?;
    let started_at = Instant::now();
    let subject_builder = args.subject_builder();
    let bufwtr = args.buffer_writer()?;
    let stats = args.stats()?.map(Mutex::new);
    let matched = AtomicBool::new(false);
    let mut searcher_err = None;
    args.walker_parallel()?.run(|| {
        let bufwtr = &bufwtr;
        let stats = &stats;
        let matched = &matched;
        let subject_builder = &subject_builder;
        let mut searcher = match args.search_worker(bufwtr.buffer()) {
            Ok(searcher) => searcher,
            Err(err) => {
                searcher_err = Some(err);
                return Box::new(move |_| WalkState::Quit);
            }
        };

        Box::new(move |result| {
            let subject = match subject_builder.build_from_result(result) {
                Some(subject) => subject,
                None => return WalkState::Continue,
            };
            searcher.printer().get_mut().clear();
            let search_result = match searcher.search(&subject) {
                Ok(search_result) => search_result,
                Err(err) => {
                    err_message!("{}: {}", subject.path().display(), err);
                    return WalkState::Continue;
                }
            };
            if search_result.has_match() {
                matched.store(true, SeqCst);
            }
            if let Some(ref locked_stats) = *stats {
                let mut stats = locked_stats.lock().unwrap();
                *stats += search_result.stats().unwrap();
            }
            if let Err(err) = bufwtr.print(searcher.printer().get_mut()) {
                // A broken pipe means graceful termination.
                if err.kind() == io::ErrorKind::BrokenPipe {
                    return WalkState::Quit;
                }
                // Otherwise, we continue on our merry way.
                err_message!("{}: {}", subject.path().display(), err);
            }
            if matched.load(SeqCst) && quit_after_match {
                WalkState::Quit
            } else {
                WalkState::Continue
            }
        })
    });
    if let Some(err) = searcher_err.take() {
        return Err(err);
    }
    if let Some(ref locked_stats) = stats {
        let elapsed = Instant::now().duration_since(started_at);
        let stats = locked_stats.lock().unwrap();
        let mut searcher = args.search_worker(args.stdout())?;
        // We don't care if we couldn't print this successfully.
        let _ = searcher.print_stats(elapsed, &stats);
    }
    Ok(matched.load(SeqCst))
}

/// The top-level entry point for listing files without searching them. This
/// recursively steps through the file list (current directory by default) and
/// prints each path sequentially using a single thread.
fn files(args: &Args) -> Result<bool> {
    let quit_after_match = args.quit_after_match()?;
    let subject_builder = args.subject_builder();
    let mut matched = false;
    let mut path_printer = args.path_printer(args.stdout())?;
    for result in args.walker()? {
        let subject = match subject_builder.build_from_result(result) {
            Some(subject) => subject,
            None => continue,
        };
        matched = true;
        if quit_after_match {
            break;
        }
        if let Err(err) = path_printer.write_path(subject.path()) {
            // A broken pipe means graceful termination.
            if err.kind() == io::ErrorKind::BrokenPipe {
                break;
            }
            // Otherwise, we have some other error that's preventing us from
            // writing to stdout, so we should bubble it up.
            return Err(err.into());
        }
    }
    Ok(matched)
}

/// The top-level entry point for listing files without searching them. This
/// recursively steps through the file list (current directory by default) and
/// prints each path sequentially using multiple threads.
fn files_parallel(args: &Args) -> Result<bool> {
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering::SeqCst;
    use std::sync::mpsc;
    use std::thread;

    let quit_after_match = args.quit_after_match()?;
    let subject_builder = args.subject_builder();
    let mut path_printer = args.path_printer(args.stdout())?;
    let matched = AtomicBool::new(false);
    let (tx, rx) = mpsc::channel::<Subject>();

    let print_thread = thread::spawn(move || -> io::Result<()> {
        for subject in rx.iter() {
            path_printer.write_path(subject.path())?;
        }
        Ok(())
    });
    args.walker_parallel()?.run(|| {
        let subject_builder = &subject_builder;
        let matched = &matched;
        let tx = tx.clone();

        Box::new(move |result| {
            let subject = match subject_builder.build_from_result(result) {
                Some(subject) => subject,
                None => return WalkState::Continue,
            };
            matched.store(true, SeqCst);
            if quit_after_match {
                WalkState::Quit
            } else {
                match tx.send(subject) {
                    Ok(_) => WalkState::Continue,
                    Err(_) => WalkState::Quit,
                }
            }
        })
    });
    drop(tx);
    if let Err(err) = print_thread.join().unwrap() {
        // A broken pipe means graceful termination, so fall through.
        // Otherwise, something bad happened while writing to stdout, so bubble
        // it up.
        if err.kind() != io::ErrorKind::BrokenPipe {
            return Err(err.into());
        }
    }
    Ok(matched.load(SeqCst))
}

/// The top-level entry point for --type-list.
fn types(args: &Args) -> Result<bool> {
    let mut count = 0;
    let mut stdout = args.stdout();
    for def in args.type_defs()? {
        count += 1;
        stdout.write_all(def.name().as_bytes())?;
        stdout.write_all(b": ")?;

        let mut first = true;
        for glob in def.globs() {
            if !first {
                stdout.write_all(b", ")?;
            }
            stdout.write_all(glob.as_bytes())?;
            first = false;
        }
        stdout.write_all(b"\n")?;
    }
    Ok(count > 0)
}

/// The top-level entry point for --pcre2-version.
fn pcre2_version(args: &Args) -> Result<bool> {
    #[cfg(feature = "pcre2")]
    fn imp(args: &Args) -> Result<bool> {
        use grep::pcre2;

        let mut stdout = args.stdout();

        let (major, minor) = pcre2::version();
        writeln!(stdout, "PCRE2 {}.{} is available", major, minor)?;

        if cfg!(target_pointer_width = "64") && pcre2::is_jit_available() {
            writeln!(stdout, "JIT is available")?;
        }
        Ok(true)
    }

    #[cfg(not(feature = "pcre2"))]
    fn imp(args: &Args) -> Result<bool> {
        let mut stdout = args.stdout();
        writeln!(stdout, "PCRE2 is not available in this build of ripgrep.")?;
        Ok(false)
    }

    imp(args)
}
