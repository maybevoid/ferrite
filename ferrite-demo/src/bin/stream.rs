use std::time::Duration;

use ferrite_session::prelude::*;
use tokio::time::sleep;

type IntStream = Rec<SendValue<u64, Z>>;

fn producer(count: u64) -> Session<IntStream>
{
  fix_session(step(async move {
    println!("[producer] Producing value: {}", count);
    sleep(Duration::from_secs(1)).await;
    send_value(count, producer(count + 1))
  }))
}

fn consumer<A: Protocol>() -> Session<ReceiveChannel<IntStream, A>>
{
  receive_channel(move |stream| {
    unfix_session(
      stream,
      receive_value_from(stream, move |count| {
        println!("[consumer] Received value: {}", count);
        include_session(consumer(), |next| {
          send_channel_to(next, stream, forward(next))
        })
      }),
    )
  })
}

pub fn stream_session() -> Session<End>
{
  let p1 = producer(0);

  let p2 = consumer();

  apply_channel(p2, p1)
}

#[tokio::main]

pub async fn main()
{
  run_session(stream_session()).await;
}
