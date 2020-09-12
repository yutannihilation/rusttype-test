const HEIGHT: f32 = 10.0;

struct Point {
    x: f32,
    y: f32,
    id: u32,
}

struct Builder {
    cur_path_id: u32,
    base_position: rusttype::Point<f32>,
    tolerance: f32,
    points_cur_glyph: Vec<Point>,
    points: Vec<Point>,
}

impl Builder {
    fn new(tolerance: f32) -> Self {
        Self {
            cur_path_id: 0,
            base_position: rusttype::point(0.0, 0.0),
            points_cur_glyph: vec![],
            points: vec![],
            tolerance,
        }
    }

    fn finish_cur_glyph(&mut self) {
        if self.points_cur_glyph.len() > 0 {
            let init_y = self.points_cur_glyph.first().unwrap().y;
            let mut y_range = [init_y, init_y];
            y_range = self.points_cur_glyph.iter().fold(y_range, |mut sum, p| {
                if p.y < sum[0] {
                    sum[0] = p.y;
                }
                if p.y > sum[1] {
                    sum[1] = p.y;
                }
                sum
            });

            self.points.append(
                &mut self
                    .points_cur_glyph
                    .iter()
                    .map(|p| {
                        // reverse and move to zero
                        let y_reverse = (y_range[1] - y_range[0])
                            * (1.0 - (p.y - y_range[0]) / (y_range[1] - y_range[0]));
                        Point {
                            x: p.x + self.base_position.x,
                            y: y_reverse,
                            id: p.id,
                        }
                    })
                    .collect(),
            );
            self.points_cur_glyph.clear();
        }
    }

    fn next_glyph(&mut self, position: &rusttype::Point<f32>) {
        self.finish_cur_glyph();
        self.base_position = position.clone();
    }

    fn add_point(&mut self, x: f32, y: f32) {
        self.points_cur_glyph.push(Point {
            x,
            y,
            id: self.cur_path_id,
        });
    }

    fn to_path(mut self) {
        self.finish_cur_glyph();

        self.points
            .iter()
            .map(|p| println!("{},{},{}", p.x, p.y, p.id))
            .collect()
    }
}

impl<'a> rusttype::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.add_point(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.add_point(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let cur = self.points_cur_glyph.last().unwrap();
        let segment = lyon::geom::QuadraticBezierSegment {
            from: lyon::math::point(cur.x, cur.y),
            ctrl: lyon::math::point(x1, y1),
            to: lyon::math::point(x, y),
        };
        for p in segment.flattened(self.tolerance) {
            self.add_point(p.x, p.y);
        }
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let cur = self.points_cur_glyph.last().unwrap();
        let segment = lyon::geom::CubicBezierSegment {
            from: lyon::math::point(cur.x, cur.y),
            ctrl1: lyon::math::point(x1, y1),
            ctrl2: lyon::math::point(x2, y2),
            to: lyon::math::point(x, y),
        };
        for p in segment.flattened(self.tolerance) {
            self.add_point(p.x, p.y);
        }
    }

    fn close(&mut self) {
        self.cur_path_id += 1;
    }
}

fn main() {
    // This TTF file can be downloaded from https://ipafont.ipa.go.jp/old/ipafont/download.html
    let font = rusttype::Font::try_from_bytes(include_bytes!("../ipam00303/ipam.ttf")).unwrap();

    let scale = rusttype::Scale::uniform(HEIGHT);
    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(0.0, v_metrics.ascent);

    let q_glyph = font.layout("東京.R", scale, offset);

    let mut builder = Builder::new(0.0001);
    for g in q_glyph {
        builder.next_glyph(&g.position());
        // println!("{:?}", g);
        if !g.build_outline(&mut builder) {
            println!("empty");
        }
    }

    builder.to_path()
}
