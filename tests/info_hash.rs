// extern crate nyaa_track;
use nyaa_tracker::read::torrent::Torrent;

#[test]
fn hash1() {
    let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\tests\hashes\".to_string();
    git_torrents.push_str("hash1");
    let mut torrent = Torrent::new_file(&git_torrents).unwrap();
    // dbg!{&torrent.info};

    assert_eq!(
        "8463057ea30edd86f3968c57ca4658090c616382",
        torrent.info_hash().unwrap()
    )
}

#[test]
fn hash2() {
    let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\tests\hashes\".to_string();
    git_torrents.push_str("hash2");
    let mut torrent = Torrent::new_file(&git_torrents).unwrap();

    assert_eq!(
        "4a7ca64ec23fa77272cc8aa06b9c1fe29e75dd54",
        torrent.info_hash().unwrap()
    )
}

#[test]
fn hash3() {
    let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\tests\hashes\".to_string();
    git_torrents.push_str("hash3");
    let mut torrent = Torrent::new_file(&git_torrents).unwrap();

    assert_eq!(
        "1e519729c2858f914a7717c62c90c76698886d42",
        torrent.info_hash().unwrap()
    )
}

#[test]
fn hash4() {
    let hash = "dc6cd1a241ab1dd62dc334d140f2b8e2ff5a1b57";
    let mut git_torrents = r"C:\Users\Brooks\github\nyaa_tracker\tests\hashes\".to_string();
    git_torrents.push_str("hash4");

    let mut torrent = Torrent::new_file(&git_torrents).unwrap();

    assert_eq!(hash, torrent.info_hash().unwrap())
}
