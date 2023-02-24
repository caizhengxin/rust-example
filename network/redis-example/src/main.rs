extern crate redis;
use std::thread;
use std::time::Duration;
use redis::Commands;

pub struct Redis {
    url: String,
    _conn: redis::Connection,
}


impl Redis {
    pub fn new(url: &str) -> redis::RedisResult<Self> {
        let client = redis::Client::open(url)?;
        let conn = client.get_connection()?;

        Ok(Redis{url: url.to_string(), _conn: conn})
    }

    pub fn try_conn(&mut self) -> redis::RedisResult<()> {
        let ret: redis::RedisResult<String> = redis::cmd("PING").query(&mut self._conn);

        match ret {
            Ok(_v) => {
                // PONG
                // println!("{}", _v);
            },
            Err(_e) => { 
                let client = redis::Client::open(&*self.url)?;
                self._conn = client.get_connection()?;
            }
        }

        Ok(())
    }

    #[allow(unused_must_use)]
    pub fn conn(&mut self) -> &mut redis::Connection {
        self.try_conn();

        &mut self._conn
    }
}


fn main() {
    let mut rds = Redis::new("redis://127.0.0.1").unwrap();

    loop {

        let _ret: redis::RedisResult<redis::Value> = rds.conn().rpush("jankincai", Some("jkc"));
        println!("{:?}", _ret);

        thread::sleep(Duration::from_secs(1));
    }
}
