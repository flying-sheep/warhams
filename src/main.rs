use std::{default::Default, fs::File, io::BufWriter};

use image_crate::codecs::png::PngDecoder;
use printpdf::*;

fn main() {
    let (mut doc, page1, layer1) = PdfDocument::new("output", Mm(247.0), Mm(210.0), "Layer 1");
    doc = doc.with_conformance(PdfConformance::Custom(CustomPdfConformance {
        requires_icc_profile: false,
        requires_xmp_metadata: false,
        ..Default::default()
    }));
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc
        .add_external_font(File::open("assets/RedeyeSerif-Bold.ttf").unwrap())
        .unwrap();
    let font2 = doc
        .add_external_font(File::open("assets/MinionPro-Regular.otf").unwrap())
        .unwrap();

    add_text(current_layer.clone(), font.clone(), font2.clone());
    add_graphics(current_layer.clone());
    add_images(current_layer.clone());

    doc.save(&mut BufWriter::new(
        File::create("test_working.pdf").unwrap(),
    ))
    .unwrap();
}

fn add_text(current_layer: PdfLayerReference, font: IndirectFontRef, font2: IndirectFontRef) {
    let text = "Lorem ipsum";
    let text2 = "unicode: стуфхfцчшщъыьэюя";

    // text, font size, x from left edge, y from bottom edge, font
    current_layer.use_text(text, 48., Mm(200.0), Mm(200.0), &font);

    // For more complex layout of text, you can use functions
    // defined on the PdfLayerReference
    // Make sure to wrap your commands
    // in a `begin_text_section()` and `end_text_section()` wrapper
    current_layer.begin_text_section();

    // setup the general fonts.
    // see the docs for these functions for details
    current_layer.set_font(&font2, 33.);
    current_layer.set_text_cursor(Mm(10.0), Mm(10.0));
    current_layer.set_line_height(33.);
    current_layer.set_word_spacing(3000.);
    current_layer.set_character_spacing(10.);
    current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);

    // write two lines (one line break)
    current_layer.write_text(text.clone(), &font2);
    current_layer.add_line_break();
    current_layer.write_text(text2.clone(), &font2);
    current_layer.add_line_break();

    // write one line, but write text2 in superscript
    current_layer.write_text(text.clone(), &font2);
    current_layer.set_line_offset(10.);
    current_layer.write_text(text2.clone(), &font2);

    current_layer.end_text_section();
}

fn add_images(current_layer: PdfLayerReference) {
    // currently, the only reliable file formats are bmp/jpeg/png
    // this is an issue of the image library, not a fault of printpdf
    let mut image_file = File::open("assets/octagon.png").unwrap();
    let mut image = Image::try_from(PngDecoder::new(&mut image_file).unwrap()).unwrap();
    // https://github.com/fschutt/printpdf/issues/119
    image.image = remove_alpha_channel_from_image_x_object(image.image);

    // translate x, translate y, rotate, scale x, scale y
    // by default, an image is optimized to 300 DPI (if scale is None)
    // rotations and translations are always in relation to the lower left corner
    image.add_to_layer(
        current_layer.clone(),
        ImageTransform {
            translate_x: Some(Mm(100.)),
            translate_y: Some(Mm(100.)),
            ..Default::default()
        },
    );

    /*
    // you can also construct images manually from your data:
    let mut image_file_2 = ImageXObject {
        width: Px(200),
        height: Px(200),
        color_space: ColorSpace::Greyscale,
        bits_per_component: ColorBits::Bit8,
        interpolate: true,
        /* put your bytes here. Make sure the total number of bytes =
           width * height * (bytes per component * number of components)
           (e.g. 2 (bytes) x 3 (colors) for RGB 16bit) */
        image_data: Vec::new(),
        image_filter: None, /* does not work yet */
        clipping_bbox: None, /* doesn't work either, untested */
    };

    let image2 = Image::from(image_file_2);
    */
}

pub fn remove_alpha_channel_from_image_x_object(image_x_object: ImageXObject) -> ImageXObject {
    if !matches!(image_x_object.color_space, ColorSpace::Rgba)
        && !matches!(image_x_object.color_space, ColorSpace::GreyscaleAlpha)
    {
        return image_x_object;
    };
    let ImageXObject {
        color_space,
        image_data,
        ..
    } = image_x_object;

    let new_image_data = image_data
        .chunks(4)
        .map(|rgba| {
            let [red, green, blue, alpha]: [u8; 4] = rgba.try_into().ok().unwrap();
            let alpha = alpha as f64 / 255.0;
            let new_red = ((1.0 - alpha) * 255.0 + alpha * red as f64) as u8;
            let new_green = ((1.0 - alpha) * 255.0 + alpha * green as f64) as u8;
            let new_blue = ((1.0 - alpha) * 255.0 + alpha * blue as f64) as u8;
            return [new_red, new_green, new_blue];
        })
        .collect::<Vec<[u8; 3]>>()
        .concat();

    let new_color_space = match color_space {
        ColorSpace::Rgba => ColorSpace::Rgb,
        ColorSpace::GreyscaleAlpha => ColorSpace::Greyscale,
        other_type => other_type,
    };

    ImageXObject {
        color_space: new_color_space,
        image_data: new_image_data,
        ..image_x_object
    }
}

fn add_graphics(current_layer: PdfLayerReference) {
    // Quadratic shape. The "false" determines if the next (following)
    // point is a bezier handle (for curves)
    // If you want holes, simply reorder the winding of the points to be
    // counterclockwise instead of clockwise.
    let points1 = vec![
        (Point::new(Mm(100.0), Mm(100.0)), false),
        (Point::new(Mm(100.0), Mm(200.0)), false),
        (Point::new(Mm(300.0), Mm(200.0)), false),
        (Point::new(Mm(300.0), Mm(100.0)), false),
    ];

    // Is the shape stroked? Is the shape closed? Is the shape filled?
    let line1 = Line {
        points: points1,
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };

    // Triangle shape
    // Note: Line is invisible by default, the previous method of
    // constructing a line is recommended!
    let mut line2 = Line::from_iter(vec![
        (Point::new(Mm(150.0), Mm(150.0)), false),
        (Point::new(Mm(150.0), Mm(250.0)), false),
        (Point::new(Mm(350.0), Mm(250.0)), false),
    ]);

    line2.set_stroke(true);
    line2.set_closed(false);
    line2.set_fill(false);
    line2.set_as_clipping_path(false);

    let fill_color = Color::Cmyk(Cmyk::new(0.0, 0.23, 0.0, 0.0, None));
    let outline_color = Color::Rgb(Rgb::new(0.75, 1.0, 0.64, None));
    let mut dash_pattern = LineDashPattern::default();
    dash_pattern.dash_1 = Some(20);

    current_layer.set_fill_color(fill_color);
    current_layer.set_outline_color(outline_color);
    current_layer.set_outline_thickness(10.0);

    // Draw first line
    current_layer.add_shape(line1);

    let fill_color_2 = Color::Cmyk(Cmyk::new(0.0, 0.0, 0.0, 0.0, None));
    let outline_color_2 = Color::Greyscale(Greyscale::new(0.45, None));

    // More advanced graphical options
    current_layer.set_overprint_stroke(true);
    current_layer.set_blend_mode(BlendMode::Seperable(SeperableBlendMode::Multiply));
    current_layer.set_line_dash_pattern(dash_pattern);
    current_layer.set_line_cap_style(LineCapStyle::Round);

    current_layer.set_fill_color(fill_color_2);
    current_layer.set_outline_color(outline_color_2);
    current_layer.set_outline_thickness(15.0);

    // draw second line
    current_layer.add_shape(line2);
}
