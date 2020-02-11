mod lib;

fn main() -> std::io::Result<()> {
    return lib::write("/tmp/results.json");
}
