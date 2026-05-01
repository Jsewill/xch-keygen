use bip39::Language;

pub fn fingerprint_name(fp: u32) -> String {
    let wl = Language::default().word_list();
    let mut fwords: Vec<&str> = Vec::new();
    for i in (0..3).rev() {
        fwords.push(wl[(fp >> i * 11 & 0x7ff) as usize]);
    }
    fwords.join("-")
}
