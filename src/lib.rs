use futures::Future;
use tokio::prelude::*;
use tokio_channel::mpsc;
use std::fmt::Debug;

pub struct ChatBox<M> {
    store: Vec<M>,
    ch_r:mpsc::Receiver<Request<M>>, 
}

pub enum Request<M> {
    Put(M),

}

impl<M: Debug> ChatBox<M> {
    pub fn new()->(Self, mpsc::Sender<Request<M>>) {
        let (ch_s, ch_r) = mpsc::channel(10);
        (
            ChatBox { 
                store: Vec::new() ,
                ch_r,
            },
            ch_s,
        )
    }
}

impl<M: Debug>  Future for ChatBox<M> {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        loop {
            let rq = match {self.ch_r.poll()?} {
                Async::NotReady=> return Ok(Async::NotReady),
                Async::Ready(Some(v))=>v,
                Async::Ready(None)=> return Ok(Async::Ready(())),
            };
            match rq {
                Request::Put(m) => {
                    println!("got message {:?}", m);
                },
            }
        }

        //OK(Async::Ready("hello".to_string()))

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::lazy;
    #[test]
    fn it_works() {
        let f = lazy(|| {
            let (f, ch_s) = ChatBox::new();
            let p = f.map(|s|println!("{:?}", s));
            tokio::spawn(p);
            tokio::spawn(
                ch_s.send(Request::Put(3))
                .map(|_| ())
                .map_err(|e| println!("Send Error: {:?}", e))
            );
            Ok(())
        });
        //let f = ChatBox{};
        //println!("Beginning");

        //let ch_s.send(3);
        //println!("Ending");
        //panic!("");
        //
        //

        tokio::run(f);




    }
}
