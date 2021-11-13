#![allow(unused_variables, dead_code)]

#[derive(Debug)]
struct CubeSat {
  id: u64,
  mailbox: Mailbox,
}

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

#[derive(Debug)]
struct Mailbox {
  messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(
      &self,
      to: &mut CubeSat,
      msg: Message,
    ) {
      to.mailbox.messages.push(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
      CubeSat { id: sat_id, mailbox: Mailbox { messages: vec![] } }
    }
}

impl CubeSat {
  fn recv(
    &mut self,
  ) -> Option<Message> {
    self.mailbox.messages.pop()
  }
}

fn fetch_sat_ids() -> Vec<u64> {
  vec![1, 2, 3]
}


fn main() {
  let base = GroundStation{}; 
  let sat_ids = fetch_sat_ids();
  for ii in 0..sat_ids.len() {
    let msg = Message::from("hello there");
    let mut curr_sat = base.connect(sat_ids[ii]);
    base.send(&mut curr_sat, msg);
  }
  for kk in 0..sat_ids.len() {
    let curr_sat = base.connect(sat_ids[kk]);
    println!("{:?}", curr_sat);
  }
  for jj in 0..sat_ids.len() {
    let mut curr_sat = base.connect(sat_ids[jj]);
    curr_sat.recv();
  }

}
