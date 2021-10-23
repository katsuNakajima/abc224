use itertools::Itertools;

#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let n = parse_line!(i32);
    let mut p = Vec::new();
    let mut ans = 0;
    for _ in 0..n {
        let (x, y) = parse_line!(i64, i64);
        let t = (x, y);
        p.push(t);
    }
    for c in (0..n).combinations(3) {
        if ((p[c[1] as usize].0 - p[c[0] as usize].0) * (p[c[2] as usize].1 - p[c[0] as usize].1)
            - (p[c[2] as usize].0 - p[c[0] as usize].0) * (p[c[1] as usize].1 - p[c[0] as usize].1))
            != 0
        {
            ans += 1;
        }
    }
    println!("{}", ans);
}
