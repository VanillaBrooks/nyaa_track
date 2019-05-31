
// extern crate nyaa_track;
use nyaa_tracker::read_torrent;

#[test]fn hash1(){
    let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
    git_torrents.push_str("ver2.torrent");
    let mut torrent = read_torrent::Torrent::new_file(&git_torrents).unwrap();
    // dbg!{&torrent.info};
    assert_eq!("8463057ea30edd86f3968c57ca4658090c616382", torrent.info_hash().unwrap())
}


#[test]
fn hash2(){
    let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
    git_torrents.push_str("1147504.torrent");
    let mut torrent = read_torrent::Torrent::new_file(&git_torrents).unwrap();

    assert_eq!("4a7ca64ec23fa77272cc8aa06b9c1fe29e75dd54", torrent.info_hash().unwrap())
}

#[test]
fn hash3() {
    let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\torrents\".to_string();
    git_torrents.push_str("test.torrent");
    let mut torrent = read_torrent::Torrent::new_file(&git_torrents).unwrap();

    assert_eq!( "1e519729c2858f914a7717c62c90c76698886d42", torrent.info_hash().unwrap())
}

// ---- hash1 stdout ----
// thread 'hash1' panicked at 'assertion failed: `(left == right)`
//   left: `"ec11ee51efd2d70649aa954101664ba9ed16b899"`,
//  right: `"8463057ea30edd86f3968c57ca4658090c616382"`', tests\info_hash.rs:11:5

// ---- hash2 stdout ----
// thread 'hash2' panicked at 'assertion failed: `(left == right)`
//   left: `"f5874dc5c5ba97eedec74f9e80979a9e3e599380"`,
//  right: `"4a7ca64ec23fa77272cc8aa06b9c1fe29e75dd54"`', tests\info_hash.rs:21:5
// note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.