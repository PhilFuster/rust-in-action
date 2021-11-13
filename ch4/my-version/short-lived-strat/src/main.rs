#![allow(unused_variables, dead_code)]

#[derive(Debug)]
struct CubeSat {
  id: u64,
}

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

#[derive(Debug)]
struct Mailbox {
  messages: Vec<Message>,
}

impl Mailbox {
  fn post(
    &mut self,
    msg: Message
  ) {
    self.messages.push(msg);
  }

  fn deliver(
    &mut self,
    recipient: &CubeSat
  ) -> Option<Message> {
    for i in 0..self.messages.len() {
      if self.messages[i].to == recipient.id {
        let msg = self.messages.remove(i);
        return Some(msg);
      }
    }
    None
  }
}

#[derive(Debug)]
struct Message {
  to: u64,
  content: String
}
struct GroundStation;

impl GroundStation {
  fn connect(&self, sat_id: u64) -> CubeSat {
    CubeSat {
      id: sat_id,
    }
  }
    fn send(
      &self,
      mailbox: &mut Mailbox,
      msg: Message,
    ) {
      mailbox.post(msg);
    }
}

impl CubeSat {
  fn recv(
    &mut self,
    mailbox: Mailbox,
  ) -> Option<Message> {
    mailbox.deliver(&self)
  }
}

fn fetch_sat_ids() -> Vec<u64> {
  vec![1, 2, 3]
}


fn main() {
  let mut mail: Mailbox = Mailbox{ messages: vec![] };

  let base = GroundStation{}; 

  let sat_ids = fetch_sat_ids();

  for sat_id in sat_ids {
    let sat = base.connect(sat_id);
    let msg = Message { to: sat_id, content: String::from("Hello") };
    base.send(&mut mail, msg);
  }

  println!("sat at 0: {}", sat_ids[0] )
}
