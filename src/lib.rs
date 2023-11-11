#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::nursery)]
#![allow(non_snake_case)]
#![allow(clippy::many_single_char_names)]

#[must_use]
pub fn dda(a: (f32, f32), b: (f32, f32)) -> Vec<(f32, f32)> {
    let Δx = b.0 - a.0;
    let Δy = b.1 - a.1;
    let m = Δy / Δx;

    let i = f32::max(f32::abs(Δx), f32::abs(Δy)).round();
    let s = unsafe { i.to_int_unchecked::<u16>() };

    let mut r = Vec::with_capacity(s as usize + 1);
    r.push(a);

    (1..=s).for_each(|x| {
        let x = f32::from(x);

        if m <= 1.0 {
            r.push(((a.0 + x).round(), x.mul_add(m, a.1).round()));
        } else {
            r.push(((a.0 + x / m).round(), (a.1 + x).round()));
        }
    });

    r
}

#[must_use]
pub fn bresenham(a: (f32, f32), b: (f32, f32)) -> Vec<(f32, f32)> {
    let Δx = b.0 - a.0;
    let Δy = b.1 - a.1;
    let s = unsafe { Δx.to_int_unchecked::<u16>() };

    let mut r = Vec::with_capacity(s as usize + 1);
    let mut x = a;
    let mut p = 2.0f32.mul_add(Δy, -Δx);

    r.push(x);

    (0..s).for_each(|_| {
        if p < 0.0 {
            p += 2.0 * Δy;
            x = (x.0 + 1.0, x.1);
        } else {
            p += 2.0f32.mul_add(Δy, -2.0 * Δx);
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
        );
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
        );
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
        );
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
        );
    }
}
