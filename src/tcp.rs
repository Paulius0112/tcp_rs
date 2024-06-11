

pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

impl Default for State {
    fn default() -> Self {
        State::Listen,
    }
}

impl State {
    // Lifetime of the packet itself
    pub fn on_packet<'a>(&mut self, iph: etherparse::Ipv4HeaderSlice<'a>, tcph: etherparse::TcpHeaderSlice<'a>, data: &'a [u8]) {
        match *self {
            State::Closed => {
                return;
            },
            State::Listen => {
                if !tcph.syn() {
                    // onyl expected syn packet
                    return;
                }

                // need to establish a connection
                
            }
        }
        eprintln!(
            "{}:{} -> {}:{} {}b of tcp",
            iph.source_addr(),
            tcph.source_port(),
            iph.destination_addr(),
            tcph.destination_port(),
            data.len(),
        )
    }
}