use lyon::tessellation::{StrokeOptions, StrokeTessellator};

struct Builder {
    builder: lyon::path::Builder,
}

impl Builder {
    fn new() -> Self {
        let builder = lyon::path::Builder::new();
        Self { builder }
    }

    fn to_path(self, tolerance: f32) {
        let mut stroke_tess = StrokeTessellator::new();
        let path = self.builder.build();
        let mut geometry: lyon::tessellation::VertexBuffers<lyon::math::Point, u16> =
            lyon::tessellation::VertexBuffers::new();
        stroke_tess
            .tessellate_path(
                &path,
                &StrokeOptions::tolerance(tolerance).with_line_width(0.0),
                &mut lyon::tessellation::BuffersBuilder::new(
                    &mut geometry,
                    |pos: lyon::math::Point, _: lyon::tessellation::StrokeAttributes| {
                        println!("{},{}", pos.x, pos.y);
                        pos
                    },
                ),
            )
            .unwrap();
    }
}

impl<'a> rusttype::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.move_to(lyon::math::point(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(lyon::math::point(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.builder
            .quadratic_bezier_to(lyon::math::point(x1, y1), lyon::math::point(x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.builder.cubic_bezier_to(
            lyon::math::point(x1, y1),
            lyon::math::point(x2, y2),
            lyon::math::point(x, y),
        );
    }

    fn close(&mut self) {
        self.builder.close();
    }
}

fn main() {
    let font =
        rusttype::Font::try_from_bytes(include_bytes!("/home/yutani/Downloads/ipam00303/ipam.ttf"))
            .unwrap();

    let height: f32 = 12.4;
    let scale = rusttype::Scale {
        x: height * 2.0,
        y: height,
    };
    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(0.0, v_metrics.ascent);

    let q_glyph = font.layout("あ", scale, offset);

    let mut builder = Builder::new();
    for g in q_glyph {
        // println!("{:?}", g);
        if !g.build_outline(&mut builder) {
            println!("empty");
        }
    }

    builder.to_path(0.0001)
}
