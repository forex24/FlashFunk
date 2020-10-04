#![allow(dead_code)]

use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crossbeam_queue::spsc::{new, Consumer, Producer};

pub(crate) fn channel<M>(cap: usize) -> (Sender<M>, Receiver<M>) {
    let (tx, rx) = new(cap);
    (Sender(tx), Receiver(rx))
}

pub(crate) struct Sender<M>(Producer<M>);

impl<M> Sender<M> {
    // 发送失败会panic
    pub(crate) fn send(&self, m: M) {
        self.0.push(m).unwrap();
    }

    // 发送失败返回消息
    pub(crate) fn try_send(&self, m: M) -> Result<(), ChannelError<M>> {
        self.0.push(m).map_err(|e| ChannelError::TrySendError(e.0))
    }
}

pub(crate) struct Receiver<M>(Consumer<M>);

impl<M> Receiver<M> {
    pub(crate) fn recv(&self) -> Result<M, ChannelError<M>> {
        self.0.pop().map_err(|_| ChannelError::RecvError)
    }
}

pub(crate) struct GroupSender<M> {
    sender: Vec<Sender<M>>,
    group: Vec<Vec<usize>>,
}

impl<M> GroupSender<M> {
    pub(crate) fn new(sender: Vec<Sender<M>>, group: Vec<Vec<usize>>) -> Self {
        Self { sender, group }
    }

    // 发送至所有sender
    pub(crate) fn send_all<MM>(&self, mm: MM)
    where
        MM: Into<M> + Clone,
    {
        self.sender.iter().for_each(|s| s.send(mm.clone().into()))
    }

    // 发送至指定index的sender. 失败会panic
    pub(crate) fn send_to(&self, m: M, sender_index: usize) {
        self.sender.get(sender_index).unwrap().send(m);
    }

    // 发送至指定index的sender. 失败会返回消息
    pub(crate) fn try_send_to(&self, m: M, sender_index: usize) -> Result<(), ChannelError<M>> {
        match self.sender.get(sender_index) {
            Some(s) => s.try_send(m),
            None => Err(ChannelError::SenderOverFlow(m)),
        }
    }

    // 发送至指定group. group查找失败失败会返回消息.(group内的sender发送失败会panic)
    pub(crate) fn try_send_group<MM>(
        &self,
        mm: MM,
        group_index: usize,
    ) -> Result<(), ChannelError<MM>>
    where
        MM: Into<M> + Clone,
    {
        match self.group.get(group_index) {
            Some(g) => {
                g.iter().for_each(|i| self.send_to(mm.clone().into(), *i));
                Ok(())
            }
            None => Err(ChannelError::SenderGroupNotFound(mm)),
        }
    }
}

pub(crate) enum ChannelError<M> {
    RecvError,
    TrySendError(M),
    SenderOverFlow(M),
    SenderGroupNotFound(M),
}

impl<M> Debug for ChannelError<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut fmt = f.debug_struct("ChannelError");

        match self {
            ChannelError::SenderOverFlow(_) => fmt
                .field("cause", &"ChannelGroupSender")
                .field("description", &"Overflow on group sender's sender index"),
            ChannelError::SenderGroupNotFound(_) => {
                fmt.field("cause", &"ChannelGroupSender").field(
                    "description",
                    &"Overflow on group sender's group index(group not found)",
                )
            }
            ChannelError::RecvError => fmt
                .field("cause", &"ChannelReceiver")
                .field("description", &"Failed to receive message from channel"),
            ChannelError::TrySendError(_) => fmt
                .field("cause", &"ChannelSender")
                .field("description", &"Failed to send message through channel"),
        };

        fmt.finish()
    }
}

impl<M> Display for ChannelError<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({})", self)
    }
}
