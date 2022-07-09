#[derive(PartialEq)]
enum PacketType {
    Sum,
    Prod,
    Min,
    Max,
    Literal,
    Gt,
    Lt,
    Eq,
}

impl From<u64> for PacketType {
    fn from(num: u64) -> Self {
        match num {
            0 => Self::Sum,
            1 => Self::Prod,
            2 => Self::Min,
            3 => Self::Max,
            4 => Self::Literal,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => panic!("Should not reach."),
        }
    }
}

struct Packet {
    version: u64,
    typ: PacketType,
    body: Option<u64>,
    children: Option<Vec<Packet>>,
}

impl Packet {
    fn sum_verstion(&self) -> u64 {
        if self.body.is_some() {
            return self.version;
        }

        let mut sum_ver = 0;
        for c in self.children.as_ref().unwrap().iter() {
            sum_ver += c.sum_verstion();
        }
        sum_ver + self.version
    }

    fn eval_expression(&self) -> u64 {
        if self.typ == PacketType::Literal {
            return self.body.unwrap();
        }

        let children = self
            .children
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.eval_expression())
            .collect::<Vec<u64>>();

        match self.typ {
            PacketType::Sum => children.iter().sum(),
            PacketType::Prod => children.iter().product(),
            PacketType::Min => children.iter().min().unwrap().to_owned(),
            PacketType::Max => children.iter().max().unwrap().to_owned(),
            PacketType::Gt => (children[0] > children[1]) as u64,
            PacketType::Lt => (children[0] < children[1]) as u64,
            PacketType::Eq => (children[0] == children[1]) as u64,
            PacketType::Literal => panic!("Should not reach"),
        }
    }
}

struct BITStream {
    consumed: usize,
    bits: Vec<u64>,
}

impl From<&str> for BITStream {
    fn from(stream: &str) -> Self {
        let bits = stream
            .chars()
            .into_iter()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();
        BITStream { consumed: 0, bits }
    }
}

impl BITStream {
    fn take(&mut self, n: usize) -> Vec<u64> {
        self.consumed += n;
        Vec::from_iter(self.bits.drain(..n))
    }

    fn checksum(&self) -> u64 {
        self.bits.iter().sum()
    }
}

fn bitvec_to_int(bits: &[u64]) -> u64 {
    // FIXME This should be a macro.
    let mut acc = 0;
    for (n, i) in bits.iter().rev().enumerate() {
        acc += i * 2_u64.pow(n as u32);
    }
    acc
}

fn parse_packets(stream: &mut BITStream) -> Packet {
    let version = bitvec_to_int(&stream.take(3));
    let typ = PacketType::from(bitvec_to_int(&stream.take(3)));

    if typ == PacketType::Literal {
        let mut lit: Vec<u64> = Vec::new();
        loop {
            let group = stream.take(5);
            lit.extend(group[1..].iter());
            if group[0] == 0 {
                break;
            }
        }

        let body = bitvec_to_int(&lit);
        Packet {
            version,
            typ,
            body: Some(body),
            children: None,
        }
    } else {
        let len_type_id = bitvec_to_int(&stream.take(1));
        let mut children: Vec<Packet> = Vec::new();
        if len_type_id == 0 {
            let to_read = bitvec_to_int(&stream.take(15)) as usize;
            let before = stream.consumed;
            loop {
                let child = parse_packets(stream);
                children.push(child);
                if stream.consumed - before == to_read {
                    break;
                }
            }
        } else {
            let num_subpackets = bitvec_to_int(&stream.take(11));
            for _ in 0..num_subpackets {
                let child = parse_packets(stream);
                children.push(child);
            }
        }

        Packet {
            version,
            typ,
            body: None,
            children: Some(children),
        }
    }
}

fn main() {
    let message = include_str!("../d16_decoded.txt").trim();
    let mut stream = BITStream::from(message);
    let packet = parse_packets(&mut stream);

    assert_eq!(stream.checksum(), 0);
    println!("{}", packet.sum_verstion());
    println!("{}", packet.eval_expression());
}
