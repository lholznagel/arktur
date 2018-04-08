use carina_hooks::{as_number, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::BlockGen;

use hooks::State;
use futures_cpupool::{CpuFuture, CpuPool};
use time;

use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::Read;
use std::net::UdpSocket;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{thread, time as std_time};

pub fn block(cpu_pool: &CpuPool, state: Arc<Mutex<State>>, udp: UdpSocket) -> CpuFuture<bool, ()> {
    #[allow(unreachable_code)]
    cpu_pool.spawn_fn(move || {
        let mut block_send = false;
        loop {
            let current_time = time::now_utc();

            if current_time.tm_sec == 0 && current_time.tm_min % 2 == 0 && !block_send {
                block_send = true;

                {
                    let mut state_lock = state.lock().unwrap();

                    let blocks_saved = match read_dir(&state_lock.storage) {
                        Ok(path) => path.count(),
                        Err(_) => 0
                    };

                    // at least 3 peers are required
                    if state_lock.peers.len() >= 2 {
                        let mut payload = BlockGen::block(0, String::from("0".repeat(64)), String::from(""));

                        if blocks_saved > 0 {
                            let mut next_block = String::from("");
                            for (_, content) in &state_lock.next_block {
                                next_block.push_str(&content);
                            }
                            state_lock.next_block = HashMap::new();

                            if Path::new(&format!("{}/last", state_lock.storage)).exists() {
                                let mut file = File::open(format!("{}/last", state_lock.storage)).expect("Should be able to read the last block");
                                let mut content = String::new();

                                file.read_to_string(&mut content).expect("Should be able to read last block");

                                let result: Vec<&str> = content.split('\n').collect();
                                payload = BlockGen::block(blocks_saved as u64 - 1, result[5].to_string(), next_block);
                            }
                        }

                        for (peer, (public_key, _)) in state_lock.peers.clone() {
                            let message = Protocol::new()
                                .set_event_code(as_number(EventCodes::BlockGen))
                                .set_payload(payload.clone())
                                .build(&mut state_lock.nacl, &public_key);
                            udp.send_to(message.as_slice(), peer).unwrap();
                        }
                    } else {
                        error!("Not enough peers to create next block.");
                    }
                }
            } else {
                thread::sleep(std_time::Duration::from_secs((60 - current_time.tm_sec) as u64));
                block_send = false;
            }
        }

        let res: Result<bool, ()> = Ok(true);
        res
    })
}