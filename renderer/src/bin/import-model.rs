use renderer::importer::wavefront::Wavefront;

enum Parser {
    Wavefront(Wavefront),
}

fn main() {
    let parser = Parser::Wavefront(Wavefront::new());
}
