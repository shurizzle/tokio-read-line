extern crate tokio;

use std::io::Write;

use tokio_read_line::*;

use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

fn print_prompt<S: AsRef<str>>(prompt: S) {
    print!("{}", prompt.as_ref());
    std::io::stdout().flush().unwrap();
}

fn stdin(token_sender: mpsc::Sender<String>) -> (oneshot::Sender<()>, JoinHandle<()>) {
    let (tx, rx) = oneshot::channel();
    let mut lines = ReadLines::new().unwrap();

    (
        tx,
        tokio::task::spawn(async move {
            tokio::pin!(rx);

            loop {
                tokio::select! {
                    _ = &mut rx => {
                        break;
                    }
                    Ok(line) = lines.next() => {
                        print_prompt("> ");
                        if !line.is_empty() {
                            token_sender.send(line).await.ok();
                        }
                    }
                }
            }
        }),
    )
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(2);
    print_prompt("> ");
    let (stdin_tx, stdin_handle) = stdin(tx);
    println!("Token: {:?}", rx.recv().await);
    stdin_tx.send(()).ok();
    stdin_handle.await.ok();
}
