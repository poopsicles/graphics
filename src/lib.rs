#![allow(non_snake_case)]

pub fn dda(a: (f32, f32), b: (f32, f32)) -> Vec<(f32, f32)> {
    let Δx = b.0 - a.0;
    let Δy = b.1 - a.1;
    let m = Δy / Δx;

    let i = f32::max(f32::abs(Δx), f32::abs(Δy)).round();
    let s = i as u32;

    let mut r = Vec::with_capacity(s as usize + 1);
    r.push(a);

    if m <= 1.0 {
        (1..=s)
            .for_each(|x| r.push(((a.0 + x as f32).round(), (x as f32).mul_add(m, a.1).round())));
    } else {
        (1..=s).for_each(|x| r.push(((a.0 + x as f32 / m).round(), (a.1 + x as f32).round())));
    };

    r
}

pub fn bresenham(a: (f32, f32), b: (f32, f32)) -> Vec<(f32, f32)> {
    let Δx = b.0 - a.0;
    let Δy = b.1 - a.1;
    let s = Δx.round() as usize;

    let mut r = Vec::with_capacity(s + 1);
    let mut x = a;
    let mut p = 2.0 * Δy - Δx;

    r.push(x);

    (0..s).for_each(|_| {
        if p < 0.0 {
            p += 2.0 * Δy;
            x = (x.0 + 1.0, x.1);
        } else {
            p += (2.0 * Δy) - (2.0 * Δx);
            x = (x.0 + 1.0, x.1 + 1.0);
        }

        r.push(x);
    });

    r
}

#[cfg(test)]
mod tests {
    use crate::{bresenham, dda};

    #[test]
    fn dda_1() {
        assert_eq!(
            dda((5.0, 6.0), (8.0, 12.0)),
            [
                (5.0, 6.0),
                (6.0, 7.0),
                (6.0, 8.0),
                (7.0, 9.0),
                (7.0, 10.0),
                (8.0, 11.0),
                (8.0, 12.0)
            ]
        )
    }

    #[test]
    fn dda_2() {
        assert_eq!(
            dda((1.0, 7.0), (11.0, 17.0)),
            [
                (1.0, 7.0),
                (2.0, 8.0),
                (3.0, 9.0),
                (4.0, 10.0),
                (5.0, 11.0),
                (6.0, 12.0),
                (7.0, 13.0),
                (8.0, 14.0),
                (9.0, 15.0),
                (10.0, 16.0),
                (11.0, 17.0)
            ]
        )
    }

    #[test]
    fn bres_1() {
        assert_eq!(
            bresenham((9.0, 18.0), (14.0, 22.0)),
            [
                (9.0, 18.0),
                (10.0, 19.0),
                (11.0, 20.0),
                (12.0, 20.0),
                (13.0, 21.0),
                (14.0, 22.0)
            ]
        )
    }

    #[test]
    fn bres_2() {
        assert_eq!(
            bresenham((20.0, 10.0), (30.0, 18.0)),
            [
                (20.0, 10.0),
                (21.0, 11.0),
                (22.0, 12.0),
                (23.0, 12.0),
                (24.0, 13.0),
                (25.0, 14.0),
                (26.0, 15.0),
                (27.0, 16.0),
                (28.0, 16.0),
                (29.0, 17.0),
                (30.0, 18.0)
            ]
        )
    }
}
