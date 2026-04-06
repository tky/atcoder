fn resolve(events: &[&[usize]]) -> usize {
    let mut ha = events[0][0];
    let mut hb = events[0][1];
    let mut hc = events[0][2];

    for i in 1..events.len() {
        let e = events[i];
        let next_ha = hb.max(hc) + e[0];
        let next_hb = ha.max(hc) + e[1];
        let next_hc = ha.max(hb) + e[2];

        ha = next_ha;
        hb = next_hb;
        hc = next_hc;
    }
    (ha.max(hb)).max(hc)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let events: &[&[usize]] = &[&[10, 40, 70], &[20, 50, 80], &[30, 60, 90]];

        assert_eq!(resolve(events), 210);
    }
    #[test]
    fn sample_02() {
        let events: &[&[usize]] = &[&[100, 1, 1], &[100, 1, 1]];
        assert_eq!(resolve(events), 101);
    }

    #[test]
    fn sample_03() {
        let events: &[&[usize]] = &[&[1, 2, 3]];
        assert_eq!(resolve(events), 3);
    }

    #[test]
    fn sample_04() {
        let events: &[&[usize]] = &[&[10, 20, 30], &[30, 20, 10]];
        assert_eq!(resolve(events), 60);
    }

    #[test]
    fn sample_05() {
        let events: &[&[usize]] = &[&[6, 7, 8], &[8, 8, 3], &[2, 5, 2], &[7, 8, 6]];
        assert_eq!(resolve(events), 28);
    }
}
