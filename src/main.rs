use clap::Parser;
use grim_rs::commands::args::{Args, FileType};
use grim_rs::commands::filetype::set_filetype;
use grim_rs::commands::geometry::handle_geometry;
use grim_rs::commands::quality::set_quality;
use grim_rs::commands::scale::set_scale;
use grim_rs::commands::level::set_compression_level;
use grim_rs::commands::output::set_output;

fn main() {
    let args = Args::parse();

    set_scale(args.scale);

    if let Some(geometry_input) = args.geometry {
        handle_geometry(geometry_input);
    }

    let filetype = set_filetype(args.filetype);

    // TODO: Add safety checks here and error out on unexpected values
    if filetype == FileType::Jpeg {
        set_quality(args.quality);
    }

    if filetype == FileType::Png {
        set_compression_level(args.level);
    }

    if let Some(output) = args.output {
        set_output(output);
    }
}
