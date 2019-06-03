use postgres::{Connection, TlsMode};

//postgresql://postgres:pass@localhost[:port][/database][?param1=val1[[&param2=val2]...]]
pub fn start() -> Result<Connection, postgres::Error>{
    let url : &'static str = "postgresql://postgres:pass@localhost/nyaa";
    Connection::connect(url, TlsMode::None)
}