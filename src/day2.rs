pub mod task1 {
    use std::net::Ipv4Addr;

    use axum::extract::Query;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Ipv4AddrKey {
        from: Ipv4Addr,
        key: Ipv4Addr,
    }

    pub async fn dest(address_key: Query<Ipv4AddrKey>) -> String {
        let Ipv4AddrKey { from, key } = address_key.0;

        let from = from.octets();
        let key = key.octets();

        let mut dest = [0; 4];
        for i in 0..4 {
            dest[i] = from[i].overflowing_add(key[i]).0;
        }

        Ipv4Addr::new(dest[0], dest[1], dest[2], dest[3]).to_string()
    }
}

pub mod task2 {
    use std::net::Ipv4Addr;

    use axum::extract::Query;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct KeyParams {
        from: Ipv4Addr,
        to: Ipv4Addr,
    }

    pub async fn key(param: Query<KeyParams>) -> String {
        let KeyParams { from, to } = param.0;

        let from = from.octets();
        let to = to.octets();

        let mut key = [0; 4];
        for i in 0..4 {
            key[i] = to[i].overflowing_sub(from[i]).0;
        }

        Ipv4Addr::from(key).to_string()
    }
}

pub mod task3 {
    use axum::extract::Query;
    use serde::Deserialize;
    use std::net::Ipv6Addr;

    #[derive(Deserialize)]
    pub struct DestParams {
        from: Ipv6Addr,
        key: Ipv6Addr,
    }

    pub async fn dest(params: Query<DestParams>) -> String {
        let DestParams { from, key } = params.0;
        xor(from, key).to_string()
    }

    #[derive(Deserialize)]
    pub struct KeyParams {
        from: Ipv6Addr,
        to: Ipv6Addr,
    }

    pub async fn key(params: Query<KeyParams>) -> String {
        let KeyParams { from, to } = params.0;
        xor(from, to).to_string()
    }

    fn xor(x: Ipv6Addr, y: Ipv6Addr) -> Ipv6Addr {
        let x = x.segments();
        let y = y.segments();

        let mut z = [0; 8];
        for i in 0..x.len() {
            z[i] = x[i] ^ y[i];
        }

        Ipv6Addr::from(z)
    }
}
